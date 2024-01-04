use crate::{internal::ppu::component::{PPU, Mode, Display}, console_log, log};

// const MBC_TYPE: u16 = 0x0147;

pub struct Memory {
    // testing
    pub debug: bool,

    memory: [u8; 0x10000],
    boot_rom: [u8; 0x100],
    boot_rom_mounted: bool,
    catridge_mounted: bool,

    // interrupts
    pub inte: u8,
    pub intf: u8,

    // joypad
    pub keypress: i8,
    joyp: u8,

    // components
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

    fn oam_dma_transfer(&mut self, source: u16) {
        for i in 0..0xA0 {
            self.ppu.oam[i] = self.memory[(source as usize) + i];
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        if self.boot_rom_mounted && addr <= 0xFF {
            return self.boot_rom[addr as usize]
        }

        if addr >= 0x8000 && addr <= 0x9FFF { return self.ppu.read_vram(addr - 0x8000) }
        if addr >= 0xFE00 && addr <= 0xFE9F { return self.ppu.read_oam(addr - 0xFE00) }
        if addr == 0xFF01 { return 0xFF }
        if addr == 0xFF40 { return self.ppu.control }
        if addr == 0xFF41 { return self.ppu.stat }
        if addr == 0xFF42 { return self.ppu.scy }
        if addr == 0xFF43 { return self.ppu.scx }
        if addr == 0xFF44 { return if self.debug { 0x90 } else { self.ppu.ly } }
        if addr == 0xFF45 { return self.ppu.lyc }
        if addr == 0xFF4A { return self.ppu.wy }
        if addr == 0xFF4B { return self.ppu.wx }
        if addr == 0xFF0F { return self.intf }
        if addr == 0xFFFF { return self.inte }
        if addr == 0xFF47 { return self.ppu.bgp }
        if addr == 0xFF48 { return self.ppu.obp0 }
        if addr == 0xFF49 { return self.ppu.obp1 }

        if addr == 0xFF00 { // JOYPAD
            if self.keypress != -1 {
                let mut buttons_pressed = 0xF;

                if ((self.joyp >> 4) & 0x1 == 0) && ((self.joyp >> 5) & 0x1 == 1) { // DPAD
                    buttons_pressed = match self.keypress {
                        1 => buttons_pressed & !(1 << 2), // UP
                        2 => buttons_pressed & !(1 << 1), // LEFT
                        3 => buttons_pressed & !(1 << 3), // DOWN
                        4 => buttons_pressed & !(1 << 0), // RIGHT 
                        _ => 0xF
                    };
                } else if ((self.joyp >> 5) & 0x1 == 0) && ((self.joyp >> 4) & 0x1 == 1) { // SELECT
                    buttons_pressed = match self.keypress {
                        5 => buttons_pressed & !(1 << 0), // A
                        6 => buttons_pressed & !(1 << 1), // B
                        7 => buttons_pressed & !(1 << 3), // START
                        8 => buttons_pressed & !(1 << 2), // SELECT 
                        _ => 0xF
                    };
                }

                return buttons_pressed
            }
            return 0xF;
        }

        if addr >= 0xE000 && addr <= 0xFDFF { return 0xFF } // prohibited
        if addr >= 0xFEA0 && addr <= 0xFEFF { return 0xFF } // prohibited

        self.memory[addr as usize]
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        if self.catridge_mounted && addr <= 0x7FFF { 
            // panic!("AHH WHAT DO I DO HERE???");
        }

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

        if addr == 0xFFFF {
            self.inte = val;
            return
        }

        if addr == 0xFF0F {
            self.intf = val;
            return
        }

        if addr == 0xFF00 {
            self.joyp = val;
            return
        }

        if addr == 0xFF46 {
            self.oam_dma_transfer((val as u16) << 8);
            return;
        }

        if addr == 0xFF47 { 
            self.ppu.bgp = val;
            return
        }

        if addr == 0xFF48 {
            self.ppu.obp0 = val;
            return
        }

        if addr == 0xFF49 {
            self.ppu.obp1 = val;
            return
        }

        if addr >= 0xE000 && addr <= 0xFDFF { return } // prohibited
        if addr >= 0xFEA0 && addr <= 0xFEFF { return } // prohibited

        self.memory[addr as usize] = val
    }

    pub fn update_requested_interrupts(&mut self) {
        let mut requests: u8 = 0x0;

        if !self.ppu.vblank_irq_triggered { // VBLANK interrupt
            requests |= 0b00000001; 
            self.ppu.vblank_irq_triggered = true;
        }

        if !self.ppu.stat_irq_triggered {
            if ((self.ppu.stat >> 6) & 0x1 == 1) && (self.ppu.ly == self.ppu.lyc) { requests |= 0b00000010; } // STAT interrupt (LY == LYC)
            if ((self.ppu.stat >> 5) & 0x1 == 1) && (self.ppu.stat & 0x3 == 2) { requests |= 0b00000010 }; // STAT interrupt (OAM)
            if ((self.ppu.stat >> 4) & 0x1 == 1) && (self.ppu.stat & 0x3 == 1) { requests |= 0b00000010 }; // STAT interrupt (VBLANK)
            if ((self.ppu.stat >> 3) & 0x1 == 1) && (self.ppu.stat & 0x3 == 0) { requests |= 0b00000010 }; // STAT interrupt (HBLANK)
            self.ppu.stat_irq_triggered = true;
        }

        self.intf |= requests;
    }

    pub fn update_components(&mut self) {
        self.ppu.update();
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
            ppu: PPU::default(),
            inte: 0x0,
            intf: 0x0,
            joyp: 0x0,
            keypress: -1
        }
    }
}