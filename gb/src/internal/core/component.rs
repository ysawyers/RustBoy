use crate::internal::ppu::Display;
use crate ::internal::memory::Memory;
use crate::internal::core::registers::{Register, Registers, Flag};
use crate::u32_to_little_endian;
use std;

pub struct CPU {
    pub registers: Registers,
    pub pc: u16,
    pub sp: u16,
    pub bus: Memory,
    pub ime: bool,
    should_enable_ime: usize,
    tick_state: Option<TickState>,
    interrupt_tick_state: Option<InterruptTickState>,
    is_halted: bool,
    halt_bug: bool,
}

pub struct Instruction {
    pub name: String,
    pub steps: Vec<MicroInstr>
}

struct TickState {
    is_prefix: bool,
    instr: Vec<MicroInstr>,
    step: usize,
    b8: u8,
    b16: u8,
}

struct InterruptTickState {
    interrupt: Interrupt,
    step: usize
}

enum Interrupt {
    VBLANK, STAT, TIMER
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum MicroInstr {
    // FUNCTIONS
    Read(Byte),
    Cond(Flag, bool),
    
    // LOADS
    LDRR(Register, Register),
    LDRN(Register),
    LDNNR(u16, Register, bool),
    LDRNN(Register, u16, bool),
    LDHLN,
    LDAHLINC,
    LDHLINCA,
    LDHLDECA,
    LDAHLDEC,
    LDSPNN,
    LDNNSP(Byte),
    LDSPHL,
    LDHLSPN,

    // INC/DEC
    INC(Register),
    DEC(Register),
    INCHLADDR,
    INCHL,
    INCBC,
    INCDE,
    INCSP,
    DECBC,
    DECDE,
    DECHL,
    DECSP,
    DECNN(u16),

    // JUMPS / CALLS / RETS
    JP,
    JPHL,
    JR,
    PUSH(u8),
    POPPC(Byte),
    POPR(Register),
    RST(u16),

    // OPERATIONS
    OR(Register),
    ORN,
    ORHL,
    XOR(Register),
    XORN,
    XORHL,
    CP(Register),
    CPN,
    CPHL,
    AND(Register),
    ANDHL,
    ANDN,
    ADD(Register),
    ADDN,
    ADDHL,
    ADDHLNN(u16),
    ADDSPN,
    ADC(Register),
    ADCHL,
    ADCN,
    SUB(Register),
    SUBHL,
    SUBN,
    SRLR(Register),
    SRLHL,
    RRR(Register),
    RRHL,
    RRA,
    RRCHL,
    RRCR(Register),
    SBC(Register),
    SBCHL,
    SBCN,
    SLAHL,
    SLAR(Register),
    DAA,
    RLCR(Register),
    RLCHL,
    RLHL,
    RLR(Register),
    SRAHL,
    SRAR(Register),
    SWAP(Register),
    SWAPHL,
    BIT(u8, Register),
    BITHL(u8),
    RES(u8, Register),
    RESHL(u8),
    SET(u8, Register),
    SETHL(u8),
    RLCA,
    RLA,
    RRCA,
    
    // SPECIAL
    CPL,
    SCF,
    CCF,
    NOP,
    HALT,
    STOP,

    // INTERRUPTS
    DI,
    RETI,
    EI
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Byte {
    LSB, MSB
}

impl CPU {
    fn fetch_instr(&mut self) -> (u8, Vec<MicroInstr>) {
        let opcode = self.bus.read(self.pc);
        self.pc += 1;

        (opcode, self.decode_instr(opcode))
    }

    fn fetch_prefix_instr(&mut self) -> (u8, Vec<MicroInstr>) {
        let opcode = self.bus.read(self.pc);        
        self.pc = self.pc.wrapping_add(1);

        (opcode, self.decode_prefix_instr(opcode))
    }

    fn execute(&mut self) {
        if self.tick_state.is_none() {
            let instr = self.fetch_instr();

            let tick_state = TickState{
                instr: instr.1,
                step: 0,
                is_prefix: instr.0 == 0xCB,
                b8: 0,
                b16: 0
            };

            self.tick_state.get_or_insert(tick_state);
        }

        let state = self.tick_state.as_mut().unwrap();

        if state.is_prefix {
            let instr: (u8, Vec<MicroInstr>) = self.fetch_prefix_instr();

            self.tick_state.as_mut().unwrap().instr = instr.1;
            self.tick_state.as_mut().unwrap().is_prefix = false;
            return
        }

        match state.instr[state.step] {
            MicroInstr::NOP => (),
            MicroInstr::Read(byte) => {
                match byte {
                    Byte::LSB => state.b8 = self.bus.read(self.pc),
                    Byte::MSB => state.b16 = self.bus.read(self.pc)
                }
                self.pc += 1;
            },
            MicroInstr::LDRN(r) => {
                self.registers[r] = self.bus.read(self.pc);
                self.pc += 1;
            },
            MicroInstr::LDRR(r1, r2) => self.registers[r1] = self.registers[r2],
            MicroInstr::LDAHLINC => {
                self.registers[Register::A] = self.bus.read(self.registers.get_hl());
                self.registers.set_hl(self.registers.get_hl().wrapping_add(1));
            }
            MicroInstr::CCF => {
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, false);
                self.registers.set_flag(Flag::C, self.registers.get_flag(Flag::C) == 0);
            },
            MicroInstr::SCF => {
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, false);
                self.registers.set_flag(Flag::C, true);
            },
            MicroInstr::LDHLINCA => {
                self.bus.write(self.registers.get_hl(), self.registers[Register::A]);
                self.registers.set_hl(self.registers.get_hl().wrapping_add(1));
            },
            MicroInstr::LDHLDECA => {
                self.bus.write(self.registers.get_hl(), self.registers[Register::A]);
                self.registers.set_hl(self.registers.get_hl().wrapping_sub(1));
            },
            MicroInstr::LDNNR(preset, register, is_offset) => {
                match preset {
                    0 => self.bus.write(((state.b16 as u16) << 8) | (state.b8 as u16), self.registers[register]),
                    _ => {
                        let addr = if is_offset { preset | state.b8 as u16 } else { preset };
                        self.bus.write(addr, self.registers[register])
                    }
                }
            },
            MicroInstr::LDRNN(register, preset, is_offset) => {
                match preset {
                    0 => self.registers[register] = self.bus.read(((state.b16 as u16) << 8) | (state.b8 as u16)),
                    _ => {
                        let addr = if is_offset { preset | state.b8 as u16 } else { preset };
                        self.registers[register] = self.bus.read(addr);
                    }
                }
            },
            MicroInstr::Cond(flag, is_set) => {
                if self.registers.get_flag(flag) != (is_set as u8) {
                    self.tick_state = None;
                    return
                }
            },
            MicroInstr::JP => self.pc = ((state.b16 as u16) << 8) | (state.b8 as u16),
            MicroInstr::JR => self.pc = self.pc.wrapping_add_signed(state.b8 as i8 as i16),
            MicroInstr::PUSH(val) => {
                self.sp -= 1;
                self.bus.write(self.sp, val);
            },
            MicroInstr::POPPC(byte) => {
                match byte {
                    Byte::LSB => state.b8 = self.bus.read(self.sp),
                    Byte::MSB => state.b16 = self.bus.read(self.sp)
                }
                self.sp += 1;
            },
            MicroInstr::POPR(register) => {
                self.registers[register] = if register == Register::F { self.bus.read(self.sp) & 0xF0 } else { self.bus.read(self.sp) };
                self.sp += 1;
            },
            MicroInstr::INC(register) => {
                self.registers.set_flag(Flag::Z, self.registers[register].wrapping_add(1) == 0);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, (((self.registers[register] & 0xF).wrapping_add(1 & 0xF)) & 0x10) == 0x10);
                self.registers[register] = self.registers[register].wrapping_add(1);
            }
            MicroInstr::DEC(register) => {
                self.registers.set_flag(Flag::Z, self.registers[register].wrapping_sub(1) == 0);
                self.registers.set_flag(Flag::N, true);
                self.registers.set_flag(Flag::H, (((self.registers[register] & 0xF).wrapping_sub(1 & 0xF)) & 0x10) == 0x10);
                self.registers[register] = self.registers[register].wrapping_sub(1);
            }
            MicroInstr::INCHL => self.registers.set_hl(self.registers.get_hl().wrapping_add(1)),
            MicroInstr::INCBC => self.registers.set_bc(self.registers.get_bc().wrapping_add(1)),
            MicroInstr::INCDE => self.registers.set_de(self.registers.get_de().wrapping_add(1)),
            MicroInstr::DI => {
                self.ime = false;
                self.should_enable_ime = 0;
            },
            MicroInstr::LDSPNN => self.sp = ((state.b16 as u16) << 8) | (state.b8 as u16),
            MicroInstr::OR(register) => {
                self.registers[Register::A] |= self.registers[register];
                self.registers.set_flag(Flag::Z, self.registers[Register::A] == 0);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, false);
                self.registers.set_flag(Flag::C, false);
            }
            MicroInstr::ORHL => {
                self.registers[Register::A] |= self.bus.read(self.registers.get_hl());
                self.registers.set_flag(Flag::Z, self.registers[Register::A] == 0);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, false);
                self.registers.set_flag(Flag::C, false);
            }
            MicroInstr::CP(register) => {
                self.registers.set_flag(Flag::Z, self.registers[Register::A].wrapping_sub(self.registers[register]) == 0);
                self.registers.set_flag(Flag::N, true);
                self.registers.set_flag(Flag::H, (((self.registers[Register::A] & 0xF).wrapping_sub(self.registers[register] & 0xF)) & 0x10) == 0x10);
                self.registers.set_flag(Flag::C, self.registers[Register::A] < self.registers[register]);
            },
            MicroInstr::CPN => {
                self.registers.set_flag(Flag::Z, self.registers[Register::A].wrapping_sub(state.b8) == 0);
                self.registers.set_flag(Flag::N, true);
                self.registers.set_flag(Flag::H, (((self.registers[Register::A] & 0xF).wrapping_sub(state.b8 & 0xF)) & 0x10) == 0x10);
                self.registers.set_flag(Flag::C, self.registers[Register::A] < state.b8);
            },
            MicroInstr::AND(register) => {
                self.registers[Register::A] &= self.registers[register];
                self.registers.set_flag(Flag::Z, self.registers[Register::A] == 0);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, true);
                self.registers.set_flag(Flag::C, false);
            },
            MicroInstr::ANDN => {
                self.registers[Register::A] &= state.b8;
                self.registers.set_flag(Flag::Z, self.registers[Register::A] == 0);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, true);
                self.registers.set_flag(Flag::C, false);
            },
            MicroInstr::ANDHL => {
                self.registers[Register::A] &= self.bus.read(self.registers.get_hl());
                self.registers.set_flag(Flag::Z, self.registers[Register::A] == 0);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, true);
                self.registers.set_flag(Flag::C, false);
            },
            MicroInstr::ADD(register) => {
                self.registers.set_flag(Flag::Z, self.registers[Register::A].wrapping_add(self.registers[register]) == 0);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, (((self.registers[Register::A] & 0xF).wrapping_add(self.registers[register] & 0xF)) & 0x10) == 0x10);
                self.registers.set_flag(Flag::C, self.registers[Register::A] as u16 + self.registers[register] as u16 > 0xFF);
                self.registers[Register::A] = self.registers[Register::A].wrapping_add(self.registers[register]);
            },
            MicroInstr::ADDN => {
                self.registers.set_flag(Flag::Z, self.registers[Register::A].wrapping_add(state.b8) == 0);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, (((self.registers[Register::A] & 0xF).wrapping_add(state.b8 & 0xF)) & 0x10) == 0x10);
                self.registers.set_flag(Flag::C, self.registers[Register::A] as u16 + state.b8 as u16 > 0xFF);
                self.registers[Register::A] = self.registers[Register::A].wrapping_add(state.b8);
            },
            MicroInstr::SUB(register) => {
                self.registers.set_flag(Flag::Z, self.registers[Register::A].wrapping_sub(self.registers[register]) == 0);
                self.registers.set_flag(Flag::N, true);
                self.registers.set_flag(Flag::H, (((self.registers[Register::A] & 0xF).wrapping_sub(self.registers[register] & 0xF)) & 0x10) == 0x10);
                self.registers.set_flag(Flag::C, self.registers[Register::A] < self.registers[register]);
                self.registers[Register::A] = self.registers[Register::A].wrapping_sub(self.registers[register]);
            },
            MicroInstr::SUBN => {
                self.registers.set_flag(Flag::Z, self.registers[Register::A].wrapping_sub(state.b8) == 0);
                self.registers.set_flag(Flag::N, true);
                self.registers.set_flag(Flag::H, (((self.registers[Register::A] & 0xF).wrapping_sub(state.b8 & 0xF)) & 0x10) == 0x10);
                self.registers.set_flag(Flag::C, self.registers[Register::A] < state.b8);
                self.registers[Register::A] = self.registers[Register::A].wrapping_sub(state.b8);
            }
            MicroInstr::XOR(register) => {
                self.registers[Register::A] ^= self.registers[register];
                self.registers.set_flag(Flag::Z, self.registers[Register::A] == 0);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, false);
                self.registers.set_flag(Flag::C, false);
            },
            MicroInstr::XORHL => {
                self.registers[Register::A] ^= self.bus.read(self.registers.get_hl());
                self.registers.set_flag(Flag::Z, self.registers[Register::A] == 0);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, false);
                self.registers.set_flag(Flag::C, false);
            },
            MicroInstr::XORN => {
                self.registers[Register::A] ^= state.b8;
                self.registers.set_flag(Flag::Z, self.registers[Register::A] == 0);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, false);
                self.registers.set_flag(Flag::C, false);
            },
            MicroInstr::SRLHL => {
                self.registers.set_flag(Flag::C, self.bus.read(self.registers.get_hl()) & 0x1 == 1);
                self.bus.write(self.registers.get_hl(), self.bus.read(self.registers.get_hl()) >> 1);
                self.registers.set_flag(Flag::Z, self.bus.read(self.registers.get_hl()) == 0);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, false);
            },
            MicroInstr::SRLR(register) => {
                self.registers.set_flag(Flag::C, self.registers[register] & 0x1 == 1);
                self.registers[register] >>= 1;
                self.registers.set_flag(Flag::Z, self.registers[register] == 0);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, false);
            },
            MicroInstr::RRR(register) => {
                let b0 = self.registers[register] & 0x1;
                self.registers[register] >>= 1;
                self.registers[register] = if self.registers.get_flag(Flag::C) == 1 { self.registers[register] | 0b10000000 } else { self.registers[register] & 0b01111111 };
                self.registers.set_flag(Flag::Z, self.registers[register] == 0);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, false);
                self.registers.set_flag(Flag::C, b0 == 1);
            },
            MicroInstr::RRHL => {
                let b0 = self.bus.read(self.registers.get_hl()) & 0x1;
                self.bus.write(self.registers.get_hl(), self.bus.read(self.registers.get_hl()) >> 1);
                if self.registers.get_flag(Flag::C) == 1 {
                    self.bus.write(self.registers.get_hl(), self.bus.read(self.registers.get_hl()) | 0b10000000);
                } else {
                    self.bus.write(self.registers.get_hl(), self.bus.read(self.registers.get_hl()) & 0b01111111);
                }
                self.registers.set_flag(Flag::Z, self.bus.read(self.registers.get_hl()) == 0);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, false);
                self.registers.set_flag(Flag::C, b0 == 1);
            },
            MicroInstr::RRA => {
                let b0 = self.registers[Register::A] & 0x1;
                self.registers[Register::A] >>= 1;
                self.registers[Register::A] = if self.registers.get_flag(Flag::C) == 1 { self.registers[Register::A] | 0b10000000 } else { self.registers[Register::A] & 0b01111111 };
                self.registers.set_flag(Flag::Z, false);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, false);
                self.registers.set_flag(Flag::C, b0 == 1);
            },
            MicroInstr::ADC(register) => {
                let c = self.registers.get_flag(Flag::C);
                self.registers.set_flag(Flag::Z, self.registers[Register::A].wrapping_add(self.registers[register]).wrapping_add(c) == 0);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, (((self.registers[Register::A] & 0xF).wrapping_add(self.registers[register] & 0xF).wrapping_add(c)) & 0x10) == 0x10);
                self.registers.set_flag(Flag::C, ((self.registers[Register::A] as u16).wrapping_add(self.registers[register] as u16).wrapping_add(c as u16)) > 0xFF);
                self.registers[Register::A] = self.registers[Register::A].wrapping_add(self.registers[register]).wrapping_add(c);
            },
            MicroInstr::ADCN => {
                let c = self.registers.get_flag(Flag::C);
                self.registers.set_flag(Flag::Z, self.registers[Register::A].wrapping_add(state.b8).wrapping_add(c) == 0);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, (((self.registers[Register::A] & 0xF).wrapping_add(state.b8 & 0xF).wrapping_add(c)) & 0x10) == 0x10);
                self.registers.set_flag(Flag::C, ((self.registers[Register::A] as u16).wrapping_add(state.b8 as u16).wrapping_add(c as u16)) > 0xFF);
                self.registers[Register::A] = self.registers[Register::A].wrapping_add(state.b8).wrapping_add(c);
            }
            MicroInstr::DECNN(addr) => {
                self.registers.set_flag(Flag::Z, self.bus.read(addr).wrapping_sub(1) == 0);
                self.registers.set_flag(Flag::N, true);
                self.registers.set_flag(Flag::H, ((self.bus.read(addr) & 0xF).wrapping_sub(1 & 0xF) & 0x10) == 0x10);
                self.bus.write(addr, self.bus.read(addr).wrapping_sub(1));
            },
            MicroInstr::ADDHLNN(val) => {
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, (((self.registers.get_hl() & 0x0FFF) + (val & 0x0FFF)) & 0x1000) == 0x1000);
                self.registers.set_flag(Flag::C, (self.registers.get_hl() as u32).wrapping_add(val as u32) > 0xFFFF);
                self.registers.set_hl(self.registers.get_hl().wrapping_add(val));
            },
            MicroInstr::ADDHL => {
                self.registers.set_flag(Flag::Z, self.registers[Register::A].wrapping_add(self.bus.read(self.registers.get_hl())) == 0);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, (((self.registers[Register::A] & 0xF).wrapping_add(self.bus.read(self.registers.get_hl()) & 0xF)) & 0x10) == 0x10);
                self.registers.set_flag(Flag::C, self.registers[Register::A] as u16 + self.bus.read(self.registers.get_hl()) as u16 > 0xFF);
                self.registers[Register::A] = self.registers[Register::A].wrapping_add(self.bus.read(self.registers.get_hl()));
            },
            MicroInstr::JPHL => self.pc = self.registers.get_hl(),
            MicroInstr::LDHLN => self.bus.write(self.registers.get_hl(), state.b8),
            MicroInstr::ORN => {
                self.registers[Register::A] |= state.b8;
                self.registers.set_flag(Flag::Z, self.registers[Register::A] == 0);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, false);
                self.registers.set_flag(Flag::C, false);
            },
            MicroInstr::SBC(register) => {
                let c = self.registers.get_flag(Flag::C);
                self.registers.set_flag(Flag::Z, self.registers[Register::A].wrapping_sub(self.registers[register].wrapping_add(c)) == 0);
                self.registers.set_flag(Flag::N, true);
                self.registers.set_flag(Flag::H, (((self.registers[Register::A] & 0xF).wrapping_sub(self.registers[register] & 0xF).wrapping_sub(c)) & 0x10) == 0x10);
                self.registers.set_flag(Flag::C, (self.registers[Register::A] as u16) < (self.registers[register] as u16).wrapping_add(c as u16));
                self.registers[Register::A] = self.registers[Register::A].wrapping_sub(self.registers[register].wrapping_add(c)); 
            },
            MicroInstr::SBCN => {
                let c = self.registers.get_flag(Flag::C);
                self.registers.set_flag(Flag::Z, self.registers[Register::A].wrapping_sub(state.b8.wrapping_add(c)) == 0);
                self.registers.set_flag(Flag::N, true);
                self.registers.set_flag(Flag::H, (((self.registers[Register::A] & 0xF).wrapping_sub(state.b8 & 0xF).wrapping_sub(c)) & 0x10) == 0x10);
                self.registers.set_flag(Flag::C, (self.registers[Register::A] as u16) < (state.b8 as u16).wrapping_add(c as u16));
                self.registers[Register::A] = self.registers[Register::A].wrapping_sub(state.b8.wrapping_add(c));
            },
            MicroInstr::SBCHL => {
                let c = self.registers.get_flag(Flag::C);
                self.registers.set_flag(Flag::Z, self.registers[Register::A].wrapping_sub(self.bus.read(self.registers.get_hl()).wrapping_add(c)) == 0);
                self.registers.set_flag(Flag::N, true);
                self.registers.set_flag(Flag::H, (((self.registers[Register::A] & 0xF).wrapping_sub(self.bus.read(self.registers.get_hl()) & 0xF).wrapping_sub(c)) & 0x10) == 0x10);
                self.registers.set_flag(Flag::C, (self.registers[Register::A] as u16) < (self.bus.read(self.registers.get_hl()) as u16).wrapping_add(c as u16));
                self.registers[Register::A] = self.registers[Register::A].wrapping_sub(self.bus.read(self.registers.get_hl()).wrapping_add(c));
            },
            MicroInstr::DECBC => self.registers.set_bc(self.registers.get_bc().wrapping_sub(1)),
            MicroInstr::DECDE => self.registers.set_de(self.registers.get_de().wrapping_sub(1)),
            MicroInstr::DECHL => self.registers.set_hl(self.registers.get_hl().wrapping_sub(1)),
            MicroInstr::LDNNSP(byte) => {
                match byte {
                    Byte::LSB => self.bus.write(((state.b16 as u16) << 8) | (state.b8 as u16), (self.sp & 0x00FF) as u8),
                    Byte::MSB => self.bus.write((((state.b16 as u16) << 8) | (state.b8 as u16)) + 1, ((self.sp & 0xFF00) >> 8) as u8),
                }
            },
            MicroInstr::CPL => {
                self.registers.set_flag(Flag::N, true);
                self.registers.set_flag(Flag::H, true);
                self.registers[Register::A] = !self.registers[Register::A]
            },
            MicroInstr::LDSPHL => self.sp = self.registers.get_hl(),
            MicroInstr::RETI => {
                self.pc = ((state.b16 as u16) << 8) | (state.b8 as u16);
                self.should_enable_ime = 2;
            },
            MicroInstr::RST(addr) => self.pc = addr,
            MicroInstr::INCSP => self.sp = self.sp.wrapping_add(1),
            MicroInstr::DECSP => self.sp = self.sp.wrapping_sub(1),
            MicroInstr::ADDSPN => {
                self.registers.set_flag(Flag::Z, false);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, (self.sp & 0x0F).wrapping_add_signed((state.b8 as i8 as i16) & 0x0F) > 0x0F);
                self.registers.set_flag(Flag::C, (self.sp & 0xFF).wrapping_add_signed((state.b8 as i8 as i16) & 0xFF) > 0xFF);
                self.sp = self.sp.wrapping_add_signed(state.b8 as i8 as i16);
            },
            MicroInstr::LDHLSPN => {
                self.registers.set_flag(Flag::Z, false);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, (((self.sp & 0x0F) as u8) + (state.b8 & 0x0F)) > 0x0F);
                self.registers.set_flag(Flag::C, ((self.sp & 0xFF) + (state.b8 as u16)) > 0xFF);
                self.registers.set_hl(self.sp.wrapping_add_signed(state.b8 as i8 as i16));
            },
            MicroInstr::DAA => {
                let mut corr = 0;

                if (self.registers.get_flag(Flag::H) == 1) || ((self.registers.get_flag(Flag::N) == 0) && ((self.registers[Register::A] & 0xF) > 9)) {
                    corr |= 0x6
                }

                if (self.registers.get_flag(Flag::C) == 1) || ((self.registers.get_flag(Flag::N) == 0) && (self.registers[Register::A] > 0x99)) {
                    corr |= 0x60;
                    self.registers.set_flag(Flag::C, true);
                }

                if self.registers.get_flag(Flag::N) == 1 {
                    self.registers[Register::A] = self.registers[Register::A].wrapping_sub(corr);
                } else {
                    self.registers[Register::A] = self.registers[Register::A].wrapping_add(corr);
                }

                self.registers.set_flag(Flag::Z, self.registers[Register::A] == 0);
                self.registers.set_flag(Flag::H, false);
            },
            MicroInstr::LDAHLDEC => {
                self.registers[Register::A] = self.bus.read(self.registers.get_hl());
                self.registers.set_hl(self.registers.get_hl() - 1);
            },
            MicroInstr::CPHL => {
                self.registers.set_flag(Flag::Z, self.registers[Register::A].wrapping_sub(self.bus.read(self.registers.get_hl())) == 0);
                self.registers.set_flag(Flag::N, true);
                self.registers.set_flag(Flag::H, (((self.registers[Register::A] & 0xF).wrapping_sub(self.bus.read(self.registers.get_hl()) & 0xF)) & 0x10) == 0x10);
                self.registers.set_flag(Flag::C, self.registers[Register::A] < self.bus.read(self.registers.get_hl()));
            },
            MicroInstr::ADCHL => {
                let c = self.registers.get_flag(Flag::C);
                self.registers.set_flag(Flag::Z, self.registers[Register::A].wrapping_add(self.bus.read(self.registers.get_hl())).wrapping_add(c) == 0);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, (((self.registers[Register::A] & 0xF).wrapping_add(self.bus.read(self.registers.get_hl()) & 0xF).wrapping_add(c)) & 0x10) == 0x10);
                self.registers.set_flag(Flag::C, ((self.registers[Register::A] as u16).wrapping_add(self.bus.read(self.registers.get_hl()) as u16).wrapping_add(c as u16)) > 0xFF);
                self.registers[Register::A] = self.registers[Register::A].wrapping_add(self.bus.read(self.registers.get_hl())).wrapping_add(c);
            },
            MicroInstr::SUBHL => {
                self.registers.set_flag(Flag::Z, self.registers[Register::A].wrapping_sub(self.bus.read(self.registers.get_hl())) == 0);
                self.registers.set_flag(Flag::N, true);
                self.registers.set_flag(Flag::H, (((self.registers[Register::A] & 0xF).wrapping_sub(self.bus.read(self.registers.get_hl()) & 0xF)) & 0x10) == 0x10);
                self.registers.set_flag(Flag::C, self.registers[Register::A] < self.bus.read(self.registers.get_hl()));
                self.registers[Register::A] = self.registers[Register::A].wrapping_sub(self.bus.read(self.registers.get_hl()));
            },
            MicroInstr::INCHLADDR => {
                self.registers.set_flag(Flag::Z, self.bus.read(self.registers.get_hl()).wrapping_add(1) == 0);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, (((self.bus.read(self.registers.get_hl()) & 0xF).wrapping_add(1 & 0xF)) & 0x10) == 0x10);
                self.bus.write(self.registers.get_hl(), self.bus.read(self.registers.get_hl()).wrapping_add(1));
            },
            MicroInstr::RLCR(register) => {
                let t = (self.registers[register] >> 7) & 0x1;
                if t == 1 {
                    self.registers[register] = (self.registers[register] << 1) | 0b00000001
                } else {
                    self.registers[register] = (self.registers[register] << 1) & 0b11111110
                }
                self.registers.set_flag(Flag::C, t == 1);
                self.registers.set_flag(Flag::Z, self.registers[register] == 0);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, false);
            },
            MicroInstr::RLCHL => {
                let t = (self.bus.read(self.registers.get_hl()) >> 7) & 0x1;
                if t == 1 {
                    self.bus.write(self.registers.get_hl(), (self.bus.read(self.registers.get_hl()) << 1) | 0b00000001);
                } else {
                    self.bus.write(self.registers.get_hl(), (self.bus.read(self.registers.get_hl()) << 1) & 0b11111110);
                }
                self.registers.set_flag(Flag::C, t == 1);
                self.registers.set_flag(Flag::Z, self.bus.read(self.registers.get_hl()) == 0);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, false);
            },
            MicroInstr::RRCR(register) => {
                let t = self.registers[register] & 0x1;
                if t == 1 {
                    self.registers[register] = self.registers[register] >> 1 | 0b10000000;
                } else {
                    self.registers[register] = self.registers[register] >> 1 & 0b01111111;
                }
                self.registers.set_flag(Flag::C, t == 1);
                self.registers.set_flag(Flag::Z, self.registers[register] == 0);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, false);
            },
            MicroInstr::RRCA => {
                let t = self.registers[Register::A] & 0x1;
                if t == 1 {
                    self.registers[Register::A] = self.registers[Register::A] >> 1 | 0b10000000;
                } else {
                    self.registers[Register::A] = self.registers[Register::A] >> 1 & 0b01111111;
                }
                self.registers.set_flag(Flag::C, t == 1);
                self.registers.set_flag(Flag::Z, false);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, false);
            },
            MicroInstr::RRCHL => {
                let t = self.bus.read(self.registers.get_hl()) & 0x1;
                if t == 1 {
                    self.bus.write(self.registers.get_hl(), (self.bus.read(self.registers.get_hl()) >> 1) | 0b10000000);
                } else {
                    self.bus.write(self.registers.get_hl(), (self.bus.read(self.registers.get_hl()) >> 1) & 0b01111111);
                }
                self.registers.set_flag(Flag::C, t == 1);
                self.registers.set_flag(Flag::Z, self.bus.read(self.registers.get_hl()) == 0);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, false);
            },
            MicroInstr::RLHL => {
                let c = self.registers.get_flag(Flag::C);
                self.registers.set_flag(Flag::C, (self.bus.read(self.registers.get_hl()) >> 7) & 0x1 == 0x1);
                if c == 1 {
                    self.bus.write(self.registers.get_hl(), (self.bus.read(self.registers.get_hl()) << 1) | 0b00000001);
                } else {
                    self.bus.write(self.registers.get_hl(), (self.bus.read(self.registers.get_hl()) << 1) & 0b11111110);
                }
                self.registers.set_flag(Flag::Z, self.bus.read(self.registers.get_hl()) == 0);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, false);
            },
            MicroInstr::RLA => {
                let c = self.registers.get_flag(Flag::C);
                self.registers.set_flag(Flag::C, (self.registers[Register::A] >> 7) & 0x1 == 0x1);
                if c == 1 {
                    self.registers[Register::A] = (self.registers[Register::A] << 1) | 0b00000001;
                } else {
                    self.registers[Register::A] = (self.registers[Register::A] << 1) & 0b11111110;
                }
                self.registers.set_flag(Flag::Z, false);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, false);
            },
            MicroInstr::RLR(register) => {
                let c = self.registers.get_flag(Flag::C);
                self.registers.set_flag(Flag::C, (self.registers[register] >> 7) & 0x1 == 0x1);
                if c == 1 {
                    self.registers[register] = (self.registers[register] << 1) | 0b00000001;
                } else {
                    self.registers[register] = (self.registers[register] << 1) & 0b11111110;
                }
                self.registers.set_flag(Flag::Z, self.registers[register] == 0);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, false);
            },
            MicroInstr::SLAHL => {
                self.registers.set_flag(Flag::C, (self.bus.read(self.registers.get_hl()) >> 7) & 0x1 == 0x1);
                self.bus.write(self.registers.get_hl(), self.bus.read(self.registers.get_hl()) << 1);
                self.registers.set_flag(Flag::Z, self.bus.read(self.registers.get_hl()) == 0);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, false);
            },
            MicroInstr::SLAR(register) => {
                self.registers.set_flag(Flag::C, (self.registers[register] >> 7) & 0x1 == 0x1);
                self.registers[register] <<= 1;
                self.registers.set_flag(Flag::Z, self.registers[register] == 0);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, false);
            },
            MicroInstr::SRAHL => {
                self.registers.set_flag(Flag::C, self.bus.read(self.registers.get_hl()) & 0x1 == 0x1);
                let t = self.bus.read(self.registers.get_hl()) >> 7 & 0x1;
                if t == 1 {
                    self.bus.write(self.registers.get_hl(), (self.bus.read(self.registers.get_hl()) >> 1) | 0b10000000);
                } else {
                    self.bus.write(self.registers.get_hl(), (self.bus.read(self.registers.get_hl()) >> 1) & 0b01111111);
                }
                self.registers.set_flag(Flag::Z, self.bus.read(self.registers.get_hl()) == 0);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, false);
            },
            MicroInstr::SRAR(register) => {
                self.registers.set_flag(Flag::C, self.registers[register] & 0x1 == 0x1);
                let t = self.registers[register] >> 7 & 0x1;
                if t == 1 {
                    self.registers[register] = (self.registers[register] >> 1) | 0b10000000;
                } else {
                    self.registers[register] = (self.registers[register] >> 1) & 0b01111111;
                }
                self.registers.set_flag(Flag::Z, self.registers[register] == 0);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, false);
            },
            MicroInstr::SWAP(register) => {
                self.registers[register] = ((self.registers[register] & 0x0F) << 4) | ((self.registers[register] & 0xF0) >> 4);
                self.registers.set_flag(Flag::Z, self.registers[register] == 0);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, false);
                self.registers.set_flag(Flag::C, false);
            },
            MicroInstr::SWAPHL => {
                self.bus.write(self.registers.get_hl(), ((self.bus.read(self.registers.get_hl()) & 0x0F) << 4) | ((self.bus.read(self.registers.get_hl()) & 0xF0) >> 4));
                self.registers.set_flag(Flag::Z, self.bus.read(self.registers.get_hl()) == 0);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, false);
                self.registers.set_flag(Flag::C, false);
            },
            MicroInstr::BIT(pos, register) => {
                self.registers.set_flag(Flag::Z, !(self.registers[register] >> pos) & 0x1 == 0x1);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, true);
            },
            MicroInstr::BITHL(pos) => {
                self.registers.set_flag(Flag::Z, !((self.bus.read(self.registers.get_hl()) >> pos) & 0x1 == 0x1));
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, true);
            },
            MicroInstr::RLCA => {
                let t = (self.registers[Register::A] >> 7) & 0x1;
                self.registers[Register::A] <<= 1;
                if t == 1 {
                    self.registers[Register::A] |= 0b00000001;
                } else {
                    self.registers[Register::A] &= 0b11111110;
                }
                self.registers.set_flag(Flag::Z, false);
                self.registers.set_flag(Flag::N, false);
                self.registers.set_flag(Flag::H, false);
                self.registers.set_flag(Flag::C, t == 1);
            },
            MicroInstr::RES(pos, register) => self.registers[register] &= !(1 << pos),
            MicroInstr::RESHL(pos) => self.bus.write(self.registers.get_hl(), self.bus.read(self.registers.get_hl()) & !(1 << pos)),
            MicroInstr::SET(pos, register) => self.registers[register] |= 1 << pos,
            MicroInstr::SETHL(pos) => self.bus.write(self.registers.get_hl(), self.bus.read(self.registers.get_hl()) | 1 << pos),
            MicroInstr::EI => self.should_enable_ime = 2,
            MicroInstr::HALT => if !self.bus.flat_ram { self.is_halted = true },
            MicroInstr::STOP => unimplemented!("encountered STOP instruction.")
        }

        if !self.is_halted {
            state.step += 1;
        } else if (self.bus.IE & self.bus.IF) != 0 {
            self.is_halted = false;
            state.step += 1;
        }

        if state.step >= state.instr.len() {
            self.tick_state = None;

            if self.should_enable_ime > 0 { // starts at 2 to delay 1 instruction
                self.should_enable_ime -= 1;
                if self.should_enable_ime == 0 {
                    self.ime = true;
                }
            }
        };
    }

    fn execute_interrupt(&mut self) { // 5 cycles to complete
        let state = self.interrupt_tick_state.as_mut().unwrap();
        match state.step {
            0 => state.step += 1,
            1 => state.step += 1,
            2 => {
                self.sp -= 1;
                self.bus.write(self.sp, ((0xFF00 & self.pc) >> 8) as u8);
                state.step += 1;
            },
            3 => {
                self.sp -= 1;
                self.bus.write(self.sp, (0x00FF & self.pc) as u8);
                state.step += 1;
            },
            4 => {
                match state.interrupt {
                    Interrupt::VBLANK => self.pc = 0x0040,
                    Interrupt::STAT => self.pc = 0x0048,
                    Interrupt::TIMER => self.pc = 0x0050
                }
                self.interrupt_tick_state = None;
            }
            _ => unreachable!()
        }
    }

    pub fn next_frame(&mut self, keypress: i8) -> Display {
        self.bus.keypress = keypress;
        let mut cycles_to_timeout = 1000000; // TODO: Figure out that weird bug that crashes games from either interrupt or halt

        while !self.bus.is_frame_rendered() && cycles_to_timeout > 0 { // represents 1 M-Cycle
            if self.interrupt_tick_state.is_none() { self.execute() } else { self.execute_interrupt() } // either servicing interrupt or executing a normal instruction
            self.bus.update_components();
            self.bus.update_requested_interrupts();
            if self.ime && self.tick_state.is_none() { // if interrupts are enabled service potential interrupts
                if (self.bus.IE & self.bus.IF) != 0 { // an interrupt has been requested and can potentially be handled
                    for i in 0..3 { // handles interrupts based on their priority
                        if (self.bus.IF >> i) & 0x1 == 1 && (self.bus.IE >> i) & 0x1 == 1 { // interrupt has been requested and allowed by IE
                            match i {
                                0 => self.interrupt_tick_state.get_or_insert(InterruptTickState { interrupt: Interrupt::VBLANK, step: 0 }),
                                1 => self.interrupt_tick_state.get_or_insert(InterruptTickState { interrupt: Interrupt::STAT, step: 0 }),
                                2 => self.interrupt_tick_state.get_or_insert(InterruptTickState { interrupt: Interrupt::TIMER, step: 0 }),
                                _ => unimplemented!("interrupt not implemented yet.")
                            };
                            self.bus.IF &= !(1 << i); // reset the bit that has been requested while processing
                            self.ime = false; // disable interrupts to prevent anymore from being serviced while processing the current one
                            break
                        }
                    }
                }
            }
            cycles_to_timeout -= 1;
        }

        if cycles_to_timeout == 0 {
            panic!("halt bug loop.")
        }

        return self.bus.get_display();
    }

    fn create_block(&self, ident: &str, block: &[u8]) -> Vec<u8> {
        let mut bess_block = vec![];
        bess_block.extend_from_slice(ident.as_bytes());
        bess_block.extend_from_slice(&u32_to_little_endian(block.len() as u32));
        bess_block.extend_from_slice(block);
        bess_block
    }

    fn next_block(&self, bess_encoding: &Vec<u8>, ptr: &mut usize) -> (String, u32) {
        let name = std::str::from_utf8(&bess_encoding[*ptr..*ptr+4]).expect("invalid utf-8 sequence");
        *ptr += 4;

        let block_len = ((bess_encoding[*ptr + 3] as u32) << 24) | ((bess_encoding[*ptr + 2] as u32) << 16) | ((bess_encoding[*ptr + 1] as u32) << 8) | (bess_encoding[*ptr] as u32);
        *ptr += (block_len as usize) + 4;

        (String::from(name), block_len)
    }

    fn create_core_block(&mut self, major_bess_ver: [u8; 2], minor_bess_ver: [u8; 2], model_identifier: &str) -> Vec<u8> {
        let mut core = vec![];

        core.extend_from_slice(&major_bess_ver);
        core.extend_from_slice(&minor_bess_ver);
        core.extend_from_slice(model_identifier.as_bytes());

        let cpu_state: [u8; 16] = [(self.pc & 0x00FF) as u8, (self.pc >> 8) as u8, self.registers[Register::F], self.registers[Register::A], self.registers[Register::C], 
        self.registers[Register::B], self.registers[Register::E], self.registers[Register::D], self.registers[Register::L], 
        self.registers[Register::H], (self.sp & 0x00FF) as u8, (self.sp >> 8) as u8, self.ime as u8, self.bus.IE, self.is_halted as u8, 0x00];
        core.extend_from_slice(&cpu_state);

        let mut mem_mapped_registers: Vec<u8> = vec![];
        for register in 0xFF00..=0xFF7F {
            mem_mapped_registers.push(self.bus.read(register));
        }

        core.extend(mem_mapped_registers);
        core.append(&mut self.bus.bess_buffer_offsets); // appends then clears offsets created from copying large buffers at the beginning of the file ( Memory::aggregate_buffers() )

        core
    }

    pub fn create_save_file(&mut self) -> Vec<u8> {
        let mut bess_encoding = vec![];

        // populates the bess_buffer_offsets vector
        let large_buffers = self.bus.aggregate_buffers();
        bess_encoding.extend(&large_buffers);

        bess_encoding.extend(self.create_block("NAME", "emufun-gb".as_bytes()));
        bess_encoding.extend(self.create_block("INFO", &self.bus.get_rom_info()));

        let core_block = self.create_core_block([0x01, 0x00], [0x01, 0x00], "GD  ");
        bess_encoding.extend(self.create_block("CORE", &core_block));

        let mbc_block = self.bus.create_bess_mbc_block();
        if !mbc_block.is_none() {
            bess_encoding.extend(self.create_block("MBC ", &mbc_block.unwrap()))
        }

        bess_encoding.extend(self.create_block("END ", &[]));

        bess_encoding.extend_from_slice(&u32_to_little_endian(large_buffers.len() as u32));
        bess_encoding.extend_from_slice("BESS".as_bytes());

        bess_encoding
    }

    pub fn load_save_file(&mut self, file: Vec<u8>) {
        if file[(file.len() - 4)..] != *("BESS".as_bytes()) {
            panic!("Invalid save file.")
        }

        // acts as starting index for the first bess block
        let mut file_ptr: usize = (((file[file.len() - 5] as u64) << 32) |  ((file[file.len() - 6] as u64) << 16) | ((file[file.len() - 7] as u64) << 8) | (file[file.len() - 8] as u64)) as usize;

        loop {
            let bess_block = self.next_block(&file,  &mut file_ptr);

            match bess_block.0.as_str() {
                "NAME" => (), // ignore for now
                "INFO" => (), // ignore for now
                "CORE" => {
                    let chunk = &file[(file_ptr - (bess_block.1 as usize))..file_ptr];

                    self.pc = ((chunk[0x09] as u16) << 8) | (chunk[0x08] as u16); 
                    self.registers[Register::F] = chunk[0x0A];
                    self.registers[Register::A] = chunk[0x0B];
                    self.registers[Register::C] = chunk[0x0C];
                    self.registers[Register::B] = chunk[0x0D];
                    self.registers[Register::E] = chunk[0x0E];
                    self.registers[Register::D] = chunk[0x0F];
                    self.registers[Register::L] = chunk[0x10];
                    self.registers[Register::H] = chunk[0x11];
                    self.sp = ((chunk[0x13] as u16) << 8) | (chunk[0x12] as u16);
                    self.ime = chunk[0x14] == 1;
                    self.bus.IE = chunk[0x15]; 
                    self.is_halted = chunk[0x16] == 1;

                    for i in 0..0x80 {
                        let addr = 0xFF00 + i;
                        let val = chunk[(0x18 + i) as usize];

                        match addr {
                            0xFF04 => self.bus.timer.sysclock = (val as u16) << 8,
                            0xFF46 => (),
                            _ => self.bus.write(addr, val) // ignore don't care values ??
                        }
                    }

                    let wram_size = ((chunk[0x9B] as u32) << 24) | ((chunk[0x9A] as u32) << 16) | ((chunk[0x99] as u32) << 8) | (chunk[0x98] as u32);
                    let wram_offset = ((chunk[0x9F] as u32) << 24) | ((chunk[0x9E] as u32) << 16) | ((chunk[0x9D] as u32) << 8) | (chunk[0x9C] as u32);
                    for i in 0..wram_size {
                        self.bus.write((0xC000 + i) as u16, file[(wram_offset + i) as usize]);
                    }

                    let vram_size = ((chunk[0xA3] as u32) << 24) | ((chunk[0xA2] as u32) << 16) | ((chunk[0xA1] as u32) << 8) | (chunk[0xA0] as u32);
                    let vram_offset = ((chunk[0xA7] as u32) << 24) | ((chunk[0xA6] as u32) << 16) | ((chunk[0xA5] as u32) << 8) | (chunk[0xA4] as u32);
                    for i in 0..vram_size {
                        self.bus.write((0x8000 + i) as u16, file[(vram_offset + i) as usize]);
                    }

                    let sram_size = ((chunk[0xAB] as u32) << 24) | ((chunk[0xAA] as u32) << 16) | ((chunk[0xA9] as u32) << 8) | (chunk[0xA8] as u32);
                    let sram_offset = ((chunk[0xAF] as u32) << 24) | ((chunk[0xAE] as u32) << 16) | ((chunk[0xAD] as u32) << 8) | (chunk[0xAC] as u32);
                    for i in 0..sram_size {
                        self.bus.sram[i as usize] = file[(sram_offset + i) as usize];
                    }

                    let oam_size = ((chunk[0xB3] as u32) << 24) | ((chunk[0xB2] as u32) << 16) | ((chunk[0xB1] as u32) << 8) | (chunk[0xB0] as u32);
                    let oam_offset = ((chunk[0xB7] as u32) << 24) | ((chunk[0xB6] as u32) << 16) | ((chunk[0xB5] as u32) << 8) | (chunk[0xB4] as u32);
                    for i in 0..oam_size {
                        self.bus.write((0xFE00 + i) as u16, file[(oam_offset + i) as usize]);
                    }

                    let hram_size = ((chunk[0xBB] as u32) << 24) | ((chunk[0xBA] as u32) << 16) | ((chunk[0xB9] as u32) << 8) | (chunk[0xB8] as u32);
                    let hram_offset = ((chunk[0xBF] as u32) << 24) | ((chunk[0xBE] as u32) << 16) | ((chunk[0xBD] as u32) << 8) | (chunk[0xBC] as u32);
                    for i in 0..hram_size {
                        self.bus.write((0xFF80 + i) as u16, file[(hram_offset + i) as usize]);
                    }
                },
                "MBC " => {
                    let chunk = &file[(file_ptr - (bess_block.1 as usize))..file_ptr];

                    if chunk.len() % 3 != 0 {
                        panic!("MBC bess block must be divisible by 3.")
                    }

                    for i in 0..(chunk.len() / 3) {
                        let offset = i * 3;
                        let addr = ((chunk[offset + 1] as u16) << 8) | (chunk[offset + 0x00] as u16);

                        if addr <= 0x7FFF || (addr >= 0xA000 || addr <= 0xBFFF) {
                            self.bus.write(addr, chunk[offset + 2]);
                        } else {
                            panic!("recieved invalid address for MBC register.");
                        }
                    }
                },
                "XOAM" => (), 
                "END " => break,
                _ => unimplemented!("Block not handled yet: ({})", bess_block.0)
            }
        }

        // reset CPU state and any requested interrupts
        self.tick_state = None;
        self.interrupt_tick_state = None;
        self.bus.IF = 0x00;
    }

    // manually sets registers to skip the boot rom
    pub fn initialize_core(&mut self) {
        self.registers[Register::A] = 0x01;
        self.registers[Register::F] = 0xB0;
        self.registers[Register::B] = 0x00;
        self.registers[Register::C] = 0x13;
        self.registers[Register::D] = 0x00;
        self.registers[Register::E] = 0xD8;
        self.registers[Register::H] = 0x01;
        self.registers[Register::L] = 0x4D;
        self.sp = 0xFFFE;
        self.pc = 0x0100;

       self.bus.write(0xFF40, 0x80);
       self.bus.write(0xFF44, 0x91);
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            bus: Memory::default(),
            registers: Registers::default(),
            sp: 0x0,
            pc: 0x0,
            tick_state: None,
            ime: false,
            should_enable_ime: 0,
            interrupt_tick_state: None,
            is_halted: false,
            halt_bug: false
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;
    use pretty_assertions::assert_eq;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct JsmooTestObject {
        name: String,
        initial: JsmooTestObjectState,
        r#final: JsmooTestObjectState,
        cycles: Vec<(u16, Option<u16>, String)>
    }

    #[derive(Debug, Deserialize)]
    struct JsmooTestObjectState {
        a: u8,
        b: u8,
        c: u8,
        d: u8,
        e: u8,
        f: u8,
        h: u8,
        l: u8,
        sp: u16,
        pc: u16,
        ram: Vec<[usize; 2]>
    }

    #[test]
    fn jsmoo_sm83_cpu_tests() {
        let files = fs::read_dir("./tests/jsmoo").unwrap();

        for file in files {
            let file_os_string = file.as_ref().unwrap().file_name();
            let file_string_split = file_os_string.to_str().unwrap().split(".");
            let file_parts: Vec<_> = file_string_split.collect();
        
            let body = fs::read_to_string(String::from("tests/jsmoo/") + file_parts[0] + ".json").expect("File not found!");
        
            let mut lines_passed = 0;

            let json_tests: Vec<JsmooTestObject> = serde_json::from_str(&body).expect("JSON was not well-formatted");
            for test in json_tests {
                let mut cpu = CPU::default();
                cpu.bus.flat_ram = true;

                cpu.registers[Register::A] = test.initial.a;
                cpu.registers[Register::B] = test.initial.b;
                cpu.registers[Register::C] = test.initial.c;
                cpu.registers[Register::D] = test.initial.d;
                cpu.registers[Register::E] = test.initial.e;
                cpu.registers[Register::F] = test.initial.f;
                cpu.registers[Register::H] = test.initial.h;
                cpu.registers[Register::L] = test.initial.l;

                cpu.pc = test.initial.pc;
                cpu.sp = test.initial.sp;

                for loc in test.initial.ram {
                    cpu.bus.write(loc[0] as u16, loc[1] as u8);
                }

                let mut prefixed = false;
                let mut opcode_str = file_parts[0];
                if opcode_str.len() == 5 {
                    let parts = opcode_str.split(" ");
                    let parts_vec: Vec<_> = parts.collect();
                    opcode_str = parts_vec[1];
                    prefixed = true;
                }

                let opcode = u8::from_str_radix(opcode_str, 16);
                let opcode_num = opcode.unwrap();
                let steps = if prefixed {
                    cpu.fetch_instr();
                    cpu.fetch_prefix_instr();
                    cpu.decode_prefix_instr(opcode_num)
                } else { 
                    cpu.fetch_instr();
                    cpu.decode_instr(opcode_num)
                };

                cpu.tick_state.get_or_insert(TickState{
                    instr: steps.clone(),
                    step: 0,
                    is_prefix: false,
                    b8: 0,
                    b16: 0
                });

                let mut steps = if prefixed { 1 } else { 0 };
                while !cpu.tick_state.is_none() {
                    steps += 1;
                    cpu.execute();
                }

                if opcode_num != 0x76 && opcode_num != 0x10 {
                    assert_eq!(steps, test.cycles.len(), "Failed instruction {}", test.name);
                }

                for mem in test.r#final.ram {
                    assert_eq!(cpu.bus.read(mem[0] as u16), mem[1] as u8, "Failed instruction {}", test.name);
                }

                let expected_state = format!("A: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} F: {:02X} H: {:02X} L: {:02X} PC: {:04X} SP: {:04X}",
                test.r#final.a, test.r#final.b, test.r#final.c, test.r#final.d, test.r#final.e, test.r#final.f, 
                test.r#final.h, test.r#final.l, test.r#final.pc, test.r#final.sp);

                let recieved_state = format!("A: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} F: {:02X} H: {:02X} L: {:02X} PC: {:04X} SP: {:04X}",
                cpu.registers[Register::A], cpu.registers[Register::B], cpu.registers[Register::C], cpu.registers[Register::D], cpu.registers[Register::E], cpu.registers[Register::F], 
                cpu.registers[Register::H], cpu.registers[Register::L], cpu.pc, cpu.sp);

                assert_eq!(recieved_state, expected_state, "Failed instruction {}", test.name);
                
                lines_passed += 1;
            }
        }
    }

    // #[test] WILL I EVER PASS THIS T_T
    // fn blargg_cpu_instr_tests() {
    //     let files = fs::read_dir("./tests/blargg/roms").unwrap();

    //     for file in files {
    //         let file_os_string = file.as_ref().unwrap().file_name();
    //         let file_string_split = file_os_string.to_str().unwrap().split(".");
    //         let file_parts: Vec<_> = file_string_split.collect();
            
    //         let cartridge = fs::read(String::from("./tests/blargg/roms/") + file.as_ref().unwrap().file_name().to_str().unwrap()).expect("File not found!");

    //         let mut core = CPU::default();
    //         core.bus.debug = true; // return 0x90 for LY register
    //         core.bus.load_cartridge(cartridge);

    //         core.pc = 0x101;
    //         core.sp = 0xFFFE;
            
    //         core.registers[Register::A] = 0x01;
    //         core.registers[Register::F] = 0xB0;
    //         core.registers[Register::B] = 0x00;
    //         core.registers[Register::C] = 0x13;
    //         core.registers[Register::D] = 0x00;
    //         core.registers[Register::E] = 0xD8;
    //         core.registers[Register::H] = 0x01;
    //         core.registers[Register::L] = 0x4D;

    //         let body = fs::read_to_string(String::from("./tests/blargg/logs/") + file_parts[0] + ".txt").expect("File not found!");

    //         for line in body.lines() {
    //             let core_state = format!(
    //                 "A: {:02X} F: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} SP: {:04X} PC: 00:{:04X} ({:02X} {:02X} {:02X} {:02X})",
    //                 core.registers[Register::A], core.registers[Register::F], core.registers[Register::B], core.registers[Register::C], core.registers[Register::D], 
    //                 core.registers[Register::E], core.registers[Register::H], core.registers[Register::L], core.sp, core.pc, 
    //                 core.bus.read(core.pc), core.bus.read(core.pc+1), core.bus.read(core.pc+2), core.bus.read(core.pc+3)
    //             );

    //             assert_eq!(core_state, line);

    //             core.next_frame(1, -1);
    //             while !core.tick_state.is_none() || !core.interrupt_tick_state.is_none() {
    //                 core.next_frame(1, -1);
    //             }
    //         }
    //     }
    // }
}