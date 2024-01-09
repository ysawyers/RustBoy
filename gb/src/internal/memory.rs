use crate::{console_log, log};
use crate::internal::ppu::{PPU, Display};
use crate::internal::timer::Timer;

#[derive(PartialEq)]
enum BankingMode {
    SIMPLE, ADVANCED
}

#[derive(PartialEq)]
enum MemoryBank {
    MBCNONE, MBC1, MBC1M
}

pub struct Memory {
    // testing
    pub flat_ram: bool,

    rom_chip: Vec<u8>,
    memory: [u8; 0x10000],

    boot_rom: [u8; 0x100],
    mbc_ram_enabled: bool,
    boot_rom_mounted: bool,

    memory_bank: MemoryBank,
    banking_mode: BankingMode,
    rom_bank_number: u8,
    ram_rom_bank_number: u8,

    pub IE: u8,
    pub IF: u8,

    pub keypress: i8,
    joyp: u8,

    ppu: PPU,
    timer: Timer
}

impl Memory {
    const NINTENDO_LOGO: [u8; 48] = [0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
                                      0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
                                      0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E];

    // fn mbc1_ram_bank_swap(&mut self) {
    //     for i in 0xA000..0xC000 {
    //         if self.mbc_ram_enabled {
    //             let offset = if self.banking_mode == BankingMode::ADVANCED { (self.ram_rom_bank_number as u16) << 13 } else { 0 } | (i & 0x1FFF);
    //             self.memory[i as usize] = self.rom_chip[(offset as usize) & (self.rom_chip.len() - 1)];
    //         } else {
    //             self.memory[i as usize] = 0xFF;
    //         }
    //     }
    // }

    pub fn mount_bootrom(&mut self, bytes: Vec<u8>) {
        for i in 0..bytes.len() {
            self.boot_rom[i] = bytes[i];
        }
        self.boot_rom_mounted = true;
    }

    pub fn load_cartridge(&mut self, bytes: Vec<u8>) {
        self.rom_chip = bytes;
        match self.rom_chip[0x0147] {
            0x00 => { // No MBC
                self.memory_bank = MemoryBank::MBCNONE;
                self.rom_chip.resize(0x10000, 0x00);
            },
            0x01..=0x03 => { // MBC1
                self.memory_bank = MemoryBank::MBC1;
                
                let mut logo_ptr = (Memory::NINTENDO_LOGO.len() - 1) as i8;
                for i in (0x4000..0x8000).rev() {
                    if logo_ptr < 0 {
                        self.memory_bank = MemoryBank::MBC1M;
                        console_log!("FOUND MBC1M");
                        unimplemented!("not implemented yet!");
                    }

                    let bank_ten_ptr = ((0x10 as u32) << 14) | (i & 0x3FFF);
                    if bank_ten_ptr >= self.rom_chip.len() as u32 { break } // out of bounds
                    if self.rom_chip[bank_ten_ptr as usize] == Memory::NINTENDO_LOGO[logo_ptr as usize] {
                        logo_ptr -= 1;
                    } else {
                        logo_ptr = (Memory::NINTENDO_LOGO.len() - 1) as i8;
                    }
                }
            }
            _ => {
                console_log!("0x{:02X}", self.rom_chip[0x0147]);
                unimplemented!("MBC NOT IMPLEMENTED YET!")
            }
        };
    }

    fn oam_dma_transfer(&mut self, source: u16) {
        for i in 0..0xA0 {
            self.ppu.oam[i] = self.rom_chip[((source as usize) + i) + 0x4000];
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        if self.boot_rom_mounted && addr <= 0xFF {
            return self.boot_rom[addr as usize]
        }

        match addr {
            0x0000..=0x3FFF => {
                let offset = if self.banking_mode == BankingMode::ADVANCED { ((self.ram_rom_bank_number as u32) << 19) | ((addr as u32) & 0x3FFF) } else { addr as u32 };
                return self.rom_chip[(offset as usize) & (self.rom_chip.len() - 1)];
            },
            0x4000..=0x7FFF => {
                let mut translated_bank_number = self.rom_bank_number & 0x1F;
                if translated_bank_number == 0x00 {
                    translated_bank_number |= 0x1;
                }
                let offset = ((self.ram_rom_bank_number as u32) << 19) | ((translated_bank_number as u32) << 14) | ((addr as u32) & 0x3FFF);
                return self.rom_chip[(offset as usize) & (self.rom_chip.len() - 1)];
            },
            0xA000..=0xBFFF => {
                if self.mbc_ram_enabled {
                    // TODO
                }
                return 0xFF
            },
            0x8000..=0x9FFF => self.ppu.read_vram(addr - 0x8000),
            0xFE00..=0xFE9F => self.ppu.read_oam(addr - 0xFE00),
            0xFF00 => {
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
            0xFF01 => 0xFF, // some serial register not implemented.
            0xFF04..=0xFF07 => self.timer.read_registers(addr),
            0xFF0F => self.IF,
            0xFF40..=0xFF4B => self.ppu.read_registers(addr),
            0xFFFF => self.IE,

            _ => self.rom_chip[(addr as usize) + 0x4000]
        }
    }

    fn mbc_handler(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x1FFF => self.mbc_ram_enabled = val & 0xF == 0xA,
            0x2000..=0x3FFF => self.rom_bank_number = val & 0x1F,
            0x4000..=0x5FFF => self.ram_rom_bank_number = val & 0x3,
            0x6000..=0x7FFF => self.banking_mode = if val & 0x1 == 1 { BankingMode::ADVANCED } else { BankingMode::SIMPLE },
            _ => ()
        }
    }

    // let rom_length = self.rom_chip.len() - 1;
    // let offset = if self.banking_mode == BankingMode::ADVANCED { (self.ram_rom_bank_number as u16) << 13 } else { 0 } | (addr & 0x1FFF);
    // self.rom_chip[(offset as usize) & rom_length] = val;

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x7FFF => self.mbc_handler(addr, val),
            0xA000..=0xBFFF => {
                if self.mbc_ram_enabled {
                    // TODO
                }
            }
            0x8000..=0x9FFF => self.ppu.write_vram(addr - 0x8000, val), // 8 KiB Video RAM (VRAM)
            0xC000..=0xCFFF => self.rom_chip[(addr as usize) + 0x4000] = val, // 4 KiB Work RAM (WRAM)
            0xD000..=0xDFFF => self.rom_chip[(addr as usize) + 0x4000] = val, // 4 KiB Work RAM (WRAM) In CGB mode, switchable bank 1~7
            0xFE00..=0xFE9F => self.ppu.write_oam(addr - 0xFE00, val), // Object attribute memory (OAM)
            0xFF00 => self.joyp = val,
            0xFF04..=0xFF07 => self.timer.write_registers(addr, val),
            0xFF0F => self.IF = val,
            0xFF46 => self.oam_dma_transfer((val as u16) << 8),
            0xFF40..=0xFF4B => self.ppu.write_registers(addr, val),
            0xFF50 => {
                if val & 0x1 == 1 { // bit 0 must be explicitly set to unmap the bootrom
                    self.boot_rom_mounted = false
                }
            },
            0xFF80..=0xFFFE => self.rom_chip[(addr as usize) + 0x4000] = val, // High RAM (HRAM)
            0xFFFF => self.IE = val,

            _ => ()
        }
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

        if self.timer.tima_irq > 0 { // starts at 2 to delay 1 cycle
            self.timer.tima_irq -= 1;
            if self.timer.tima_irq == 0 { 
                requests |= 0b00000100; // TIMER interrupt
            }
        }

        self.IF |= requests;
    }

    pub fn update_components(&mut self) { // 1 cycle
        self.ppu.update();
        self.timer.update();
    }

    pub fn get_display(&self) -> Display {
        self.ppu.lcd
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            rom_chip: vec![],
            banking_mode: BankingMode::SIMPLE,
            memory_bank: MemoryBank::MBCNONE,
            mbc_ram_enabled: false,
            memory: [0x0; 0x10000],
            boot_rom: [0x0; 0x100],
            boot_rom_mounted: false,
            ppu: PPU::default(),
            IE: 0x0,
            IF: 0x0,
            joyp: 0x0,
            keypress: -1,
            timer: Timer::default(),
            flat_ram: false,
            ram_rom_bank_number: 0x00,
            rom_bank_number: 0x00
        }
    }
}