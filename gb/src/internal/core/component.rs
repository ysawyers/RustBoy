use crate::internal::ppu::component::Display;
use crate ::internal::memory::Memory;
use crate::internal::core::registers::{Register, Registers, Flag};

const CYCLES_PER_FRAME: u32 = 69905;

pub struct CPU {
    pub bus: Memory,
    pub registers: Registers,
    pub pc: u16,
    pub sp: u16,
    pub tick_state: Option<TickState>
}

struct TickState {
    is_prefix: bool,
    instr: Vec<MicroInstr>,
    step: usize,
    pub b8: u8,
    pub b16: u8,
}

#[derive(Clone, PartialEq, Eq)]
pub enum MicroInstr {
    NOP,

    // FUNCTIONS
    Read(Byte),
    Cond(Flag, bool),
    
    // LOADS
    LDRR(Register, Register),
    LDRN(Register),
    LDNNA(u16, bool),
    LDANN(u16, bool),
    LDAHLINC,
    LDSPNN,

    // INC/DEC
    INCR(Register),
    DECR(Register),

    INCHL,
    INCBC,

    // JUMPS / CALLS / RETS
    JP,
    JR,
    PUSH(u8),
    POPPC(Byte),
    POPR(Register),

    // OPERATIONS
    ANDRN(Register),
    ORRR(Register, Register),
    CPRN(Register),

    // INTERRUPTS
    DI
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Byte {
    LSB, MSB
}

impl CPU {
    fn fetch_instr(&mut self) -> (u8, Vec<MicroInstr>) {
        let opcode = self.bus.read(self.pc);
        self.pc += 1;

        (opcode, self.decode_instr(opcode))
    }

    fn fetch_prefix_instr(&mut self) -> Vec<MicroInstr> {
        let opcode = self.bus.read(self.pc);
        self.pc += 1;

        self.decode_prefix_instr(opcode)
    }

    pub fn execute(&mut self) {
        if self.tick_state.is_none() {
            let instr = self.fetch_instr();
            let tick_state = TickState{
                instr: instr.1,
                step: 0,
                is_prefix: instr.0 == 0xCB,
                b8: 0,
                b16: 0
            };
            if self.tick_state.get_or_insert(tick_state).is_prefix { return };
        }

        if self.tick_state.as_ref().unwrap().is_prefix {
            let instr = self.fetch_prefix_instr();
            self.tick_state.as_mut().unwrap().instr = instr;
            self.tick_state.as_mut().unwrap().is_prefix = false;
        }

        match self.tick_state.as_ref().unwrap().instr[self.tick_state.as_ref().unwrap().step] {
            MicroInstr::NOP => (),
            MicroInstr::Read(byte) => {
                match byte {
                    Byte::LSB => self.tick_state.as_mut().unwrap().b8 = self.bus.read(self.pc),
                    Byte::MSB => self.tick_state.as_mut().unwrap().b16 = self.bus.read(self.pc)
                }
                self.pc += 1;
            },
            MicroInstr::LDRN(r) => {
                self.registers[r] = self.bus.read(self.pc);
                self.pc += 1;
            },
            MicroInstr::LDRR(r1, r2) => self.registers[r1] = self.registers[r2],
            MicroInstr::LDAHLINC => {
                let val = self.bus.read(self.registers.get_hl());
                self.registers[Register::A] = val;
                self.registers.set_hl(self.registers.get_hl().wrapping_add(1));
            }
            MicroInstr::LDNNA(preset, is_offset) => {
                match preset {
                    0 => self.bus.write(((self.tick_state.as_ref().unwrap().b16 as u16) << 8) | (self.tick_state.as_ref().unwrap().b8 as u16), self.registers[Register::A]),
                    _ => {
                        let addr = if is_offset { preset | self.tick_state.as_ref().unwrap().b8 as u16 } else { preset };
                        self.bus.write(addr, self.registers[Register::A])
                    }
                }
            },
            MicroInstr::LDANN(preset, is_offset) => {
                match preset {    
                    0 => self.registers[Register::A] = self.bus.read(((self.tick_state.as_ref().unwrap().b16 as u16) << 8) | (self.tick_state.as_ref().unwrap().b8 as u16)),
                    _ => {
                        let addr = if is_offset { preset | self.tick_state.as_ref().unwrap().b8 as u16 } else { preset };
                        self.registers[Register::A] = self.bus.read(addr);
                    }
                }
            },
            MicroInstr::Cond(flag, is_set) => {
                if self.registers.get_flag(flag) != (is_set as u8) {
                    self.tick_state = None;
                    return
                }
            },
            MicroInstr::JP => self.pc = ((self.tick_state.as_ref().unwrap().b16 as u16) << 8) | (self.tick_state.as_ref().unwrap().b8 as u16),
            MicroInstr::JR => self.pc = self.pc.wrapping_add_signed(self.tick_state.as_ref().unwrap().b8 as i8 as i16),
            MicroInstr::PUSH(val) => {
                self.sp -= 1;
                self.bus.write(self.sp, val);
            },
            MicroInstr::POPPC(byte) => {
                match byte {
                    Byte::LSB => self.tick_state.as_mut().unwrap().b8 = self.bus.read(self.sp),
                    Byte::MSB => self.tick_state.as_mut().unwrap().b16 = self.bus.read(self.sp)
                }
                self.sp += 1;
            },
            MicroInstr::POPR(register) => {
                self.registers[register] = self.bus.read(self.sp);
                self.sp += 1;
            },
            MicroInstr::INCR(r) => {
                self.registers.set_flag(Flag::Z, self.registers[r].wrapping_add(1) == 0);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, (((self.registers[r] & 0xF).wrapping_add(1 & 0xF)) & 0x10) == 0x10);
                self.registers[r] = self.registers[r].wrapping_add(1);
            }
            MicroInstr::DECR(r) => {
                self.registers.set_flag(Flag::Z, self.registers[r].wrapping_sub(1) == 0);
                self.registers.set_flag(Flag::N, true);
                self.registers.set_flag(Flag::H, (((self.registers[r] & 0xF).wrapping_sub(1 & 0xF)) & 0x10) == 0x10);
                self.registers[r] = self.registers[r].wrapping_sub(1);
            }
            MicroInstr::INCHL => self.registers.set_hl(self.registers.get_hl().wrapping_add(1)),
            MicroInstr::INCBC => self.registers.set_bc(self.registers.get_bc().wrapping_add(1)),
            MicroInstr::DI => { /* TODO */ },
            MicroInstr::LDSPNN => self.sp = ((self.tick_state.as_ref().unwrap().b16 as u16) << 8) | (self.tick_state.as_ref().unwrap().b8 as u16),
            MicroInstr::ORRR(r1, r2) => {
                self.registers[r1] |= self.registers[r2];
                self.registers.set_flag(Flag::Z, self.registers[r1] == 0);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, false);
                self.registers.set_flag(Flag::C, false);
            }
            MicroInstr::CPRN(register) => {
                self.registers.set_flag(Flag::Z, self.registers[register].wrapping_sub(self.tick_state.as_ref().unwrap().b8) == 0);
                self.registers.set_flag(Flag::N, true);
                self.registers.set_flag(Flag::H, (((self.registers[register] & 0xF).wrapping_sub(self.tick_state.as_ref().unwrap().b8 & 0xF)) & 0x10) == 0x10);
                self.registers.set_flag(Flag::C, self.registers[register] < self.tick_state.as_ref().unwrap().b8);
            },
            MicroInstr::ANDRN(register) => {
                self.registers[register] &= self.tick_state.as_ref().unwrap().b8;
                self.registers.set_flag(Flag::Z, self.registers[register] == 0);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, true);
                self.registers.set_flag(Flag::C, false);
            },
        }
        self.tick_state.as_mut().unwrap().step += 1;

        if self.tick_state.as_ref().unwrap().step >= self.tick_state.as_ref().unwrap().instr.len() {
            self.tick_state = None
        };
    }

    pub fn next_frame(&mut self) -> Display {
        for _ in 0..CYCLES_PER_FRAME {
            self.execute();
            self.bus.update_components();
        }
        return self.bus.get_display();
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            bus: Memory::default(),
            registers: Registers::default(),
            sp: 0x0,
            pc: 0x0,
            tick_state: None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn blargg_cpu_tests() {
        let files = fs::read_dir("./logs/blargg").unwrap();

        for file in files {
            let mut core = CPU::default();

            let file_name = file.as_ref().unwrap().file_name();
            let file_name_parts: Vec<_> = file_name.to_str().unwrap().split(".").collect();

            let bytes = fs::read(String::from("./logs/blargg/") + file_name_parts[0] + ".gb").expect("File not found!");
            core.bus.load_cartridge(bytes);
            core.bus.debug = true;

            core.pc = 0x100;
            core.sp = 0xFFFE;
            
            core.registers[Register::A] = 0x01;
            core.registers[Register::F] = 0xB0;
            core.registers[Register::B] = 0x00;
            core.registers[Register::C] = 0x13;
            core.registers[Register::D] = 0x00;
            core.registers[Register::E] = 0xD8;
            core.registers[Register::H] = 0x01;
            core.registers[Register::L] = 0x4D;

            let body = fs::read_to_string(String::from("./logs/blargg/") + file_name.to_str().unwrap()).unwrap();

            let mut prev = core.bus.read(core.pc);
            let mut lines = 0;

            for line in body.lines() {
                let core_state = format!(
                    "A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}",
                    core.registers[Register::A], core.registers[Register::F], core.registers[Register::B], core.registers[Register::C], core.registers[Register::D], 
                    core.registers[Register::E], core.registers[Register::H], core.registers[Register::L], core.sp, core.pc, 
                    core.bus.read(core.pc), core.bus.read(core.pc+1), core.bus.read(core.pc+2), core.bus.read(core.pc+3)
                );
                lines += 1;

                assert_eq!(core_state, line, "PROBLEM WITH OPCODE 0x{:02X} LINE: {}", prev, lines);

                prev = core.bus.read(core.pc);

                core.execute();
                while !core.tick_state.is_none() {
                    core.execute();
                }
                
            }
        }
    }
}