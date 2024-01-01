// use crate::{log, console_log};

const LCD_ENABLED: u8 = 7;
const WINDOW_TILE_MAP: u8 = 6;
const WINDOW_ENABLED: u8 = 5;
const TILE_ADDRESSING: u8 = 4;
const BG_TILE_MAP: u8 = 3;
const SPRITE_SIZE: u8 = 2;
const SPRITES_ENABLED: u8 = 1;
const BG_OR_WINDOW_ENABLED: u8 = 0;

#[derive(PartialEq)]
pub enum Mode {
    OAMSCAN, DRAW, HBLANK, VBLANK
}

pub type Display = [u8; 23040];

pub struct PPU {
    pub lcd: Display,
    pub ly: u8, // read only
    pub lyc: u8, // read/write
    pub control: u8, // read/write
    pub stat: u8, // read/write
    pub scy: u8,
    pub scx: u8,
    pub wy: u8,
    pub wx: u8,
    pub new_mode: Option<Mode>,
    vram: [u8; 0x2000],
    oam: [u8; 0xA0],
    scanline_timeline: usize,
    vblank_timeline: usize,
    tick_state: TickState,
    window_line_counter: u8,
}

struct TickState {
    fetcher_x: usize,
    scanline_x: usize,
    sprite_fifo: Vec<Sprite>,
    background_fifo: Vec<u8>,
    sprite_buffer: Vec<u8>,
    tile_number: u8,
    tile_data_low: u8,
    tile_data_high: u8,
    current_sprite: Option<[u8; 4]>,
    new_scanline: bool,
    fetching_sprite: bool,
    window_on_scanline: bool,

    oam_ptr: usize,
    bg_fetcher_step: u8,
    sprite_fetcher_step: u8
}

struct Sprite {
    value: u8,
    flags: u8
}

impl PPU {
    fn get_mode(&self) -> Mode {
        match self.stat & 0x3 {
            0 => Mode::HBLANK,
            1 => Mode::VBLANK,
            2 => Mode::OAMSCAN,
            3 => Mode::DRAW,
            _ => {
                // console_log!("BRUH");
                panic!("Unexpected branch.")
            }
        }
    }

    fn update_mode(&mut self, mode: Mode) {
        self.stat &= 0b11111100;
        match mode {
            Mode::OAMSCAN => self.stat |= 0b00000010,
            Mode::DRAW => self.stat |= 0b00000011,
            Mode::HBLANK => self.stat |= 0b00000000,
            Mode::VBLANK => self.stat |= 0b00000001,
        };
        self.new_mode.get_or_insert(mode);
    }

    fn is_window_in_view(&self) -> bool { (self.control >> WINDOW_ENABLED) & 0x1 == 1 && self.tick_state.scanline_x as u8 >= self.wx - 7 && self.tick_state.window_on_scanline }

    fn detect_sprite(&mut self) -> Option<[u8; 4]> {
        let objects = self.tick_state.sprite_buffer.len() / 4;
        for i in 0..objects {
            let base_ptr = i * 4;
            if self.tick_state.sprite_buffer[base_ptr + 1] <= self.tick_state.scanline_x as u8 + 8 {
                return Some([self.tick_state.sprite_buffer.remove(base_ptr), self.tick_state.sprite_buffer.remove(base_ptr), self.tick_state.sprite_buffer.remove(base_ptr), self.tick_state.sprite_buffer.remove(base_ptr)]);
            }
        }
        return None
    }

    pub fn read_vram(&self, addr: u16) -> u8 {
        if self.get_mode() != Mode::DRAW {
            return self.vram[addr as usize];
        }
        return 0xFF;
    }
    
    pub fn write_vram(&mut self, addr: u16, val: u8) {
        if self.get_mode() != Mode::DRAW {
            self.vram[addr as usize] = val;
        }
    }

    pub fn read_oam(&self, addr: u16) -> u8 {
        if self.get_mode() != Mode::DRAW && self.get_mode() != Mode::OAMSCAN {
            return self.oam[addr as usize];
        }
        return 0xFF;
    }

    pub fn write_oam(&mut self, addr: u16, val: u8) {
        if self.get_mode() != Mode::DRAW && self.get_mode() != Mode::OAMSCAN {
            self.oam[addr as usize] = val;
        }
    }

    pub fn sprite_pixel_fetcher(&mut self) {
        if self.tick_state.current_sprite.is_none() { self.tick_state.current_sprite = self.detect_sprite() }

        if !self.tick_state.current_sprite.is_none() {
            if self.tick_state.sprite_fetcher_step < 1 {
                self.tick_state.fetching_sprite = true;
                self.tick_state.sprite_fetcher_step += 1;
            } else if self.tick_state.sprite_fetcher_step < 2 {
                let offset = 2 * ((self.ly as u16 + self.scy as u16) % 8);
                let tile: u16 = 0x8000 + (self.tick_state.current_sprite.unwrap()[2] as u16 * 16);
                self.tick_state.tile_data_low = self.vram[((tile + offset) - 0x8000) as usize];
                self.tick_state.sprite_fetcher_step += 1;
            } else if self.tick_state.sprite_fetcher_step < 3 {
                let offset = 2 * ((self.ly as u16 + self.scy as u16) % 8);
                let tile: u16 = 0x8000 + (self.tick_state.current_sprite.unwrap()[2] as u16 * 16);
                self.tick_state.tile_data_high = self.vram[((tile + offset + 1) - 0x8000) as usize];
                self.tick_state.sprite_fetcher_step += 1;
            } else {
                let base = (if self.tick_state.current_sprite.unwrap()[1] < 8 { 8 - self.tick_state.current_sprite.unwrap()[1] } else { 0 }) + (self.tick_state.sprite_fifo.len() as u8);
                for i in base..8 {
                    self.tick_state.sprite_fifo.push(Sprite{
                        value: (((self.tick_state.tile_data_high >> (7 - i)) & 0x1) << 1) | ((self.tick_state.tile_data_low >> (7 - i)) & 0x1),
                        flags: self.tick_state.current_sprite.unwrap()[3]
                    });
                }

                // check if another sprite can be fetched, and if not re-enable pixel transfer
                self.tick_state.current_sprite = self.detect_sprite();
                if self.tick_state.current_sprite.is_none() {
                    self.tick_state.fetching_sprite = false;
                }
            }
        }
    }

    pub fn background_pixel_fetcher(&mut self) {
        if (self.control >> BG_OR_WINDOW_ENABLED) & 0x1 == 0 { // clear background with white pixels, sprites unaffected.
            if self.tick_state.background_fifo.len() <= 8 {
                for _ in 0..8 {
                    self.tick_state.background_fifo.push(0);
                }
            }
            return
        }

        if self.tick_state.bg_fetcher_step < 1 {
            let mut tile_map: u16 = 0x9800;
            let mut tile_x: u16 = 0;
            let mut tile_y: u16 = 0;

            if self.is_window_in_view() && (self.control >> WINDOW_ENABLED) & 0x1 == 1 {
                if (self.control >> WINDOW_TILE_MAP) & 0x1 == 1 {
                    tile_map = 0x9C00;
                }
                tile_x = (self.tick_state.fetcher_x as u16 + ((self.wx as u16) / 8)) & 0x1F;
                tile_y = 32 * ((self.window_line_counter as u16) / 8)
            } else {
                if (self.control >> BG_TILE_MAP) & 0x1 == 1 {
                    tile_map = 0x9C00;
                }
                tile_x = (self.tick_state.fetcher_x as u16 + ((self.scx as u16) / 8)) & 0x1F;
                tile_y = 32 * ((((self.ly as u16) + (self.scy as u16)) & 0xFF) / 8);
            }

            self.tick_state.tile_number = self.vram[((tile_map + ((tile_x + tile_y) & 0x3FF)) - 0x8000) as usize];
            self.tick_state.bg_fetcher_step += 1;
        } else if self.tick_state.bg_fetcher_step < 2 {
            let mut offset = 2 * ((self.ly as u16 + self.scy as u16) % 8);
            let mut tile: u16 = 0;

            if self.is_window_in_view() && (self.control >> WINDOW_ENABLED) & 0x1 == 1 {
                offset = 2 * ((self.window_line_counter as u16) % 8)
            }

            if (self.control >> TILE_ADDRESSING) & 0x1 == 1 {
                tile = 0x8000 + (self.tick_state.tile_number as u16 * 16)
            } else {
                tile = (0x9000 as u16).wrapping_add_signed((self.tick_state.tile_number as i8 as i16) * 16);
            }

            self.tick_state.tile_data_low = self.vram[((tile + offset) - 0x8000) as usize];
            self.tick_state.bg_fetcher_step += 1;
        } else if self.tick_state.bg_fetcher_step < 3 {
            let mut offset = 2 * ((self.ly as u16 + self.scy as u16) % 8);
            let mut tile: u16 = 0;

            if self.is_window_in_view() && (self.control >> WINDOW_ENABLED) & 0x1 == 1 {
                offset = 2 * ((self.window_line_counter as u16) % 8)
            }

            if (self.control >> TILE_ADDRESSING) & 0x1 == 1 {
                tile = 0x8000 + (self.tick_state.tile_number as u16 * 16)
            } else {
                tile = (0x9000 as u16).wrapping_add_signed((self.tick_state.tile_number as i8 as i16) * 16);
            }

            self.tick_state.tile_data_high = self.vram[((tile + offset + 1) - 0x8000) as usize];
            self.tick_state.bg_fetcher_step += 1;
        } else if self.tick_state.new_scanline {
            self.tick_state.new_scanline = false;
            self.tick_state.bg_fetcher_step = 0;
        } else {
            if self.tick_state.background_fifo.len() <= 8 {
                for i in 0..8 {
                    self.tick_state.background_fifo.push((((self.tick_state.tile_data_high >> (7 - i)) & 0x1) << 1) | ((self.tick_state.tile_data_low >> (7 - i)) & 0x1));
                }
                self.tick_state.bg_fetcher_step = 0;
            }
            self.tick_state.fetcher_x += 1;
        }
    }

    fn tick(&mut self) { // 2 dots
        self.scanline_timeline += 2;

        if self.wy == self.ly {
            self.tick_state.window_on_scanline = true;
        }

        match self.get_mode() {
            Mode::OAMSCAN => {
                if self.tick_state.sprite_buffer.len() < 40 {
                    let base_ptr = 4 * self.tick_state.oam_ptr;

                    let y_pos = self.oam[base_ptr];
                    let x_pos = self.oam[base_ptr + 1];
                    let tile_number = self.oam[base_ptr + 2];
                    let sprite_flags = self.oam[base_ptr + 3];

                    let mut sprite_height: u8 = 8;
                    if (self.control >> SPRITE_SIZE) & 0x1 == 1 {
                        sprite_height = 16;
                    }

                    if x_pos > 0 && self.ly + 16 >= y_pos && self.ly + 16 < y_pos + sprite_height {
                        self.tick_state.sprite_buffer.push(y_pos);
                        self.tick_state.sprite_buffer.push(x_pos);
                        self.tick_state.sprite_buffer.push(tile_number);
                        self.tick_state.sprite_buffer.push(sprite_flags);
                    }
                }

                if self.tick_state.oam_ptr < 39 {
                    self.tick_state.oam_ptr += 1;
                } else {
                    self.update_mode(Mode::DRAW);
                    self.tick_state.oam_ptr = 0;
                }
            },
            Mode::DRAW => {
                if !self.tick_state.fetching_sprite || !self.tick_state.bg_fetcher_step == 0 { // don't continue to next tile if currently fetching sprite
                    self.background_pixel_fetcher()
                }
                if (self.control >> SPRITES_ENABLED) & 0x1 == 1 { self.sprite_pixel_fetcher() }

                if !self.tick_state.fetching_sprite { // temporarily pause lcd pushing while sprites are being fetched
                    for _ in 0..2 { // draws 1 pixel per dot
                        if self.tick_state.scanline_x == 0 { // at the start of each scanline discard SCX mod 8 pixels from FIFO and push the rest to LCD
                            for _ in 0..(self.scx % 8) {
                                if self.tick_state.background_fifo.len() > 0 {
                                    self.tick_state.background_fifo.remove(0);
                                }
                            }
                        }

                        if self.tick_state.background_fifo.len() > 0 {
                            if self.tick_state.sprite_fifo.len() > 0 {
                                let sprite = self.tick_state.sprite_fifo.remove(0);
                                let bg = self.tick_state.background_fifo.remove(0);

                                if sprite.value == 0 {
                                    self.lcd[(self.ly as usize * 160) + self.tick_state.scanline_x] = bg;
                                } else if (sprite.flags >> 7) & 0x1 == 1 {
                                    self.lcd[(self.ly as usize * 160) + self.tick_state.scanline_x] = bg;
                                } else {
                                    self.lcd[(self.ly as usize * 160) + self.tick_state.scanline_x] = sprite.value;
                                }
                            } else {
                                self.lcd[(self.ly as usize * 160) + self.tick_state.scanline_x] = self.tick_state.background_fifo.remove(0);
                            }
                            self.tick_state.scanline_x += 1;
                        
                            if self.is_window_in_view() {
                                self.tick_state.bg_fetcher_step = 0;
                                self.tick_state.fetcher_x = 0;
                                self.tick_state.background_fifo = vec![];
                                break
                            }
                        }

                        if self.tick_state.scanline_x > 159 { // end of scanline
                            self.update_mode(Mode::HBLANK);
                            break
                        }
                    }
                }
            },
            Mode::HBLANK => {
                if self.tick_state.window_on_scanline {
                    self.window_line_counter += 1;
                }
                self.tick_state = TickState::default();

                if self.scanline_timeline == 456 { // 456 dots per scanline
                    self.ly += 1;
                    if self.ly > 143 {
                        self.window_line_counter = 0;
                        self.update_mode(Mode::VBLANK);
                    } else {
                        self.update_mode(Mode::OAMSCAN);
                    }
                }
            },
            Mode::VBLANK => {
                self.vblank_timeline += 2;
                if self.vblank_timeline == 4560 { // 4560 dots per vblank
                    self.vblank_timeline = 0;
                    self.ly = 0;
                    self.update_mode(Mode::OAMSCAN)
                } else if self.vblank_timeline % 456 == 0 {
                    self.ly += 1;
                }
            }
        }

        if self.scanline_timeline == 456 {
            self.scanline_timeline = 0;
        }
    }

    pub fn update(&mut self) {
        if (self.control >> LCD_ENABLED) & 0x1 == 1 {
            self.tick();
            self.tick();
        }
    }
}

impl Default for PPU {
    fn default() -> Self {
        Self {
            lcd: [0; 23040],
            vram: [0x0; 0x2000],
            oam: [0x0; 0xA0],
            ly: 0,
            lyc: 0,
            control: 0x0,
            stat: 0x0,
            scy: 0,
            scx: 0,
            wy: 0,
            wx: 0,
            tick_state: TickState::default(),
            scanline_timeline: 0,
            vblank_timeline: 0,
            window_line_counter: 0,
            new_mode: None
        }
    }
}

impl Default for TickState {
    fn default() -> Self {
        Self {
            oam_ptr: 0,
            sprite_buffer: vec![],
            fetcher_x: 0,
            scanline_x: 0,
            bg_fetcher_step: 0,
            sprite_fetcher_step: 0,
            tile_data_high: 0,
            tile_data_low: 0,
            tile_number: 0,
            background_fifo: vec![],
            sprite_fifo: vec![],
            new_scanline: true,
            fetching_sprite: false,
            current_sprite: None,
            window_on_scanline: false
        }
    }
}