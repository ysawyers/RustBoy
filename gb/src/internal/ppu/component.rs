pub struct PPU {
    pub lcd: Display,
    // vram
    // oam
    // LY
    // LYC
}

pub type Display = [[u8; 160]; 144];

impl PPU {
    pub fn read_vram(&self) {}
    
    pub fn write_vram(&self) {}

    pub fn update(&self) {}
}

impl Default for PPU {
    fn default() -> Self {
        Self {
            lcd: [[0; 160]; 144],
        }
    }
}