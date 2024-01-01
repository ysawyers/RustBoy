use crate::internal::ppu::component::{PPU, Mode, Display};

const MBC_TYPE: u16 = 0x0147;

pub struct Memory {
    // testing
    pub debug: bool,

    memory: [u8; 0x10000],
    boot_rom: [u8; 0x100],
    pub boot_rom_mounted: bool,
    catridge_mounted: bool,

    // interrupts
    pub inte: u8,
    pub intf: u8,

    // timers
    div: u8,
    tima: u8,
    tma: u8,
    old_tma: Option<u8>,
    tac: u8,
    cycles_passed_div: usize,
    cycles_passed_tima: usize,
    tima_overflow: bool,

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

    pub fn read(&self, addr: u16) -> u8 {
        if self.boot_rom_mounted && addr <= 0xFF {
            return self.boot_rom[addr as usize]
        }
        
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
        if addr == 0xFF04 { return self.div }
        if addr == 0xFF05 { return self.tima }
        if addr == 0xFF06 { return self.tma }
        if addr == 0xFF07 { return self.tac }
        if addr == 0xFF0F { return self.intf }
        if addr == 0xFFFF { return self.inte }

        if addr == 0xFF00 { // JOYPAD
            return 0xFF
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

        if addr == 0xFF04 {
            self.div = 0;
            return
        }

        if addr == 0xFF05 {
            self.tima = val;
            return
        }

        if addr == 0xFF06 {
            self.old_tma.get_or_insert(self.tma);
            self.tma = val;
            return
        }

        if addr == 0xFF07 {
            self.tac = val;
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

        if addr >= 0xE000 && addr <= 0xFDFF { return } // prohibited
        if addr >= 0xFEA0 && addr <= 0xFEFF { return } // prohibited

        self.memory[addr as usize] = val
    }

    pub fn update_requested_interrupts(&mut self) { // "IF" register
        let mut requests: u8 = 0x0;

        if self.tima_overflow { // TIMER interrupt
            requests |= 0b00000100;
            self.tima_overflow = false;
        }

        if !self.ppu.new_mode.is_none() {
            match self.ppu.new_mode.as_ref().unwrap() {
                Mode::HBLANK => requests |= 0b00000010, // STAT interrupt
                Mode::VBLANK => requests |= 0b00000001, // VBLANK / STAT interrupt
                Mode::OAMSCAN => requests |= 0b00000010, // STAT interrupt
                _ => ()
            }
            self.ppu.new_mode = None;
        }

       if self.ppu.ly == self.ppu.lyc { requests |= 0b00000010 } // STAT interrupt

        self.intf |= requests;
    }

    // return number of cycles needed to increment TIMA
    fn clock_select(&self, clock: u8) -> usize {
        match clock {
            0 => 1024,
            1 => 16,
            2 => 64,
            3 => 256,
            _ => {
                panic!("UNEXPECTED!");
            }
        }
    }

    pub fn update_components(&mut self) {
        self.cycles_passed_div += 4;
        if (self.tac >> 2) & 0x1 == 1 { // TIMA ENABLED
            self.cycles_passed_tima += 4;
        }

        self.ppu.update();

        if self.cycles_passed_div == 256 {
            self.div = self.div.wrapping_add(1);
            self.cycles_passed_div = 0;
        }
        
        if self.cycles_passed_tima == self.clock_select(self.tac & 0x3) { // incremented at clock frequency specified by TAC if TIMA enabled
            let incr = self.tima.overflowing_add(1);
            if incr.1 { // if tima overflow reset to value specified by TMA
                self.tima = if self.old_tma.is_none() { self.tma } else { self.old_tma.unwrap() };
                self.tima_overflow = true;
            } else {
                self.tima = incr.0
            }
            self.cycles_passed_tima = 0;
        }
        
        if !self.old_tma.is_none() {
            self.old_tma = None;
        }
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
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0x0,
            inte: 0x0,
            intf: 0x0,
            cycles_passed_div: 0,
            cycles_passed_tima: 0,
            old_tma: None,
            tima_overflow: false
        }
    }
}