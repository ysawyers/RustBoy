use crate::CPU;
use crate::internal::core::component::{MicroInstr, Byte};
use crate::internal::core::registers::{Register, Flag};

impl CPU {
    pub fn decode_instr(&self, opcode: u8) -> Vec<MicroInstr> {
        match opcode {
            0x00 => vec![MicroInstr::NOP], // NOP
            
            0x0E => vec![MicroInstr::NOP, MicroInstr::LDRN(Register::C)], // LD C,u8
            0x11 => vec![MicroInstr::NOP, MicroInstr::LDRN(Register::E), MicroInstr::LDRN(Register::D)], // LD DE,u16
            0x21 => vec![MicroInstr::NOP, MicroInstr::LDRN(Register::L), MicroInstr::LDRN(Register::H)], // LD HL,u16
            0x01 => vec![MicroInstr::NOP, MicroInstr::LDRN(Register::C), MicroInstr::LDRN(Register::B)], // LD BC,u16
            0x47 => vec![MicroInstr::LDRR(Register::B, Register::A)], // LD B,A
            0x78 => vec![MicroInstr::LDRR(Register::A, Register::B)], // LD A,B
            0x7D => vec![MicroInstr::LDRR(Register::A, Register::L)], // LD A,L
            0x7C => vec![MicroInstr::LDRR(Register::A, Register::H)], // LD A,H
            0x2A => vec![MicroInstr::NOP, MicroInstr::LDAHLINC], // LD A,(HL+)
            0x12 => vec![MicroInstr::NOP, MicroInstr::LDNNA(self.registers.get_de(), false)], // LD (DE),A
            0x31 => vec![MicroInstr::Read(Byte::LSB), MicroInstr::Read(Byte::MSB), MicroInstr::LDSPNN], // LD SP,u16
            0xEA => vec![MicroInstr::NOP, MicroInstr::Read(Byte::LSB), MicroInstr::Read(Byte::MSB), MicroInstr::LDNNA(0, false)], // LD (u16),A
            0x3E => vec![MicroInstr::NOP, MicroInstr::LDRN(Register::A)], // LD A,u8
            0xE0 => vec![MicroInstr::NOP, MicroInstr::Read(Byte::LSB), MicroInstr::LDNNA(0xFF00, true)], // LD (FF00+u8),A
            0xF0 => vec![MicroInstr::NOP, MicroInstr::Read(Byte::LSB), MicroInstr::LDANN(0xFF00, true)], // LD A,(FF00+u8)
            0xFA => vec![MicroInstr::NOP, MicroInstr::Read(Byte::LSB), MicroInstr::Read(Byte::MSB), MicroInstr::LDANN(0, false)], // LD A,(u16)
            
            0x18 => vec![MicroInstr::Read(Byte::LSB), MicroInstr::NOP, MicroInstr::JR], // JR i8
            0x20 => vec![MicroInstr::Read(Byte::LSB), MicroInstr::Cond(Flag::Z, false), MicroInstr::JR], // JR NZ,i8
            0x28 => vec![MicroInstr::Read(Byte::LSB), MicroInstr::Cond(Flag::Z, true), MicroInstr::JR], // JR Z,i8
            0xC3 => vec![MicroInstr::Read(Byte::LSB), MicroInstr::Read(Byte::MSB), MicroInstr::NOP, MicroInstr::JP], // JP u16
            0xCD => vec![MicroInstr::NOP, MicroInstr::Read(Byte::LSB), MicroInstr::Read(Byte::MSB), MicroInstr::PUSH(((0xFF00 & (self.pc + 2)) >> 8) as u8), MicroInstr::PUSH((0x00FF & (self.pc + 2)) as u8), MicroInstr::JP], // CALL u16
            0xC4 => vec![MicroInstr::Read(Byte::LSB), MicroInstr::Read(Byte::MSB), MicroInstr::Cond(Flag::Z, false), MicroInstr::PUSH(((0xFF00 & (self.pc + 2)) >> 8) as u8), MicroInstr::PUSH((0x00FF & (self.pc + 2)) as u8), MicroInstr::JP], // CALL NZ,u16
            0xC9 => vec![MicroInstr::NOP, MicroInstr::POPPC(Byte::LSB), MicroInstr::POPPC(Byte::MSB), MicroInstr::JP], // RET

            0x23 => vec![MicroInstr::NOP, MicroInstr::INCHL], // INC HL
            0x03 => vec![MicroInstr::NOP, MicroInstr::INCBC], // INC BC
            0x1C => vec![MicroInstr::INCR(Register::E)], // INC E
            0x14 => vec![MicroInstr::INCR(Register::D)], // INC D
            0x0D => vec![MicroInstr::DECR(Register::C)], // DEC C

            0xB1 => vec![MicroInstr::ORRR(Register::A, Register::C)], // OR A,C

            0xE6 => vec![MicroInstr::Read(Byte::LSB), MicroInstr::ANDRN(Register::A)], // AND A,u8

            0xFE => vec![MicroInstr::Read(Byte::LSB), MicroInstr::CPRN(Register::A)], // CP A,u8

            0xF5 => vec![MicroInstr::NOP, MicroInstr::NOP, MicroInstr::PUSH(self.registers[Register::A]), MicroInstr::PUSH(self.registers[Register::F])], // PUSH AF
            0xE5 => vec![MicroInstr::NOP, MicroInstr::NOP, MicroInstr::PUSH(self.registers[Register::H]), MicroInstr::PUSH(self.registers[Register::L])], // PUSH HL
            0xC5 => vec![MicroInstr::NOP, MicroInstr::NOP, MicroInstr::PUSH(self.registers[Register::B]), MicroInstr::PUSH(self.registers[Register::C])], // PUSH BC
            0xE1 => vec![MicroInstr::NOP, MicroInstr::POPR(Register::L), MicroInstr::POPR(Register::H)], // POP HL
            0xF1 => vec![MicroInstr::NOP, MicroInstr::POPR(Register::F), MicroInstr::POPR(Register::A)], // POP AF
            0xC1 => vec![MicroInstr::NOP, MicroInstr::POPR(Register::C), MicroInstr::POPR(Register::B)], // POP BC

            0xF3 => vec![MicroInstr::DI], // DI

            _ => panic!("Instruction not implemented: 0x{:02X}", opcode)
        }
    }
}