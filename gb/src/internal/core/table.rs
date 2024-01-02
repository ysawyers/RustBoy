use crate::{CPU};
use crate::{console_log, log};
use crate::internal::core::component::{MicroInstr, Byte};
use crate::internal::core::registers::{Register, Flag};

impl CPU {
    pub fn decode_instr(&self, opcode: u8) -> Vec<MicroInstr> {
        match opcode {
            0x26 => vec![MicroInstr::NOP, MicroInstr::LDRN(Register::H)], // LD H,u8
            0x0E => vec![MicroInstr::NOP, MicroInstr::LDRN(Register::C)], // LD C,u8
            0x06 => vec![MicroInstr::NOP, MicroInstr::LDRN(Register::B)], // LD B,u8
            0x2E => vec![MicroInstr::NOP, MicroInstr::LDRN(Register::L)], // LD L,u8
            0x16 => vec![MicroInstr::NOP, MicroInstr::LDRN(Register::D)], // LD D,u8
            0x1E => vec![MicroInstr::NOP, MicroInstr::LDRN(Register::E)], // LD E,u8
            0x11 => vec![MicroInstr::NOP, MicroInstr::LDRN(Register::E), MicroInstr::LDRN(Register::D)], // LD DE,u16
            0x21 => vec![MicroInstr::NOP, MicroInstr::LDRN(Register::L), MicroInstr::LDRN(Register::H)], // LD HL,u16
            0x01 => vec![MicroInstr::NOP, MicroInstr::LDRN(Register::C), MicroInstr::LDRN(Register::B)], // LD BC,u16
            0x47 => vec![MicroInstr::LDRR(Register::B, Register::A)], // LD B,A
            0x78 => vec![MicroInstr::LDRR(Register::A, Register::B)], // LD A,B
            0x7D => vec![MicroInstr::LDRR(Register::A, Register::L)], // LD A,L
            0x7C => vec![MicroInstr::LDRR(Register::A, Register::H)], // LD A,H
            0x5F => vec![MicroInstr::LDRR(Register::E, Register::A)], // LD E,A
            0x79 => vec![MicroInstr::LDRR(Register::A, Register::C)], // LD A,C
            0x4F => vec![MicroInstr::LDRR(Register::C, Register::A)], // LD C,A
            0x7A => vec![MicroInstr::LDRR(Register::A, Register::D)], // LD A,D
            0x57 => vec![MicroInstr::LDRR(Register::D, Register::A)], // LD D,A
            0x7B => vec![MicroInstr::LDRR(Register::A, Register::E)], // LD A,E
            0x6F => vec![MicroInstr::LDRR(Register::L, Register::A)], // LD L,A
            0x5D => vec![MicroInstr::LDRR(Register::E, Register::L)], // LD E,L
            0x67 => vec![MicroInstr::LDRR(Register::H, Register::A)], // LD H,A
            0x40 => vec![MicroInstr::LDRR(Register::B, Register::B)], // LD B,B
            0x41 => vec![MicroInstr::LDRR(Register::B, Register::C)], // LD B,C
            0x42 => vec![MicroInstr::LDRR(Register::B, Register::D)], // LD B,D
            0x43 => vec![MicroInstr::LDRR(Register::B, Register::E)], // LD B,E
            0x44 => vec![MicroInstr::LDRR(Register::B, Register::H)], // LD B,H
            0x45 => vec![MicroInstr::LDRR(Register::B, Register::L)], // LD B,L
            0x48 => vec![MicroInstr::LDRR(Register::C, Register::B)], // LD C,B
            0x49 => vec![MicroInstr::LDRR(Register::C, Register::C)], // LD C,C
            0x4A => vec![MicroInstr::LDRR(Register::C, Register::D)], // LD C,D
            0x4B => vec![MicroInstr::LDRR(Register::C, Register::E)], // LD C,E
            0x4C => vec![MicroInstr::LDRR(Register::C, Register::H)], // LD C,H
            0x4D => vec![MicroInstr::LDRR(Register::C, Register::L)], // LD C,L
            0x50 => vec![MicroInstr::LDRR(Register::D, Register::B)], // LD D,B
            0x51 => vec![MicroInstr::LDRR(Register::D, Register::C)], // LD D,C
            0x52 => vec![MicroInstr::LDRR(Register::D, Register::D)], // LD D,D
            0x53 => vec![MicroInstr::LDRR(Register::D, Register::E)], // LD D,E
            0x54 => vec![MicroInstr::LDRR(Register::D, Register::H)], // LD D,H
            0x55 => vec![MicroInstr::LDRR(Register::D, Register::L)], // LD D,L
            0x58 => vec![MicroInstr::LDRR(Register::E, Register::B)], // LD E,B
            0x59 => vec![MicroInstr::LDRR(Register::E, Register::C)], // LD E,C
            0x5A => vec![MicroInstr::LDRR(Register::E, Register::D)], // LD E,D
            0x5B => vec![MicroInstr::LDRR(Register::E, Register::E)], // LD E,E
            0x5C => vec![MicroInstr::LDRR(Register::E, Register::H)], // LD E,H
            0x60 => vec![MicroInstr::LDRR(Register::H, Register::B)], // LD H,B
            0x61 => vec![MicroInstr::LDRR(Register::H, Register::C)], // LD H,C
            0x62 => vec![MicroInstr::LDRR(Register::H, Register::D)], // LD H,D
            0x63 => vec![MicroInstr::LDRR(Register::H, Register::E)], // LD H,E
            0x64 => vec![MicroInstr::LDRR(Register::H, Register::H)], // LD H,H
            0x65 => vec![MicroInstr::LDRR(Register::H, Register::L)], // LD H,L
            0x68 => vec![MicroInstr::LDRR(Register::L, Register::B)], // LD L,B
            0x69 => vec![MicroInstr::LDRR(Register::L, Register::C)], // LD L,C
            0x6A => vec![MicroInstr::LDRR(Register::L, Register::D)], // LD L,D
            0x6B => vec![MicroInstr::LDRR(Register::L, Register::E)], // LD L,E
            0x6C => vec![MicroInstr::LDRR(Register::L, Register::H)], // LD L,H
            0x6D => vec![MicroInstr::LDRR(Register::L, Register::L)], // LD L,L
            0x7F => vec![MicroInstr::LDRR(Register::L, Register::L)], // LD A,A
            0x2A => vec![MicroInstr::NOP, MicroInstr::LDAHLINC], // LD A,(HL+)
            0x22 => vec![MicroInstr::NOP, MicroInstr::LDHLINCA], // LD (HL+),A
            0x32 => vec![MicroInstr::NOP, MicroInstr::LDHLDECA], // LD (HL-),A
            0x3A => vec![MicroInstr::NOP, MicroInstr::LDAHLDEC], // LD A,(HL-)
            0x12 => vec![MicroInstr::NOP, MicroInstr::LDNNR(self.registers.get_de(), Register::A, false)], // LD (DE),A
            0x77 => vec![MicroInstr::NOP, MicroInstr::LDNNR(self.registers.get_hl(), Register::A, false)], // LD (HL),A
            0x72 => vec![MicroInstr::NOP, MicroInstr::LDNNR(self.registers.get_hl(), Register::D, false)], // LD (HL),D
            0x71 => vec![MicroInstr::NOP, MicroInstr::LDNNR(self.registers.get_hl(), Register::C, false)], // LD (HL),C
            0x70 => vec![MicroInstr::NOP, MicroInstr::LDNNR(self.registers.get_hl(), Register::B, false)], // LD (HL),B
            0x73 => vec![MicroInstr::NOP, MicroInstr::LDNNR(self.registers.get_hl(), Register::E, false)], // LD (HL),E
            0x74 => vec![MicroInstr::NOP, MicroInstr::LDNNR(self.registers.get_hl(), Register::H, false)], // LD (HL),H
            0x75 => vec![MicroInstr::NOP, MicroInstr::LDNNR(self.registers.get_hl(), Register::L, false)], // LD (HL),L
            0x02 => vec![MicroInstr::NOP, MicroInstr::LDNNR(self.registers.get_bc(), Register::A, false)], // LD (BC),A
            0x36 => vec![MicroInstr::NOP, MicroInstr::Read(Byte::LSB), MicroInstr::LDHLN], // LD (HL),u8
            0x1A => vec![MicroInstr::NOP, MicroInstr::LDRNN(Register::A, self.registers.get_de(), false)], // LD A,(DE)
            0x46 => vec![MicroInstr::NOP, MicroInstr::LDRNN(Register::B, self.registers.get_hl(), false)], // LD B,(HL)
            0x4E => vec![MicroInstr::NOP, MicroInstr::LDRNN(Register::C, self.registers.get_hl(), false)], // LD C,(HL)
            0x56 => vec![MicroInstr::NOP, MicroInstr::LDRNN(Register::D, self.registers.get_hl(), false)], // LD D,(HL)
            0x6E => vec![MicroInstr::NOP, MicroInstr::LDRNN(Register::L, self.registers.get_hl(), false)], // LD L,(HL)
            0x7E => vec![MicroInstr::NOP, MicroInstr::LDRNN(Register::A, self.registers.get_hl(), false)], // LD A,(HL)
            0x5E => vec![MicroInstr::NOP, MicroInstr::LDRNN(Register::E, self.registers.get_hl(), false)], // LD E,(HL)
            0x66 => vec![MicroInstr::NOP, MicroInstr::LDRNN(Register::H, self.registers.get_hl(), false)], // LD H,(HL)
            0x0A => vec![MicroInstr::NOP, MicroInstr::LDRNN(Register::A, self.registers.get_bc(), false)], // LD A,(BC)
            0x31 => vec![MicroInstr::Read(Byte::LSB), MicroInstr::Read(Byte::MSB), MicroInstr::LDSPNN], // LD SP,u16
            0x08 => vec![MicroInstr::NOP, MicroInstr::Read(Byte::LSB), MicroInstr::Read(Byte::MSB), MicroInstr::LDNNSP(Byte::LSB), MicroInstr::LDNNSP(Byte::MSB)], // LD (u16),SP
            0xF9 => vec![MicroInstr::NOP, MicroInstr::LDSPHL], // LD SP,HL
            0xF8 => vec![MicroInstr::NOP, MicroInstr::Read(Byte::LSB), MicroInstr::LDHLSPN], // LD HL,SP+i8
            0xEA => vec![MicroInstr::NOP, MicroInstr::Read(Byte::LSB), MicroInstr::Read(Byte::MSB), MicroInstr::LDNNR(0, Register::A, false)], // LD (u16),A
            0x3E => vec![MicroInstr::NOP, MicroInstr::LDRN(Register::A)], // LD A,u8
            0xE0 => vec![MicroInstr::NOP, MicroInstr::Read(Byte::LSB), MicroInstr::LDNNR(0xFF00, Register::A, true)], // LD (FF00+u8),A
            0xE2 => vec![MicroInstr::NOP, MicroInstr::LDNNR(0xFF00 + (self.registers[Register::C] as u16), Register::A, false)], // LD (FF00+C),A 
            0xF0 => vec![MicroInstr::NOP, MicroInstr::Read(Byte::LSB), MicroInstr::LDRNN(Register::A, 0xFF00, true)], // LD A,(FF00+u8)
            0xF2 => vec![MicroInstr::NOP, MicroInstr::LDRNN(Register::A, 0xFF00 + (self.registers[Register::C] as u16), false)], // LD A,(FF00+C)
            0xFA => vec![MicroInstr::NOP, MicroInstr::Read(Byte::LSB), MicroInstr::Read(Byte::MSB), MicroInstr::LDRNN(Register::A, 0, false)], // LD A,(u16)

            0x18 => vec![MicroInstr::Read(Byte::LSB), MicroInstr::NOP, MicroInstr::JR], // JR i8
            0x20 => vec![MicroInstr::Read(Byte::LSB), MicroInstr::Cond(Flag::Z, false), MicroInstr::JR], // JR NZ,i8
            0x30 => vec![MicroInstr::Read(Byte::LSB), MicroInstr::Cond(Flag::C, false), MicroInstr::JR], // JR NC,i8
            0x38 => vec![MicroInstr::Read(Byte::LSB), MicroInstr::Cond(Flag::C, true), MicroInstr::JR], // JR C,i8
            0x28 => vec![MicroInstr::Read(Byte::LSB), MicroInstr::Cond(Flag::Z, true), MicroInstr::JR], // JR Z,i8
            0xC3 => vec![MicroInstr::Read(Byte::LSB), MicroInstr::Read(Byte::MSB), MicroInstr::NOP, MicroInstr::JP], // JP u16
            0xC2 => vec![MicroInstr::Read(Byte::LSB), MicroInstr::Read(Byte::MSB), MicroInstr::Cond(Flag::Z, false), MicroInstr::JP], // JP NZ,u16
            0xCA => vec![MicroInstr::Read(Byte::LSB), MicroInstr::Read(Byte::MSB), MicroInstr::Cond(Flag::Z, true), MicroInstr::JP], // JP Z,u16
            0xD2 => vec![MicroInstr::Read(Byte::LSB), MicroInstr::Read(Byte::MSB), MicroInstr::Cond(Flag::C, false), MicroInstr::JP], // JP NC,u16
            0xDA => vec![MicroInstr::Read(Byte::LSB), MicroInstr::Read(Byte::MSB), MicroInstr::Cond(Flag::C, true), MicroInstr::JP], // JP C,u16
            0xE9 => vec![MicroInstr::JPHL], // JP HL
            0xCD => vec![MicroInstr::NOP, MicroInstr::Read(Byte::LSB), MicroInstr::Read(Byte::MSB), MicroInstr::PUSH(((0xFF00 & (self.pc + 2)) >> 8) as u8), MicroInstr::PUSH((0x00FF & (self.pc + 2)) as u8), MicroInstr::JP], // CALL u16
            0xC4 => vec![MicroInstr::Read(Byte::LSB), MicroInstr::Read(Byte::MSB), MicroInstr::Cond(Flag::Z, false), MicroInstr::PUSH(((0xFF00 & (self.pc + 2)) >> 8) as u8), MicroInstr::PUSH((0x00FF & (self.pc + 2)) as u8), MicroInstr::JP], // CALL NZ,u16
            0xCC => vec![MicroInstr::Read(Byte::LSB), MicroInstr::Read(Byte::MSB), MicroInstr::Cond(Flag::Z, true), MicroInstr::PUSH(((0xFF00 & (self.pc + 2)) >> 8) as u8), MicroInstr::PUSH((0x00FF & (self.pc + 2)) as u8), MicroInstr::JP], // CALL Z,u16
            0xD4 => vec![MicroInstr::Read(Byte::LSB), MicroInstr::Read(Byte::MSB), MicroInstr::Cond(Flag::C, false), MicroInstr::PUSH(((0xFF00 & (self.pc + 2)) >> 8) as u8), MicroInstr::PUSH((0x00FF & (self.pc + 2)) as u8), MicroInstr::JP], // CALL NC,u16
            0xDC => vec![MicroInstr::Read(Byte::LSB), MicroInstr::Read(Byte::MSB), MicroInstr::Cond(Flag::C, true), MicroInstr::PUSH(((0xFF00 & (self.pc + 2)) >> 8) as u8), MicroInstr::PUSH((0x00FF & (self.pc + 2)) as u8), MicroInstr::JP], // CALL C,u16
            0xC9 => vec![MicroInstr::NOP, MicroInstr::POPPC(Byte::LSB), MicroInstr::POPPC(Byte::MSB), MicroInstr::JP], // RET
            0xD0 => vec![MicroInstr::NOP, MicroInstr::Cond(Flag::C, false), MicroInstr::POPPC(Byte::LSB), MicroInstr::POPPC(Byte::MSB), MicroInstr::JP], // RET NC
            0xC8 => vec![MicroInstr::NOP, MicroInstr::Cond(Flag::Z, true), MicroInstr::POPPC(Byte::LSB), MicroInstr::POPPC(Byte::MSB), MicroInstr::JP], // RET Z
            0xC0 => vec![MicroInstr::NOP, MicroInstr::Cond(Flag::Z, false), MicroInstr::POPPC(Byte::LSB), MicroInstr::POPPC(Byte::MSB), MicroInstr::JP], // RET NZ
            0xD8 => vec![MicroInstr::NOP, MicroInstr::Cond(Flag::C, true), MicroInstr::POPPC(Byte::LSB), MicroInstr::POPPC(Byte::MSB), MicroInstr::JP], // RET C
            0xD9 => vec![MicroInstr::NOP, MicroInstr::POPPC(Byte::LSB), MicroInstr::POPPC(Byte::MSB), MicroInstr::RETI], // RETI
            0xC7 => vec![MicroInstr::NOP, MicroInstr::PUSH(((0xFF00 & self.pc) >> 8) as u8), MicroInstr::PUSH((0x00FF & self.pc) as u8), MicroInstr::RST(0x0000)], // RST 00h
            0xCF => vec![MicroInstr::NOP, MicroInstr::PUSH(((0xFF00 & self.pc) >> 8) as u8), MicroInstr::PUSH((0x00FF & self.pc) as u8), MicroInstr::RST(0x0008)], // RST 08h
            0xD7 => vec![MicroInstr::NOP, MicroInstr::PUSH(((0xFF00 & self.pc) >> 8) as u8), MicroInstr::PUSH((0x00FF & self.pc) as u8), MicroInstr::RST(0x0010)], // RST 10h
            0xDF => vec![MicroInstr::NOP, MicroInstr::PUSH(((0xFF00 & self.pc) >> 8) as u8), MicroInstr::PUSH((0x00FF & self.pc) as u8), MicroInstr::RST(0x0018)], // RST 18h
            0xE7 => vec![MicroInstr::NOP, MicroInstr::PUSH(((0xFF00 & self.pc) >> 8) as u8), MicroInstr::PUSH((0x00FF & self.pc) as u8), MicroInstr::RST(0x0020)], // RST 20h
            0xEF => vec![MicroInstr::NOP, MicroInstr::PUSH(((0xFF00 & self.pc) >> 8) as u8), MicroInstr::PUSH((0x00FF & self.pc) as u8), MicroInstr::RST(0x0028)], // RST 28h
            0xF7 => vec![MicroInstr::NOP, MicroInstr::PUSH(((0xFF00 & self.pc) >> 8) as u8), MicroInstr::PUSH((0x00FF & self.pc) as u8), MicroInstr::RST(0x0030)], // RST 30h
            0xFF => vec![MicroInstr::NOP, MicroInstr::PUSH(((0xFF00 & self.pc) >> 8) as u8), MicroInstr::PUSH((0x00FF & self.pc) as u8), MicroInstr::RST(0x0038)], // RST 38h

            0x34 => vec![MicroInstr::NOP, MicroInstr::NOP, MicroInstr::INCHLADDR],
            0x13 => vec![MicroInstr::NOP, MicroInstr::INCDE], // INC DE
            0x23 => vec![MicroInstr::NOP, MicroInstr::INCHL], // INC HL
            0x03 => vec![MicroInstr::NOP, MicroInstr::INCBC], // INC BC
            0x33 => vec![MicroInstr::NOP, MicroInstr::INCSP], // INC SP
            0x1C => vec![MicroInstr::INC(Register::E)], // INC E
            0x14 => vec![MicroInstr::INC(Register::D)], // INC D
            0x2C => vec![MicroInstr::INC(Register::L)], // INC L
            0x24 => vec![MicroInstr::INC(Register::H)], // INC H
            0x3C => vec![MicroInstr::INC(Register::A)], // INC A
            0x04 => vec![MicroInstr::INC(Register::B)], // INC B
            0x0C => vec![MicroInstr::INC(Register::C)], // INC C
            
            0x1B => vec![MicroInstr::NOP, MicroInstr::DECDE], // DEC DE
            0x0B => vec![MicroInstr::NOP, MicroInstr::DECBC], // DEC BC
            0x2B => vec![MicroInstr::NOP, MicroInstr::DECHL], // DEC HL
            0x3B => vec![MicroInstr::NOP, MicroInstr::DECSP], // DEC SP
            0x0D => vec![MicroInstr::DEC(Register::C)], // DEC C
            0x05 => vec![MicroInstr::DEC(Register::B)], // DEC B
            0x2D => vec![MicroInstr::DEC(Register::L)], // DEC L
            0x25 => vec![MicroInstr::DEC(Register::H)], // DEC H
            0x3D => vec![MicroInstr::DEC(Register::A)], // DEC A
            0x1D => vec![MicroInstr::DEC(Register::E)], // DEC E
            0x15 => vec![MicroInstr::DEC(Register::D)], // DEC D
            0x35 => vec![MicroInstr::NOP, MicroInstr::NOP, MicroInstr::DECNN(self.registers.get_hl())], // DEC (HL)
            
            0xB1 => vec![MicroInstr::OR(Register::C)], // OR A,C
            0xB7 => vec![MicroInstr::OR(Register::A)], // OR A,A
            0xB0 => vec![MicroInstr::OR(Register::B)], // OR A,B
            0xB2 => vec![MicroInstr::OR(Register::D)], // OR A,D
            0xB3 => vec![MicroInstr::OR(Register::E)], // OR A,E
            0xB4 => vec![MicroInstr::OR(Register::H)], // OR A,H
            0xB5 => vec![MicroInstr::OR(Register::L)], // OR A,L
            0xF6 => vec![MicroInstr::Read(Byte::LSB), MicroInstr::ORN], // OR A,u8
            0xB6 => vec![MicroInstr::NOP, MicroInstr::ORHL], // OR A,(HL)

            0xAF => vec![MicroInstr::XOR(Register::A)], // XOR A,A
            0xA9 => vec![MicroInstr::XOR(Register::C)], // XOR A,C
            0xAD => vec![MicroInstr::XOR(Register::L)], // XOR A,L
            0xA8 => vec![MicroInstr::XOR(Register::B)], // XOR A,B
            0xAA => vec![MicroInstr::XOR(Register::D)], // XOR A,D
            0xAB => vec![MicroInstr::XOR(Register::E)], // XOR A,E
            0xAC => vec![MicroInstr::XOR(Register::H)], // XOR A,H
            0xEE => vec![MicroInstr::Read(Byte::LSB), MicroInstr::XORN], // XOR A,u8
            0xAE => vec![MicroInstr::XORHL], // XOR A,(HL)

            0xA0 => vec![MicroInstr::AND(Register::B)], // AND A,B
            0xA1 => vec![MicroInstr::AND(Register::C)], // AND A,C
            0xA2 => vec![MicroInstr::AND(Register::D)], // AND A,D
            0xA3 => vec![MicroInstr::AND(Register::E)], // AND A,E
            0xA4 => vec![MicroInstr::AND(Register::H)], // AND A,H
            0xA5 => vec![MicroInstr::AND(Register::L)], // AND A,L
            0xA7 => vec![MicroInstr::AND(Register::A)], // AND A,A
            0xA6 => vec![MicroInstr::NOP, MicroInstr::ANDHL], // AND A,(HL)
            0xE6 => vec![MicroInstr::Read(Byte::LSB), MicroInstr::ANDN], // AND A,u8

            0xBE => vec![MicroInstr::NOP, MicroInstr::CPHL], // CP A,(HL)
            0xFE => vec![MicroInstr::Read(Byte::LSB), MicroInstr::CPN], // CP A,u8
            0xBB => vec![MicroInstr::CP(Register::E)], // CP A,E
            0xBA => vec![MicroInstr::CP(Register::D)], // CP A,D
            0xB9 => vec![MicroInstr::CP(Register::C)], // CP A,C
            0xB8 => vec![MicroInstr::CP(Register::B)], // CP A,B
            0xBC => vec![MicroInstr::CP(Register::H)], // CP A,H
            0xBD => vec![MicroInstr::CP(Register::L)], // CP A,L
            0xBF => vec![MicroInstr::CP(Register::A)], // CP A,A

            0x80 => vec![MicroInstr::ADD(Register::B)], // ADD A,B
            0x81 => vec![MicroInstr::ADD(Register::C)], // ADD A,C
            0x82 => vec![MicroInstr::ADD(Register::D)], // ADD A,D
            0x83 => vec![MicroInstr::ADD(Register::E)], // ADD A,E
            0x84 => vec![MicroInstr::ADD(Register::H)], // ADD A,H
            0x85 => vec![MicroInstr::ADD(Register::L)], // ADD A,L
            0x87 => vec![MicroInstr::ADD(Register::A)], // ADD A,A
            0x86 => vec![MicroInstr::NOP, MicroInstr::ADDHL],
            0xC6 => vec![MicroInstr::Read(Byte::LSB), MicroInstr::ADDN], // ADD A,u8
            0x29 => vec![MicroInstr::NOP, MicroInstr::ADDHLNN(self.registers.get_hl())], // ADD HL,HL
            0x09 => vec![MicroInstr::NOP, MicroInstr::ADDHLNN(self.registers.get_bc())], // ADD HL,BC
            0x19 => vec![MicroInstr::NOP, MicroInstr::ADDHLNN(self.registers.get_de())], // ADD HL,DE
            0x39 => vec![MicroInstr::NOP, MicroInstr::ADDHLNN(self.sp)], // ADD HL,SP
            0xE8 => vec![MicroInstr::NOP, MicroInstr::Read(Byte::LSB), MicroInstr::NOP, MicroInstr::ADDSPN], // ADD SP,i8

            0x88 => vec![MicroInstr::ADC(Register::B)], // ADC A,B
            0x89 => vec![MicroInstr::ADC(Register::C)], // ADC A,C
            0x8A => vec![MicroInstr::ADC(Register::D)], // ADC A,D
            0x8B => vec![MicroInstr::ADC(Register::E)], // ADC A,E
            0x8C => vec![MicroInstr::ADC(Register::H)], // ADC A,H
            0x8D => vec![MicroInstr::ADC(Register::L)], // ADC A,L
            0x8F => vec![MicroInstr::ADC(Register::A)], // ADC A,A
            0x8E => vec![MicroInstr::NOP, MicroInstr::ADCHL], // ADC A,(HL)
            0xCE => vec![MicroInstr::Read(Byte::LSB), MicroInstr::ADCN], // ADC A,u8

            0x90 => vec![MicroInstr::SUB(Register::B)], // SUB A,B
            0x91 => vec![MicroInstr::SUB(Register::C)], // SUB A,C
            0x92 => vec![MicroInstr::SUB(Register::D)], // SUB A,D
            0x93 => vec![MicroInstr::SUB(Register::E)], // SUB A,E
            0x94 => vec![MicroInstr::SUB(Register::H)], // SUB A,H
            0x95 => vec![MicroInstr::SUB(Register::L)], // SUB A,L
            0x97 => vec![MicroInstr::SUB(Register::A)], // SUB A,A
            0x96 => vec![MicroInstr::NOP, MicroInstr::SUBHL], // SUB A,(HL)
            0xD6 => vec![MicroInstr::Read(Byte::LSB), MicroInstr::SUBN], // SUB A,u8

            0x98 => vec![MicroInstr::SBC(Register::B)], // SBC A,B
            0x99 => vec![MicroInstr::SBC(Register::C)], // SBC A,C
            0x9A => vec![MicroInstr::SBC(Register::D)], // SBC A,D
            0x9B => vec![MicroInstr::SBC(Register::E)], // SBC A,E
            0x9C => vec![MicroInstr::SBC(Register::H)], // SBC A,H
            0x9D => vec![MicroInstr::SBC(Register::L)], // SBC A,L
            0x9F => vec![MicroInstr::SBC(Register::A)], // SBC A,A
            0x9E => vec![MicroInstr::NOP, MicroInstr::SBCHL], // SBC A,(HL)
            0xDE => vec![MicroInstr::Read(Byte::LSB), MicroInstr::SBCN], // SBC A,u8

            0xF5 => vec![MicroInstr::NOP, MicroInstr::NOP, MicroInstr::PUSH(self.registers[Register::A]), MicroInstr::PUSH(self.registers[Register::F])], // PUSH AF
            0xE5 => vec![MicroInstr::NOP, MicroInstr::NOP, MicroInstr::PUSH(self.registers[Register::H]), MicroInstr::PUSH(self.registers[Register::L])], // PUSH HL
            0xC5 => vec![MicroInstr::NOP, MicroInstr::NOP, MicroInstr::PUSH(self.registers[Register::B]), MicroInstr::PUSH(self.registers[Register::C])], // PUSH BC
            0xD5 => vec![MicroInstr::NOP, MicroInstr::NOP, MicroInstr::PUSH(self.registers[Register::D]), MicroInstr::PUSH(self.registers[Register::E])], // PUSH DE
            0xE1 => vec![MicroInstr::NOP, MicroInstr::POPR(Register::L), MicroInstr::POPR(Register::H)], // POP HL
            0xF1 => vec![MicroInstr::NOP, MicroInstr::POPR(Register::F), MicroInstr::POPR(Register::A)], // POP AF
            0xC1 => vec![MicroInstr::NOP, MicroInstr::POPR(Register::C), MicroInstr::POPR(Register::B)], // POP BC
            0xD1 => vec![MicroInstr::NOP, MicroInstr::POPR(Register::E), MicroInstr::POPR(Register::D)], // POP DE

            0x00 => vec![MicroInstr::NOP], // NOP
            0x1F => vec![MicroInstr::RRA], // RRA
            0x2F => vec![MicroInstr::CPL], // CPL
            0x27 => vec![MicroInstr::DAA], // DAA
            0x37 => vec![MicroInstr::SCF], // SCF
            0x3F => vec![MicroInstr::CCF], // CCF
            0xF3 => vec![MicroInstr::DI], // DI
            0x07 => vec![MicroInstr::RLCA], // RLCA
            0x17 => vec![MicroInstr::RLA], // RLA
            0x0F => vec![MicroInstr::RRCA], // RRCA
            0xFB => vec![MicroInstr::EI], // EI
            0x76 => vec![MicroInstr::HALT], // HALT
            0x10 => vec![MicroInstr::STOP], // STOP

            0xCB => vec![], // PREFIX

            _ => {
                console_log!("Instruction not implemented: 0x{:02X}", opcode);
                panic!("Instruction not implemented: 0x{:02X}", opcode)
            }
        }
    }
}