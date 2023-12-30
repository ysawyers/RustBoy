use crate::{log, console_log};

const LCD_ENABLED: u8 = 7;
const WINDOW_TILE_MAP: u8 = 6;
const WINDOW_ENABLED: u8 = 5;
const TILE_ADDRESSING: u8 = 4;
const BG_TILE_MAP: u8 = 3;
const SPRITE_SIZE: u8 = 2;
const SPRITES_ENABLED: u8 = 1;
const BG_OR_WINDOW_ENABLED: u8 = 0;

enum Mode {
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
    vram: [u8; 0x2000],
    oam: [u8; 0xA0],
    scanline_timeline: usize,
    vblank_timeline: usize,
    tick_state: TickState,
}

struct TickState {
    fetcher_x: usize,
    scanline_x: usize,
    sprite_fifo: Vec<u8>,
    background_fifo: Vec<u8>,
    sprite_buffer: Vec<u8>,
    tile_number: u8,
    tile_data_low: u8,
    tile_data_high: u8,
    new_scanline: bool,
    fetching_sprite: bool,

    oam_ptr: usize,
    fetcher_step: u8,
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
    }

    pub fn read_vram(&self, addr: u16) -> u8 {
        self.vram[addr as usize]
    }
    
    pub fn write_vram(&mut self, addr: u16, val: u8) {
        self.vram[addr as usize] = val;
    }

    pub fn read_oam(&self, addr: u16) -> u8 {
        self.oam[addr as usize]
    }

    pub fn write_oam(&mut self, addr: u16, val: u8) {
        self.oam[addr as usize] = val;
    }

    fn is_window_in_view(&self) -> bool {
        true
    }

    pub fn sprite_pixel_fetcher(&mut self) {
        
    }

    pub fn background_pixel_fetcher(&mut self) {
        if self.tick_state.fetcher_step < 1 {
            let mut tile_map: u16 = 0x9800;
            let mut tile_x: u16 = 0;
            let mut tile_y: u16 = 0;

            if self.is_window_in_view() && (self.control >> WINDOW_ENABLED) & 0x1 == 1 {
                console_log!("WINDOW IN VIEW");
                panic!("IMPLEMEMENT WINDOW RENDERING STEP 1");
            } else {
                if (self.control >> BG_TILE_MAP) & 0x1 == 1 {
                    tile_map = 0x9C00;
                }
                tile_x = (self.tick_state.fetcher_x as u16 + ((self.scx as u16) / 8)) & 0x1F;
                tile_y = 32 * ((((self.ly as u16) + (self.scy as u16)) & 0xFF) / 8);
            }

            self.tick_state.tile_number = self.vram[((tile_map + ((tile_x + tile_y) & 0x3FF)) - 0x8000) as usize];
            self.tick_state.fetcher_step += 1;
        } else if self.tick_state.fetcher_step < 2 {
            let offset = 2 * ((self.ly as u16 + self.scy as u16) % 8);
            let mut tile: u16 = 0;

            if (self.control >> TILE_ADDRESSING) & 0x1 == 1 {
                tile = 0x8000 + (self.tick_state.tile_number as u16 * 16)
            } else {
                tile = (0x9000 as u16).wrapping_add_signed((self.tick_state.tile_number as i8 as i16) * 16);
            }

            self.tick_state.tile_data_low = self.vram[((tile + offset) - 0x8000) as usize];
            self.tick_state.fetcher_step += 1;
        } else if self.tick_state.fetcher_step < 3 {
            let offset = 2 * ((self.ly as u16 + self.scy as u16) % 8);
            let mut tile: u16 = 0;

            if (self.control >> TILE_ADDRESSING) & 0x1 == 1 {
                tile = 0x8000 + (self.tick_state.tile_number as u16 * 16)
            } else {
                tile = (0x9000 as u16).wrapping_add_signed((self.tick_state.tile_number as i8 as i16) * 16);
            }

            self.tick_state.tile_data_high = self.vram[((tile + offset + 1) - 0x8000) as usize];
            self.tick_state.fetcher_step += 1;
        } else if self.tick_state.new_scanline {
            self.tick_state.new_scanline = false;
            self.tick_state.fetcher_step = 0;
        } else {
            if self.tick_state.background_fifo.len() <= 8 {
                for i in 0..8 {
                    self.tick_state.background_fifo.push((((self.tick_state.tile_data_high >> (7 - i)) & 0x1) << 1) | ((self.tick_state.tile_data_low >> (7 - i)) & 0x1));
                }
                self.tick_state.fetcher_step = 0;
            }
            self.tick_state.fetcher_x += 1;
        }
    }

    fn tick(&mut self) { // 2 dots
        self.scanline_timeline += 2;

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
                if (self.control >> BG_OR_WINDOW_ENABLED) & 0x1 == 0 { self.background_pixel_fetcher() }
                if (self.control >> SPRITES_ENABLED) & 0x1 == 1 { self.sprite_pixel_fetcher() }

                if !self.tick_state.fetching_sprite {
                    for _ in 0..2 { // draws 1 pixel per dot
                        if self.tick_state.background_fifo.len() > 0 {
                            self.lcd[(self.ly as usize * 160) + self.tick_state.scanline_x] = self.tick_state.background_fifo.remove(0);
                            self.tick_state.scanline_x += 1;
                        }
    
                        if self.tick_state.scanline_x > 159 { // end of scanline
                            self.update_mode(Mode::HBLANK);
                            break
                        }
                    }
                }
            },
            Mode::HBLANK => {
                self.tick_state = TickState::default();
                if self.scanline_timeline == 456 { // 456 dots per scanline
                    self.ly += 1;
                    if self.ly > 143 {
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
            vblank_timeline: 0
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
            fetcher_step: 0,
            tile_data_high: 0,
            tile_data_low: 0,
            tile_number: 0,
            background_fifo: vec![],
            sprite_fifo: vec![],
            new_scanline: true,
            fetching_sprite: false
        }
    }
}