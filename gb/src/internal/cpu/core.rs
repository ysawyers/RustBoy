use crate::internal::cpu::registers::Registers;

pub struct CPU {
    registers: Registers,
    pc: u16,
    sp: u16
}

struct StepState {
    current_opcode: Opcode,
}

struct Opcode {}

impl CPU {
    fn fetch(&self) {}

    pub fn step(&self) {}
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            registers: Registers::default(),
            sp: 0x0,
            pc: 0x0
        }
    }
}