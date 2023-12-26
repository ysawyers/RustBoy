use crate::internal::ppu::component::PPU;
use crate::internal::ppu::component::Display;

pub struct Memory {
    // testing
    pub debug: bool,

    memory: [u8; 0x10000],
    boot_rom: [u8; 0x100],
    boot_rom_mounted: bool,

    // internal components
    ppu: PPU
}

impl Memory {
    pub fn mount_bootrom(&mut self, bytes: Vec<u8>) {
        if self.boot_rom.len() != bytes.len() {
            panic!("Invalid boot ROM!")
        }

        self.boot_rom_mounted = true;
        for i in 0..bytes.len() {
            self.boot_rom[i] = bytes[i];
        }
    }

    pub fn load_cartridge(&mut self, bytes: Vec<u8>) {
        for i in 0..bytes.len() {
            self.memory[i] = bytes[i];
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        if self.debug && addr == 0xFF44 { // for blargg test cases
            return 0x90;
        }

        if self.boot_rom_mounted && addr < 0x100 {
            return self.boot_rom[addr as usize];
        }
        return self.memory[addr as usize];
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        self.memory[addr as usize] = val;
    }

    pub fn update_components(&self) {
        self.ppu.update();
    }

    pub fn get_display(&self) -> Display {
        return self.ppu.lcd;
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            debug: false,
            memory: [0x0; 0x10000],
            boot_rom: [0x0; 0x100],
            boot_rom_mounted: false,
            ppu: PPU::default()
        }
    }
}