use crate::CPU;
use crate::internal::core::component::MicroInstr;

impl CPU {
    pub fn decode_prefix_instr(&self, opcode: u8) -> Vec<MicroInstr> {
        match opcode {
            _ => panic!("Instruction not implemented: CB -> 0x{:02X}", opcode)
        }
    }
}