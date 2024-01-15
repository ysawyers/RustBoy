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
    pub oam: [u8; 0xA0],
    pub vram: [u8; 0x2000],
    pub vblank_irq_triggered: bool,
    pub stat_irq_triggered: bool,
    pub rendered_frame: bool,
    scy: u8,
    scx: u8,
    wy: u8,
    wx: u8,
    bgp: u8,
    obp0: u8,
    obp1: u8,
    scanline_timeline: usize,
    vblank_timeline: usize,
    window_in_frame: bool,
    window_line_counter: usize,
    rendered_window_on_scanline: bool,
    tick_state: TickState,

    sprite_fifo: Vec<ObjectPixel>,
    background_fifo: Vec<u8>,
    sprite_buffer: Vec<Object>,
}

struct TickState {
    is_fetching_window: bool,
    fetcher_x: usize,
    scanline_x: usize,
    tile_number: u8,
    tile_data_low: u8,
    tile_data_high: u8,
    current_sprite: Option<Object>,
    new_scanline: bool,

    oam_ptr: usize,
    bg_fetcher_step: u8,
    sprite_fetcher_step: u8
}

#[derive(Clone, Copy)]
struct Object {
    y_pos: u8,
    x_pos: u8,
    tile_number: u8,
    sprite_flags: u8
}

struct ObjectPixel {
    color_id: u8,
    flags: u8,
    x_pos: u8
}

impl PPU {
    pub fn read_registers(&self, addr: u16) -> u8 {
        match addr {
            0xFF40 => self.control,
            0xFF41 => self.stat,
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            0xFF46 => 0x00,
            0xFF47 => self.bgp,
            0xFF48 => self.obp0,
            0xFF49 => self.obp1,
            0xFF4A => self.wy,
            0xFF4B => self.wx,
            _ => unreachable!()
        }
    }

    pub fn write_registers(&mut self, addr: u16, val: u8) {
        match addr {
            0xFF40 => {
                self.control = val;
                if self.control >> 7 & 0x1 == 0 { // if LCD is switched off
                    self.stat &= 0b11111100; // reset stat mode to 0
                    self.ly = 0;
                }
                return
            },
            0xFF41 => self.stat = val,
            0xFF42 => self.scy = val,
            0xFF43 => self.scx = val,
            0xFF44 => (), // writes to LY are ignored.
            0xFF45 => self.lyc = val,
            0xFF47 => self.bgp = val,
            0xFF48 => self.obp0 = val,
            0xFF49 => self.obp1 = val, 
            0xFF4A => self.wy = val,
            0xFF4B => self.wx = val,

            _ => panic!("recieved invalid address")
        };
    }

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

    fn detect_sprite(&mut self) -> Option<Object> {
        // sort sprites in the order they appear in on the scanline (x pos)
        for i in 1..self.sprite_buffer.len() {
            while self.sprite_buffer[i - 1].x_pos > self.sprite_buffer[i].x_pos {
                let temp = self.sprite_buffer[i - 1];
                self.sprite_buffer[i - 1] = self.sprite_buffer[i];
                self.sprite_buffer[i] = temp;
            }
        }

        // grab the first valid element and pop it from the buffer (can i just always check from the front now that im sorting?)
        for i in 0..self.sprite_buffer.len() {
            if self.sprite_buffer[i].x_pos <= self.tick_state.scanline_x as u8 + 8 {
                return Some(self.sprite_buffer.remove(i));
            }
        }
        return None
    }

    pub fn read_vram(&self, addr: u16) -> u8 {
        return self.vram[addr as usize];
    }

    pub fn write_vram(&mut self, addr: u16, val: u8) {
        self.vram[addr as usize] = val;
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
            let sprite = self.tick_state.current_sprite.as_ref().unwrap();
            let sprite_height = (if self.control >> SPRITE_SIZE & 0x1 == 1 { 16 } else { 8 }) as u16;

            let vertical_flip = (sprite.sprite_flags >> 6) & 0x1 == 1;
            
            // shoutout to nemo for helping me with this math lol
            let mut vertical_offset = ((self.ly as u16).wrapping_sub((sprite.y_pos as u16).wrapping_sub(16)) % sprite_height).wrapping_mul(2) as u16;
            if vertical_flip {
                vertical_offset = ((sprite_height - 1) * 2) - vertical_offset;
            }

            if self.tick_state.sprite_fetcher_step < 1 {
                self.tick_state.tile_number = sprite.tile_number;
                self.tick_state.bg_fetcher_step = 0;
                self.tick_state.sprite_fetcher_step += 1;
            } else if self.tick_state.sprite_fetcher_step < 2 {
                let tile: u16 = self.tick_state.tile_number as u16 * 16;
                self.tick_state.tile_data_low = self.vram[(tile + vertical_offset) as usize];
                self.tick_state.sprite_fetcher_step += 1;
            } else if self.tick_state.sprite_fetcher_step < 3 {
                let tile: u16 = self.tick_state.tile_number as u16 * 16;
                self.tick_state.tile_data_high = self.vram[(tile + vertical_offset + 1) as usize];
                self.tick_state.sprite_fetcher_step += 1;
            } else {
                let horizontal_flip = sprite.sprite_flags >> 5 & 0x1 == 1;
                let base = if sprite.x_pos < 8 { 8 - sprite.x_pos } else { 0 };

                for i in (base as usize)..8 {
                    let pos = if horizontal_flip { i } else { 7 - i };
                    let pixel = ObjectPixel {
                        color_id: (((self.tick_state.tile_data_high >> pos) & 0x1) << 1) | ((self.tick_state.tile_data_low >> pos) & 0x1), 
                        flags: sprite.sprite_flags,
                        x_pos: sprite.x_pos
                    };

                    // mix overlapping pixels
                    if i + 1 <= self.sprite_fifo.len() {
                        if self.sprite_fifo[i].color_id == 0 && pixel.color_id != 0 {
                            self.sprite_fifo.remove(i);
                            self.sprite_fifo.insert(i, pixel);
                        }
                    } else {
                        self.sprite_fifo.push(pixel);
                    }
                }

                self.tick_state.current_sprite = self.detect_sprite();
                self.tick_state.sprite_fetcher_step = 0;
            }
        }
    }

    pub fn background_pixel_fetcher(&mut self) {
        if (self.control >> BG_OR_WINDOW_ENABLED) & 0x1 == 0 { // clear background with white pixels, sprites unaffected.
            if self.background_fifo.len() <= 8 {
                for _ in 0..8 {
                    self.background_fifo.push(0);
                }
            }
            return
        }

        if self.tick_state.bg_fetcher_step < 1 {
            let mut tile_map: u16 = 0x9800;
            let tile_x;
            let tile_y;

            if self.tick_state.is_fetching_window {
                self.rendered_window_on_scanline = true;
                if (self.control >> WINDOW_TILE_MAP) & 0x1 == 1 {
                    tile_map = 0x9C00;
                }
                tile_x = self.tick_state.fetcher_x as u16 & 0x1F;
                tile_y = (32 * (self.window_line_counter / 8)) as u16;
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
            let offset = if self.tick_state.is_fetching_window { (2 * (self.window_line_counter % 8)) as u16 } else { 2 * ((self.ly as u16 + self.scy as u16) % 8) };
            let tile;

            if (self.control >> TILE_ADDRESSING) & 0x1 == 1 {
                tile = 0x8000 + (self.tick_state.tile_number as u16 * 16)
            } else {
                tile = (0x9000 as u16).wrapping_add_signed((self.tick_state.tile_number as i8 as i16) * 16);
            }

            self.tick_state.tile_data_low = self.vram[((tile + offset) - 0x8000) as usize];
            self.tick_state.bg_fetcher_step += 1;
        } else if self.tick_state.bg_fetcher_step < 3 {
            let offset = if self.tick_state.is_fetching_window { (2 * (self.window_line_counter % 8)) as u16 } else { 2 * ((self.ly as u16 + self.scy as u16) % 8) };
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
            if self.background_fifo.len() <= 8 {
                for i in 0..8 {
                    self.background_fifo.push((((self.tick_state.tile_data_high >> (7 - i)) & 0x1) << 1) | ((self.tick_state.tile_data_low >> (7 - i)) & 0x1));
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

        if self.wy == self.ly {
            self.window_in_frame = true;
        }

        match self.get_mode() {
            Mode::OAMSCAN => {
                if self.sprite_buffer.len() < 10 {
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
                        self.sprite_buffer.push(Object {
                            y_pos,
                            x_pos,
                            tile_number,
                            sprite_flags
                        })
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

                let sprite_fetching = !self.tick_state.current_sprite.is_none();

                if !sprite_fetching { self.background_pixel_fetcher() }

                if !sprite_fetching {
                    for _ in 0..2 { // draws 1 pixel per dot
                        if self.background_fifo.len() > 8 {
                            if (self.tick_state.scanline_x == 0) && !self.rendered_window_on_scanline { // at the start of each scanline discard SCX mod 8 pixels from FIFO and push the rest to LCD ** A BIT INACCURATE EACH REMOVAL SHOULD BE A CYCLE
                                for _ in 0..(self.scx % 8) {
                                    self.background_fifo.remove(0);
                                }
                            }

                            let bg_color_id = self.background_fifo.remove(0);
                            let bg_color_value = (self.bgp >> (bg_color_id * 2)) & 0x3;

                            if self.sprite_fifo.len() > 0 {
                                let sprite = self.sprite_fifo.remove(0);
                                let sprite_color_value = self.get_object_color((sprite.flags >> 4) & 0x1, sprite.color_id);    

                                if sprite.color_id == 0x00 { // sprite is transparent so background is visible
                                    self.lcd[(self.ly as usize * 160) + self.tick_state.scanline_x] = bg_color_value;
                                } else if (sprite.flags >> 7) & 0x1 == 1 && bg_color_id != 0 { // background has priority and isn't transparent
                                    self.lcd[(self.ly as usize * 160) + self.tick_state.scanline_x] = bg_color_value;
                                } else { // otherwise just default to showing the sprite
                                    self.lcd[(self.ly as usize * 160) + self.tick_state.scanline_x] = sprite_color_value;
                                }
                            } else {
                                self.lcd[(self.ly as usize * 160) + self.tick_state.scanline_x] = bg_color_value;
                            }

                            self.tick_state.scanline_x += 1;
                        }

                        if !self.tick_state.is_fetching_window && self.window_in_frame && ((self.control >> WINDOW_ENABLED) & 0x1 == 1) && self.wx <= self.tick_state.scanline_x as u8 + 7 {
                            self.tick_state.bg_fetcher_step = 0;
                            self.tick_state.fetcher_x = 0;
                            self.background_fifo.clear();
                            self.tick_state.is_fetching_window = true;
                            break;
                        }

                        if self.tick_state.scanline_x > 159 {
                            self.update_mode(Mode::HBLANK);
                            break
                        }
                    }
                }
            },
            Mode::HBLANK => {
                self.tick_state = TickState::default();
                self.background_fifo.clear();
                self.sprite_fifo.clear();
                self.sprite_buffer.clear();

                if self.scanline_timeline == 456 { // 456 dots per scanline
                    if self.rendered_window_on_scanline {
                        self.window_line_counter += 1;
                        self.rendered_window_on_scanline = false;
                    }

                    self.stat_irq_triggered = false;
                    self.ly += 1;
                    if self.ly > 143 {
                        self.update_mode(Mode::VBLANK);
                    } else {
                        self.update_mode(Mode::OAMSCAN);
                    }
                }
            },
            Mode::VBLANK => {
                self.window_line_counter = 0;
                self.vblank_timeline += 2;
                if self.vblank_timeline == 4560 { // 4560 dots per vblank
                    self.vblank_timeline = 0;
                    self.ly = 0;
                    self.vblank_irq_triggered = false;
                    self.window_in_frame = false;
                    self.update_mode(Mode::OAMSCAN);
                    self.rendered_frame = true;
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
            obp0: 0x0,
            obp1: 0x0,
            tick_state: TickState::default(),
            scanline_timeline: 0,
            vblank_timeline: 0,
            vblank_irq_triggered: false,
            stat_irq_triggered: false,
            sprite_buffer: vec![],
            background_fifo: vec![],
            sprite_fifo: vec![],
            window_in_frame: false,
            window_line_counter: 0,
            rendered_window_on_scanline: false,
            rendered_frame: false
        }
    }
}

impl Default for TickState {
    fn default() -> Self {
        Self {
            oam_ptr: 0,
            fetcher_x: 0,
            scanline_x: 0,
            bg_fetcher_step: 0,
            sprite_fetcher_step: 0,
            tile_data_high: 0,
            tile_data_low: 0,
            tile_number: 0,
            new_scanline: true,
            current_sprite: None,
            is_fetching_window: false
        }
    }
}