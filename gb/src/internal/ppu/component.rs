use crate::{log, console_log};

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
    pub ly: u8,
    pub lyc: u8,
    pub control: u8,
    pub stat: u8,
    pub scy: u8,
    pub scx: u8,
    pub wy: u8,
    pub wx: u8,
    pub bgp: u8,
    pub obp0: u8,
    pub obp1: u8,
    pub new_mode: Option<Mode>,
    pub oam: [u8; 0xA0],
    vram: [u8; 0x2000],
    scanline_timeline: usize,
    vblank_timeline: usize,
    tick_state: TickState,
    window_line_counter: u8,
    window_in_frame: bool
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
    lyc_triggered: bool,
    window_in_scanline: bool,

    oam_ptr: usize,
    bg_fetcher_step: u8,
    sprite_fetcher_step: u8
}

struct Sprite {
    color_id: u8,
    flags: u8
}

impl PPU {
    fn get_mode(&self) -> Mode {
        match self.stat & 0x3 {
            0 => Mode::HBLANK,
            1 => Mode::VBLANK,
            2 => Mode::OAMSCAN,
            3 => Mode::DRAW,
            _ => panic!("Unexpected branch.")
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

    fn is_window_in_view(&mut self) -> bool { (self.control >> WINDOW_ENABLED) & 0x1 == 1 && self.tick_state.scanline_x as u8 >= self.wx - 7 && self.window_in_frame }

    fn detect_sprite(&mut self) -> Option<[u8; 4]> {
        let objects = self.tick_state.sprite_buffer.len() / 4;
        for i in 0..objects {
            let base_ptr = i * 4;
            if self.tick_state.sprite_buffer[base_ptr + 1] <= self.tick_state.scanline_x as u8 + 8 {
                let y_pos = self.tick_state.sprite_buffer.remove(base_ptr);
                let x_pos = self.tick_state.sprite_buffer.remove(base_ptr);
                let tile_number = self.tick_state.sprite_buffer.remove(base_ptr);
                let sprite_flags = self.tick_state.sprite_buffer.remove(base_ptr);
                return Some([y_pos, x_pos, tile_number, sprite_flags]);
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
            let vertical_flip = self.tick_state.current_sprite.unwrap()[3] >> 6 & 0x1 == 1;

            let mut vertical_offset: u16 = self.ly as u16;
            if vertical_flip {
                let y_pos = self.tick_state.current_sprite.unwrap()[0] as u16;
                let sprite_height = (if self.control >> SPRITE_SIZE & 0x1 == 1 { 16 } else { 8 }) as u16;
                vertical_offset = ((y_pos + sprite_height) - (self.ly as u16)) - 1;
            }

            if self.tick_state.sprite_fetcher_step < 1 {
                self.tick_state.tile_number = self.tick_state.current_sprite.unwrap()[2];
                self.tick_state.bg_fetcher_step = 0;
                self.tick_state.sprite_fetcher_step += 1;
            } else if self.tick_state.sprite_fetcher_step < 2 {
                let offset = 2 * ((vertical_offset as u16 + self.scy as u16) % 8);
                let tile: u16 = 0x8000 + (self.tick_state.tile_number as u16 * 16);
                self.tick_state.tile_data_low = self.vram[(tile + offset - 0x8000) as usize];
                self.tick_state.sprite_fetcher_step += 1;
            } else if self.tick_state.sprite_fetcher_step < 3 {
                let offset = 2 * ((vertical_offset as u16 + self.scy as u16) % 8);
                let tile: u16 = 0x8000 + (self.tick_state.tile_number as u16 * 16);
                self.tick_state.tile_data_high = self.vram[(tile + offset + 1 - 0x8000) as usize];
                self.tick_state.sprite_fetcher_step += 1;
            } else {
                let horizontal_flip = self.tick_state.current_sprite.unwrap()[3] >> 5 & 0x1 == 1;
                let base = (if self.tick_state.current_sprite.unwrap()[1] < 8 { 8 - self.tick_state.current_sprite.unwrap()[1] } else { 0 }) + (self.tick_state.sprite_fifo.len() as u8);
                for i in base..8 {
                    self.tick_state.sprite_fifo.push(Sprite{
                        color_id: (((self.tick_state.tile_data_high >> (if horizontal_flip { i } else { 7 - i })) & 0x1) << 1) | ((self.tick_state.tile_data_low >> (if horizontal_flip { i } else { 7 - i })) & 0x1), 
                        flags: self.tick_state.current_sprite.unwrap()[3]
                    });
                }
                self.tick_state.current_sprite = self.detect_sprite();
                self.tick_state.sprite_fetcher_step = 0;
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
            let tile_x;
            let tile_y;

            if self.is_window_in_view() {
                if (self.control >> WINDOW_TILE_MAP) & 0x1 == 0 {
                    tile_map = 0x9C00;
                }
                // TODO!
                tile_x = (self.tick_state.fetcher_x as u16) + (self.wx as u16);
                tile_y = 32 * ((self.window_line_counter as u16) / 8);
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
            let offset = if self.is_window_in_view() { 2 * ((self.window_line_counter as u16) % 8) } else { 2 * ((self.ly as u16 + self.scy as u16) % 8) };
            let tile;

            if (self.control >> TILE_ADDRESSING) & 0x1 == 1 {
                tile = 0x8000 + (self.tick_state.tile_number as u16 * 16)
            } else {
                tile = (0x9000 as u16).wrapping_add_signed((self.tick_state.tile_number as i8 as i16) * 16);
            }

            self.tick_state.tile_data_low = self.vram[((tile + offset) - 0x8000) as usize];
            self.tick_state.bg_fetcher_step += 1;
        } else if self.tick_state.bg_fetcher_step < 3 {
            let offset = if self.is_window_in_view() { 2 * ((self.window_line_counter as u16) % 8) } else { 2 * ((self.ly as u16 + self.scy as u16) % 8) };
            let tile;

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

    fn get_object_color(&self, pallete: u8, color_id: u8) -> u8 {
        if pallete == 1 {
            return (self.obp1 >> (2 * color_id)) & 0x3
        } else if pallete == 0 {
            return (self.obp0 >> (2 * color_id)) & 0x3
        }
        panic!("invalid pallete number!");
    }

    fn tick(&mut self) { // 2 dots
        self.scanline_timeline += 2;

        if self.ly == self.lyc && !self.tick_state.lyc_triggered {
            self.tick_state.lyc_triggered = true;
            self.stat |= 0b00000100;
        }

        if self.wy == self.ly {
            self.window_in_frame = true;
        }

        match self.get_mode() {
            Mode::OAMSCAN => {
                if self.tick_state.sprite_buffer.len() < 40 {
                    let base_ptr = 4 * self.tick_state.oam_ptr;

                    let y_pos = self.oam[base_ptr];
                    let x_pos = self.oam[base_ptr + 1];
                    let mut tile_number = self.oam[base_ptr + 2];
                    let sprite_flags = self.oam[base_ptr + 3];

                    let mut sprite_height: u8 = 8;
                    if (self.control >> SPRITE_SIZE) & 0x1 == 1 {
                        tile_number &= 0b11111110; // bit 0 of tile index for 8x16 objects should be ignored
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
                if (self.control >> SPRITES_ENABLED) & 0x1 == 1 { self.sprite_pixel_fetcher() }

                if self.tick_state.current_sprite.is_none() { // don't continue to next tile if currently fetching sprite
                    self.background_pixel_fetcher()
                }

                if self.tick_state.current_sprite.is_none() { // temporarily pause lcd pushing while sprites are being fetched
                    for _ in 0..2 { // draws 1 pixel per dot
                        if self.tick_state.background_fifo.len() > 0 {
                            if self.tick_state.scanline_x == 0 { // at the start of each scanline discard SCX mod 8 pixels from FIFO and push the rest to LCD
                                for _ in 0..(self.scx % 8) {
                                    self.tick_state.background_fifo.remove(0);
                                }
                            }

                            let bg_color_id = self.tick_state.background_fifo.remove(0);
                            let bg_color_value = (self.bgp >> (bg_color_id * 2)) & 0x3;

                            if self.tick_state.sprite_fifo.len() > 0 {
                                let sprite = self.tick_state.sprite_fifo.remove(0);
                                let sprite_color_value = self.get_object_color((sprite.flags >> 4) & 0x1, sprite.color_id);    

                                if sprite_color_value == 0 { // sprite is transparent so background is visible
                                    self.lcd[(self.ly as usize * 160) + self.tick_state.scanline_x] = bg_color_value;
                                } else if (sprite.flags >> 7) & 0x1 == 1 && bg_color_value != 0 { // background has priority and isn't transparent
                                    self.lcd[(self.ly as usize * 160) + self.tick_state.scanline_x] = bg_color_value;
                                } else { // otherwise just default to showing the sprite
                                    self.lcd[(self.ly as usize * 160) + self.tick_state.scanline_x] = sprite_color_value;
                                }
                            } else {
                                self.lcd[(self.ly as usize * 160) + self.tick_state.scanline_x] = bg_color_value;
                            }

                            self.tick_state.scanline_x += 1;

                            if !self.tick_state.window_in_scanline && self.is_window_in_view() {
                                self.tick_state.window_in_scanline = true;
    
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
                if self.tick_state.window_in_scanline {
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
                    self.window_in_frame = false;
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
            bgp: 0x0,
            obp0: 0,
            obp1: 0,
            tick_state: TickState::default(),
            scanline_timeline: 0,
            vblank_timeline: 0,
            window_line_counter: 0,
            new_mode: None,
            window_in_frame: false
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
            current_sprite: None,
            lyc_triggered: false,
            window_in_scanline: false
        }
    }
}