use crate::log;
use crate::console_log;

use crate::internal::ppu::component::PPU;
use crate::internal::ppu::component::Display;

const MBC_TYPE: u16 = 0x0147;

pub struct Memory {
    // testing
    pub debug: bool,

    memory: [u8; 0x10000],
    boot_rom: [u8; 0x100],
    boot_rom_mounted: bool,
    catridge_mounted: bool,

    // internal components
    ppu: PPU
}

impl Memory {
    pub fn mount_bootrom(&mut self, bytes: Vec<u8>) {
        for i in 0..bytes.len() {
            self.boot_rom[i] = bytes[i];
        }
        self.boot_rom_mounted = true;
    }

    pub fn load_cartridge(&mut self, bytes: Vec<u8>) {
        for i in 0..bytes.len() {
            self.write(i as u16, bytes[i]);
        }
        self.catridge_mounted = true;
    }

    pub fn read(&self, addr: u16) -> u8 {
        if self.boot_rom_mounted && addr <= 0xFF { return self.boot_rom[addr as usize] }
        
        if addr >= 0x8000 && addr <= 0x9FFF { return self.ppu.read_vram(addr - 0x8000) }
        if addr >= 0xFE00 && addr <= 0xFE9F { return self.ppu.read_oam(addr - 0xFE00) }
        if addr == 0xFF40 { return self.ppu.control }
        if addr == 0xFF41 { return self.ppu.stat }
        if addr == 0xFF42 { return self.ppu.scy }
        if addr == 0xFF43 { return self.ppu.scx }
        if addr == 0xFF44 { return if self.debug { 0x90 } else { self.ppu.ly } }
        if addr == 0xFF45 { return self.ppu.lyc }
        if addr == 0xFF4A { return self.ppu.wy }
        if addr == 0xFF4B { return self.ppu.wx }
        
        self.memory[addr as usize]
    }

    pub fn write(&mut self, addr: u16, val: u8) {      
        if self.catridge_mounted && addr <= 0x7FFF { todo!("Needs Implementing MBCS's") }

        if addr >= 0x8000 && addr <= 0x9FFF {
            self.ppu.write_vram(addr - 0x8000, val);
            return
        }

        if addr >= 0xFE00 && addr <= 0xFE9F {
            self.ppu.write_oam(addr - 0xFE00, val);
            return
        }

        if addr == 0xFF40 {
            self.ppu.control = val;
            return
        }

        if addr == 0xFF41 {
            self.ppu.stat = val;
            return
        }

        if addr == 0xFF42 {
            self.ppu.scy = val;
            return
        }

        if addr == 0xFF43 {
            self.ppu.scx = val;
            return;
        }

        if addr == 0xFF45 {
            self.ppu.lyc = val;
            return
        }

        if addr == 0xFF50 {
            self.boot_rom_mounted = false;
            return
        }

        if addr == 0xFF4A {
            self.ppu.wy = val;
            return
        }

        if addr == 0xFF4B {
            self.ppu.wx = val;
            return
        }

        self.memory[addr as usize] = val
    }

    pub fn update_components(&mut self) {
        self.ppu.update()
    }

    pub fn get_display(&self) -> Display {
        self.ppu.lcd
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            debug: false,
            memory: [0x0; 0x10000],
            boot_rom: [0x0; 0x100],
            boot_rom_mounted: false,
            catridge_mounted: false,
            ppu: PPU::default()
        }
    }
}

// let cartridge_type = bytes.get(0x147).expect("Invalid cartridge.");
// match cartridge_type {
//     01 => println!("NORMAL"),
//     _ => println!("Not mapped ${:02X}", cartridge_type),
// }