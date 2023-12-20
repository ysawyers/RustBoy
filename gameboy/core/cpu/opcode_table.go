package cpu

import (
	"github.com/ysawyers/emufun/gameboy/core/memory"
)

var opcodeTable = map[byte]opcode{
	0x00: {label: "NOP", value: 0x00, steps: []stepFunc{
		func() bool {
			return false
		},
	}},

	0x01: {label: "LD BC,u16", value: 0x01, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.Registers[c] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return true
		},
		func() bool {
			Processor.Registers[b] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return false
		},
	}},
	0x02: {label: "LD (BC),A", value: 0x02, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			memory.CPU.Write(getRegisterBC(), Processor.Registers[a])
			return false
		},
	}},
	0x03: {label: "INC BC", value: 0x03, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			setRegisterBC(getRegisterBC() + 1)
			return false
		},
	}},
	0x04: {label: "INC B", value: 0x04, steps: []stepFunc{
		func() bool {
			setZeroFlag((Processor.Registers[b] + 1) == 0)
			setSubtractFlag(false)
			setHalfCarryFlag((((Processor.Registers[b] & 0xF) + (1 & 0xF)) & 0x10) == 0x10)
			Processor.Registers[b]++
			return false
		},
	}},
	0x05: {label: "DEC B", value: 0x05, steps: []stepFunc{
		func() bool {
			setZeroFlag((Processor.Registers[b] - 1) == 0)
			setSubtractFlag(true)
			setHalfCarryFlag((((Processor.Registers[b] & 0xF) - (1 & 0xF)) & 0x10) == 0x10)
			Processor.Registers[b]--
			return false
		},
	}},
	0x06: {label: "LD B,u8", value: 0x06, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.Registers[b] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return false
		},
	}},
	0x07: {label: "RLCA", value: 0x07, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] = rotateLeftC(Processor.Registers[a])
			setZeroFlag(false)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x08: {label: "LD (u16),SP", value: 0x08, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.State.buffer[0] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return true
		},
		func() bool {
			Processor.State.buffer[1] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return true
		},
		func() bool {
			memory.CPU.Write((uint16(Processor.State.buffer[1])<<8)|uint16(Processor.State.buffer[0]), byte(0x00FF&Processor.SP))
			return true
		},
		func() bool {
			memory.CPU.Write((((uint16(Processor.State.buffer[1]) << 8) | uint16(Processor.State.buffer[0])) + 1), byte((0xFF00&Processor.SP)>>8))
			return false
		},
	}},
	0x09: {label: "ADD HL,BC", value: 0x09, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			setSubtractFlag(false)
			setHalfCarryFlag((((getRegisterHL() & 0x0FFF) + (getRegisterBC() & 0x0FFF)) & 0x1000) == 0x1000)
			setCarryFlag((uint32(getRegisterHL()) + uint32(getRegisterBC())) > 0xFFFF)
			setRegisterHL(getRegisterHL() + getRegisterBC())
			return false
		},
	}},
	0x0A: {label: "LD A,(BC)", value: 0x0A, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.Registers[a] = memory.CPU.Read(getRegisterBC())
			return false
		},
	}},
	0x0B: {label: "DEC BC", value: 0x0B, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			setRegisterBC(getRegisterBC() - 1)
			return false
		},
	}},
	0x0D: {label: "DEC C", value: 0x0D, steps: []stepFunc{
		func() bool {
			setZeroFlag((Processor.Registers[c] - 1) == 0)
			setSubtractFlag(true)
			setHalfCarryFlag((((Processor.Registers[c] & 0xF) - (1 & 0xF)) & 0x10) == 0x10)
			Processor.Registers[c]--
			return false
		},
	}},
	0x0F: {label: "RRCA", value: 0x0F, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] = rotateRightC(Processor.Registers[a])
			setZeroFlag(false)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x12: {label: "LD (DE),A", value: 0x12, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			memory.CPU.Write(getRegisterDE(), Processor.Registers[a])
			return false
		},
	}},
	0x13: {label: "INC DE", value: 0x13, steps: []stepFunc{
		func() bool {
			return true
		},
		func() bool {
			setRegisterDE(getRegisterDE() + 1)
			return false
		},
	}},
	0x15: {label: "DEC D", value: 0x15, steps: []stepFunc{
		func() bool {
			setZeroFlag((Processor.Registers[d] - 1) == 0)
			setSubtractFlag(true)
			setHalfCarryFlag((((Processor.Registers[d] & 0xF) - (1 & 0xF)) & 0x10) == 0x10)
			Processor.Registers[d]--
			return false
		},
	}},
	0x16: {label: "LD D,u8", value: 0x16, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.Registers[d] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return false
		},
	}},
	0x17: {label: "RLA", value: 0x17, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] = rotateLeft(Processor.Registers[a])
			setZeroFlag(false)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x18: {label: "JR i8", value: 0x18, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.State.buffer[0] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return true
		},
		func() bool {
			if int8(Processor.State.buffer[0]) < 0 {
				Processor.PC -= uint16(int8(Processor.State.buffer[0]) * -1)
			} else {
				Processor.PC += uint16(int8(Processor.State.buffer[0]))
			}
			return false
		},
	}},
	0x1E: {label: "LD E,u8", value: 0x1E, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.Registers[e] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return false
		},
	}},
	0x1F: {label: "RRA", value: 0x1F, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] = rotateRight(Processor.Registers[a])
			setZeroFlag(false)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x0C: {label: "INC C", value: 0x0C, steps: []stepFunc{
		func() bool {
			setZeroFlag((Processor.Registers[c] + 1) == 0)
			setSubtractFlag(false)
			setHalfCarryFlag((((Processor.Registers[c] & 0xF) + (1 & 0xF)) & 0x10) == 0x10)
			Processor.Registers[c]++
			return false
		},
	}},
	0x0E: {label: "LD C,u8", value: 0x0E, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.Registers[c] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return false
		},
	}},
	0x11: {label: "LD DE,u16", value: 0x11, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.Registers[e] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return true
		},
		func() bool {
			Processor.Registers[d] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return false
		},
	}},
	0x14: {label: "INC D", value: 0x14, steps: []stepFunc{
		func() bool {
			setZeroFlag((Processor.Registers[d] + 1) == 0)
			setSubtractFlag(false)
			setHalfCarryFlag((((Processor.Registers[d] & 0xF) + (1 & 0xF)) & 0x10) == 0x10)
			Processor.Registers[d]++
			return false
		},
	}},
	0x1A: {label: "LD A,(DE)", value: 0x1A, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.Registers[a] = memory.CPU.Read(getRegisterDE())
			return false
		},
	}},
	0x1B: {label: "DEC DE", value: 0x1B, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			setRegisterDE(getRegisterDE() - 1)
			return false
		},
	}},
	0x1C: {label: "INC E", value: 0x1C, steps: []stepFunc{
		func() bool {
			setZeroFlag((Processor.Registers[e] + 1) == 0)
			setSubtractFlag(false)
			setHalfCarryFlag((((Processor.Registers[e] & 0xF) + (1 & 0xF)) & 0x10) == 0x10)
			Processor.Registers[e]++
			return false
		},
	}},
	0x1D: {label: "DEC E", value: 0x1D, steps: []stepFunc{
		func() bool {
			setZeroFlag((Processor.Registers[e] - 1) == 0)
			setSubtractFlag(true)
			setHalfCarryFlag((((Processor.Registers[e] & 0xF) - (1 & 0xF)) & 0x10) == 0x10)
			Processor.Registers[e]--
			return false
		},
	}},
	0x19: {label: "ADD HL,DE", value: 0x19, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			setSubtractFlag(false)
			setHalfCarryFlag((((getRegisterHL() & 0x0FFF) + (getRegisterDE() & 0x0FFF)) & 0x1000) == 0x1000)
			setCarryFlag((uint32(getRegisterHL()) + uint32(getRegisterDE())) > 0xFFFF)
			setRegisterHL(getRegisterHL() + getRegisterDE())
			return false
		},
	}},
	0x20: {label: "JR NZ,i8", value: 0x20, steps: []stepFunc{
		func() bool {
			Processor.State.buffer[0] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return true
		},
		func() bool {
			return getZeroFlag() == 0
		},
		func() bool {
			if int8(Processor.State.buffer[0]) < 0 {
				Processor.PC -= uint16(int8(Processor.State.buffer[0]) * -1)
			} else {
				Processor.PC += uint16(int8(Processor.State.buffer[0]))
			}
			return false
		},
	}},
	0x21: {label: "LD HL,u16", value: 0x21, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.Registers[l] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return true
		},
		func() bool {
			Processor.Registers[h] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return false
		},
	}},
	0x22: {label: "LD (HL+),A", value: 0x22, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			memory.CPU.Write(getRegisterHL(), Processor.Registers[a])
			setRegisterHL(getRegisterHL() + 1)
			return false
		},
	}},
	0x23: {label: "INC HL", value: 0x23, steps: []stepFunc{
		func() bool {
			return true
		},
		func() bool {
			setRegisterHL(getRegisterHL() + 1)
			return false
		},
	}},
	0x24: {label: "INC H", value: 0x24, steps: []stepFunc{
		func() bool {
			setZeroFlag((Processor.Registers[h] + 1) == 0)
			setSubtractFlag(false)
			setHalfCarryFlag((((Processor.Registers[h] & 0xF) + (1 & 0xF)) & 0x10) == 0x10)
			Processor.Registers[h]++
			return false
		},
	}},
	0x25: {label: "DEC H", value: 0x25, steps: []stepFunc{
		func() bool {
			setZeroFlag((Processor.Registers[h] - 1) == 0)
			setSubtractFlag(true)
			setHalfCarryFlag((((Processor.Registers[h] & 0xF) - (1 & 0xF)) & 0x10) == 0x10)
			Processor.Registers[h]--
			return false
		},
	}},
	0x26: {label: "LD H,u8", value: 0x26, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.Registers[h] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return false
		},
	}},
	0x27: {label: "DAA", value: 0x27, steps: []stepFunc{
		func() bool {
			var corr byte = 0

			if (getHalfCarryFlag() == 1) || ((getSubtractFlag() == 0) && ((Processor.Registers[a] & 0xF) > 9)) {
				corr |= 0x6
			}

			if (getCarryFlag() == 1) || ((getSubtractFlag() == 0) && (Processor.Registers[a] > 0x99)) {
				corr |= 0x60
				setCarryFlag(true)
			}

			if getSubtractFlag() == 1 {
				Processor.Registers[a] -= corr
			} else {
				Processor.Registers[a] += corr
			}

			setZeroFlag(Processor.Registers[a] == 0)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x28: {label: "JR Z,i8", value: 0x28, steps: []stepFunc{
		func() bool { // "fetch"
			Processor.State.buffer[0] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return true
		},
		func() bool {
			return getZeroFlag() == 1
		},
		func() bool {
			if int8(Processor.State.buffer[0]) < 0 {
				Processor.PC -= uint16(int8(Processor.State.buffer[0]) * -1)
			} else {
				Processor.PC += uint16(int8(Processor.State.buffer[0]))
			}
			return false
		},
	}},
	0x29: {label: "ADD HL,HL", value: 0x29, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			setSubtractFlag(false)
			setHalfCarryFlag((((getRegisterHL() & 0x0FFF) + (getRegisterHL() & 0x0FFF)) & 0x1000) == 0x1000)
			setCarryFlag((uint32(getRegisterHL()) + uint32(getRegisterHL())) > 0xFFFF)
			setRegisterHL(getRegisterHL() + getRegisterHL())
			return false
		},
	}},
	0x30: {label: "JR NC,i8", value: 0x30, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.State.buffer[0] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return true
		},
		func() bool {
			return getCarryFlag() == 0
		},
		func() bool {
			if int8(Processor.State.buffer[0]) < 0 {
				Processor.PC -= uint16(int8(Processor.State.buffer[0]) * -1)
			} else {
				Processor.PC += uint16(int8(Processor.State.buffer[0]))
			}
			return false
		},
	}},
	0x2A: {label: "LD A,(HL+)", value: 0x2A, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.Registers[a] = memory.CPU.Read(getRegisterHL())
			setRegisterHL(getRegisterHL() + 1)
			return false
		},
	}},
	0x2B: {label: "DEC HL", value: 0x2B, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			setRegisterHL(getRegisterHL() - 1)
			return false
		},
	}},
	0x2C: {label: "INC L", value: 0x2C, steps: []stepFunc{
		func() bool {
			setZeroFlag((Processor.Registers[l] + 1) == 0)
			setSubtractFlag(false)
			setHalfCarryFlag((((Processor.Registers[l] & 0xF) + (1 & 0xF)) & 0x10) == 0x10)
			Processor.Registers[l]++
			return false
		},
	}},
	0x2D: {label: "DEC L", value: 0x2D, steps: []stepFunc{
		func() bool {
			setZeroFlag((Processor.Registers[l] - 1) == 0)
			setSubtractFlag(true)
			setHalfCarryFlag((((Processor.Registers[l] & 0xF) - (1 & 0xF)) & 0x10) == 0x10)
			Processor.Registers[l]--
			return false
		},
	}},
	0x2E: {label: "LD L,u8", value: 0x2E, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.Registers[l] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return false
		},
	}},
	0x2F: {label: "CPL", value: 0x2F, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] = ^Processor.Registers[a]
			setSubtractFlag(true)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x31: {label: "LD SP,u16", value: 0x31, steps: []stepFunc{
		func() bool {
			Processor.State.buffer[0] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return true
		},
		func() bool {
			Processor.State.buffer[1] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return true
		},
		func() bool {
			Processor.SP = (uint16(Processor.State.buffer[1]) << 8) | uint16(Processor.State.buffer[0])
			return false
		},
	}},
	0x32: {label: "LD (HL-),A", value: 0x32, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			memory.CPU.Write(getRegisterHL(), Processor.Registers[a])
			setRegisterHL(getRegisterHL() - 1)
			return false
		},
	}},
	0x33: {label: "INC SP", value: 0x33, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.SP++
			return false
		},
	}},
	0x34: {label: "INC (HL)", value: 0x34, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool { // "read"
			return true
		},
		func() bool {
			setZeroFlag((memory.CPU.Read(getRegisterHL()) + 1) == 0)
			setSubtractFlag(false)
			setHalfCarryFlag((((memory.CPU.Read(getRegisterHL()) & 0xF) + (1 & 0xF)) & 0x10) == 0x10)
			memory.CPU.Write(getRegisterHL(), (memory.CPU.Read(getRegisterHL()) + 1))
			return false
		},
	}},
	0x35: {label: "DEC (HL)", value: 0x35, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool { // "read"
			return true
		},
		func() bool {
			setZeroFlag((memory.CPU.Read(getRegisterHL()) - 1) == 0)
			setSubtractFlag(true)
			setHalfCarryFlag((((memory.CPU.Read(getRegisterHL()) & 0xF) - (1 & 0xF)) & 0x10) == 0x10)
			memory.CPU.Write(getRegisterHL(), (memory.CPU.Read(getRegisterHL()) - 1))
			return false
		},
	}},
	0x36: {label: "LD (HL),u8", value: 0x36, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool { // "read"
			return true
		},
		func() bool {
			memory.CPU.Write(getRegisterHL(), memory.CPU.Read(Processor.PC))
			Processor.PC++
			return false
		},
	}},
	0x37: {label: "SCF", value: 0x37, steps: []stepFunc{
		func() bool {
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			setCarryFlag(true)
			return false
		},
	}},
	0x38: {label: "JR C,i8", value: 0x38, steps: []stepFunc{
		func() bool {
			Processor.State.buffer[0] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return true
		},
		func() bool {
			return getCarryFlag() == 1
		},
		func() bool {
			if int8(Processor.State.buffer[0]) < 0 {
				Processor.PC -= uint16(int8(Processor.State.buffer[0]) * -1)
			} else {
				Processor.PC += uint16(int8(Processor.State.buffer[0]))
			}
			return false
		},
	}},
	0x39: {label: "ADD HL,SP", value: 0x39, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			setSubtractFlag(false)
			setHalfCarryFlag((((getRegisterHL() & 0x0FFF) + (Processor.SP & 0x0FFF)) & 0x1000) == 0x1000)
			setCarryFlag((uint32(getRegisterHL()) + uint32(Processor.SP)) > 0xFFFF)
			setRegisterHL(getRegisterHL() + Processor.SP)
			return false
		},
	}},
	0x3A: {label: "LD A,(HL-)", value: 0x3A, steps: []stepFunc{
		func() bool {
			return true
		},
		func() bool {
			Processor.Registers[a] = memory.CPU.Read(getRegisterHL())
			setRegisterHL(getRegisterHL() - 1)
			return false
		},
	}},
	0x3B: {label: "DEC SP", value: 0x3B, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.SP--
			return false
		},
	}},
	0x3C: {label: "INC A", value: 0x3C, steps: []stepFunc{
		func() bool {
			setZeroFlag((Processor.Registers[a] + 1) == 0)
			setSubtractFlag(false)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) + (1 & 0xF)) & 0x10) == 0x10)
			Processor.Registers[a]++
			return false
		},
	}},
	0x3D: {label: "DEC A", value: 0x3D, steps: []stepFunc{
		func() bool {
			setZeroFlag((Processor.Registers[a] - 1) == 0)
			setSubtractFlag(true)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) - (1 & 0xF)) & 0x10) == 0x10)
			Processor.Registers[a]--
			return false
		},
	}},
	0x3E: {label: "LD A,u8", value: 0x3E, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.Registers[a] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return false
		},
	}},
	0x3F: {label: "CCF", value: 0x3F, steps: []stepFunc{
		func() bool {
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			setCarryFlag(!(getCarryFlag() == 0x1))
			return false
		},
	}},
	0x40: {label: "LD B,B", value: 0x40, steps: []stepFunc{
		func() bool { // NOP
			return false
		},
	}},
	0x41: {label: "LD B,C", value: 0x41, steps: []stepFunc{
		func() bool {
			Processor.Registers[b] = Processor.Registers[c]
			return false
		},
	}},
	0x42: {label: "LD B, D", value: 0x42, steps: []stepFunc{
		func() bool {
			Processor.Registers[b] = Processor.Registers[d]
			return false
		},
	}},
	0x43: {label: "LD B,E", value: 0x43, steps: []stepFunc{
		func() bool {
			Processor.Registers[b] = Processor.Registers[e]
			return false
		},
	}},
	0x44: {label: "LD B,H", value: 0x44, steps: []stepFunc{
		func() bool {
			Processor.Registers[b] = Processor.Registers[h]
			return false
		},
	}},
	0x45: {label: "LD B,L", value: 0x45, steps: []stepFunc{
		func() bool {
			Processor.Registers[b] = Processor.Registers[l]
			return false
		},
	}},
	0x46: {label: "LD B,(HL)", value: 0x46, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.Registers[b] = memory.CPU.Read(getRegisterHL())
			return false
		},
	}},
	0x47: {label: "LD B,A", value: 0x47, steps: []stepFunc{
		func() bool {
			Processor.Registers[b] = Processor.Registers[a]
			return false
		},
	}},
	0x48: {label: "LD C,B", value: 0x48, steps: []stepFunc{
		func() bool {
			Processor.Registers[c] = Processor.Registers[b]
			return false
		},
	}},
	0x49: {label: "LD C,C", value: 0x49, steps: []stepFunc{
		func() bool { // NOP
			return false
		},
	}},
	0x4A: {label: "LD C,D", value: 0x4A, steps: []stepFunc{
		func() bool {
			Processor.Registers[c] = Processor.Registers[d]
			return false
		},
	}},
	0x4B: {label: "LD C,E", value: 0x4B, steps: []stepFunc{
		func() bool {
			Processor.Registers[c] = Processor.Registers[e]
			return false
		},
	}},
	0x4C: {label: "LD C,H", value: 0x4C, steps: []stepFunc{
		func() bool {
			Processor.Registers[c] = Processor.Registers[h]
			return false
		},
	}},
	0x4D: {label: "LD C,L", value: 0x4D, steps: []stepFunc{
		func() bool {
			Processor.Registers[c] = Processor.Registers[l]
			return false
		},
	}},
	0x4E: {label: "LD C,(HL)", value: 0x4E, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.Registers[c] = memory.CPU.Read(getRegisterHL())
			return false
		},
	}},
	0x4F: {label: "LD C,A", value: 0x4F, steps: []stepFunc{
		func() bool {
			Processor.Registers[c] = Processor.Registers[a]
			return false
		},
	}},
	0x50: {label: "LD D,B", value: 0x50, steps: []stepFunc{
		func() bool {
			Processor.Registers[d] = Processor.Registers[b]
			return false
		},
	}},
	0x51: {label: "LD D,C", value: 0x51, steps: []stepFunc{
		func() bool {
			Processor.Registers[d] = Processor.Registers[c]
			return false
		},
	}},
	0x52: {label: "LD D,D", value: 0x52, steps: []stepFunc{
		func() bool { // NOP
			return false
		},
	}},
	0x53: {label: "LD D,E", value: 0x53, steps: []stepFunc{
		func() bool {
			Processor.Registers[d] = Processor.Registers[e]
			return false
		},
	}},
	0x54: {label: "LD D,H", value: 0x54, steps: []stepFunc{
		func() bool {
			Processor.Registers[d] = Processor.Registers[h]
			return false
		},
	}},
	0x55: {label: "LD D,L", value: 0x55, steps: []stepFunc{
		func() bool {
			Processor.Registers[d] = Processor.Registers[l]
			return false
		},
	}},
	0x56: {label: "LD D,(HL)", value: 0x56, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.Registers[d] = memory.CPU.Read(getRegisterHL())
			return false
		},
	}},
	0x57: {label: "LD D,A", value: 0x57, steps: []stepFunc{
		func() bool {
			Processor.Registers[d] = Processor.Registers[a]
			return false
		},
	}},
	0x58: {label: "LD E,B", value: 0x58, steps: []stepFunc{
		func() bool {
			Processor.Registers[e] = Processor.Registers[b]
			return false
		},
	}},
	0x59: {label: "LD E,C", value: 0x59, steps: []stepFunc{
		func() bool {
			Processor.Registers[e] = Processor.Registers[c]
			return false
		},
	}},
	0x5A: {label: "LD E,D", value: 0x5A, steps: []stepFunc{
		func() bool {
			Processor.Registers[e] = Processor.Registers[d]
			return false
		},
	}},
	0x5B: {label: "LD E,E", value: 0x5B, steps: []stepFunc{
		func() bool { // NOP
			return false
		},
	}},
	0x5C: {label: "LD E,H", value: 0x5C, steps: []stepFunc{
		func() bool {
			Processor.Registers[e] = Processor.Registers[h]
			return false
		},
	}},
	0x5D: {label: "LD E,L", value: 0x5D, steps: []stepFunc{
		func() bool {
			Processor.Registers[e] = Processor.Registers[l]
			return false
		},
	}},
	0x5E: {label: "LD E,(HL)", value: 0x5E, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.Registers[e] = memory.CPU.Read(getRegisterHL())
			return false
		},
	}},
	0x5F: {label: "LD E,A", value: 0x5F, steps: []stepFunc{
		func() bool {
			Processor.Registers[e] = Processor.Registers[a]
			return false
		},
	}},
	0x60: {label: "LD H,B", value: 0x60, steps: []stepFunc{
		func() bool {
			Processor.Registers[h] = Processor.Registers[b]
			return false
		},
	}},
	0x61: {label: "LD H,C", value: 0x61, steps: []stepFunc{
		func() bool {
			Processor.Registers[h] = Processor.Registers[c]
			return false
		},
	}},
	0x62: {label: "LD H,D", value: 0x62, steps: []stepFunc{
		func() bool {
			Processor.Registers[h] = Processor.Registers[d]
			return false
		},
	}},
	0x63: {label: "LD H,E", value: 0x63, steps: []stepFunc{
		func() bool {
			Processor.Registers[h] = Processor.Registers[e]
			return false
		},
	}},
	0x64: {label: "LD H,H", value: 0x64, steps: []stepFunc{
		func() bool { // NOP
			return false
		},
	}},
	0x65: {label: "LD H,L", value: 0x65, steps: []stepFunc{
		func() bool {
			Processor.Registers[h] = Processor.Registers[l]
			return false
		},
	}},
	0x66: {label: "LD H,(HL)", value: 0x66, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.Registers[h] = memory.CPU.Read(getRegisterHL())
			return false
		},
	}},
	0x67: {label: "LD H,A", value: 0x67, steps: []stepFunc{
		func() bool {
			Processor.Registers[h] = Processor.Registers[a]
			return false
		},
	}},
	0x68: {label: "LD L,B", value: 0x68, steps: []stepFunc{
		func() bool {
			Processor.Registers[l] = Processor.Registers[b]
			return false
		},
	}},
	0x69: {label: "LD L,C", value: 0x69, steps: []stepFunc{
		func() bool {
			Processor.Registers[l] = Processor.Registers[c]
			return false
		},
	}},
	0x6A: {label: "LD L,D", value: 0x6A, steps: []stepFunc{
		func() bool {
			Processor.Registers[l] = Processor.Registers[d]
			return false
		},
	}},
	0x6B: {label: "LD L,E", value: 0x6B, steps: []stepFunc{
		func() bool {
			Processor.Registers[l] = Processor.Registers[e]
			return false
		},
	}},
	0x6C: {label: "LD L,H", value: 0x6C, steps: []stepFunc{
		func() bool {
			Processor.Registers[l] = Processor.Registers[h]
			return false
		},
	}},
	0x6D: {label: "LD L,L", value: 0x6D, steps: []stepFunc{
		func() bool { // NOP
			return false
		},
	}},
	0x6E: {label: "LD L,(HL)", value: 0x6E, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.Registers[l] = memory.CPU.Read(getRegisterHL())
			return false
		},
	}},
	0x6F: {label: "LD L,A", value: 0x6F, steps: []stepFunc{
		func() bool {
			Processor.Registers[l] = Processor.Registers[a]
			return false
		},
	}},
	0x70: {label: "LD (HL),B", value: 0x70, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			memory.CPU.Write(getRegisterHL(), Processor.Registers[b])
			return false
		},
	}},
	0x71: {label: "LD (HL),C", value: 0x71, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			memory.CPU.Write(getRegisterHL(), Processor.Registers[c])
			return false
		},
	}},
	0x72: {label: "LD (HL),D", value: 0x72, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			memory.CPU.Write(getRegisterHL(), Processor.Registers[d])
			return false
		},
	}},
	0x73: {label: "LD (HL),E", value: 0x73, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			memory.CPU.Write(getRegisterHL(), Processor.Registers[e])
			return false
		},
	}},
	0x74: {label: "LD (HL),H", value: 0x74, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			memory.CPU.Write(getRegisterHL(), Processor.Registers[h])
			return false
		},
	}},
	0x75: {label: "LD (HL),L", value: 0x75, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			memory.CPU.Write(getRegisterHL(), Processor.Registers[l])
			return false
		},
	}},
	// 0x76: {label: "HALT", value: 0x76, steps: []stepFunc{
	// 	func() bool {
	// 		// TODO: Figure out how I want to handle this.
	// 		Processor.PC--
	// 		return false
	// 	},
	// }},
	0x77: {label: "LD (HL),A", value: 0x77, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			memory.CPU.Write(getRegisterHL(), Processor.Registers[a])
			return false
		},
	}},
	0x78: {label: "LD A,B", value: 0x78, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] = Processor.Registers[b]
			return false
		},
	}},
	0x79: {label: "LD A,C", value: 0x79, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] = Processor.Registers[c]
			return false
		},
	}},
	0x7A: {label: "LD A,D", value: 0x7A, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] = Processor.Registers[d]
			return false
		},
	}},
	0x7B: {label: "LD A,E", value: 0x7B, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] = Processor.Registers[e]
			return false
		},
	}},
	0x7C: {label: "LD A,H", value: 0x7C, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] = Processor.Registers[h]
			return false
		},
	}},
	0x7D: {label: "LD A,L", value: 0x7D, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] = Processor.Registers[l]
			return false
		},
	}},
	0x7E: {label: "LD A,(HL)", value: 0x7E, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.Registers[a] = memory.CPU.Read(getRegisterHL())
			return false
		},
	}},
	0x7F: {label: "LD A,A", value: 0x7F, steps: []stepFunc{
		func() bool { // NOP
			return false
		},
	}},
	0x80: {label: "ADD A,B", value: 0x80, steps: []stepFunc{
		func() bool {
			setZeroFlag((Processor.Registers[a] + Processor.Registers[b]) == 0)
			setSubtractFlag(false)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) + (Processor.Registers[b] & 0xF)) & 0x10) == 0x10)
			setCarryFlag((uint16(Processor.Registers[a]) + uint16(Processor.Registers[b])) > 0xFF)
			Processor.Registers[a] += Processor.Registers[b]
			return false
		},
	}},
	0x81: {label: "ADD A,C", value: 0x81, steps: []stepFunc{
		func() bool {
			setZeroFlag((Processor.Registers[a] + Processor.Registers[c]) == 0)
			setSubtractFlag(false)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) + (Processor.Registers[c] & 0xF)) & 0x10) == 0x10)
			setCarryFlag((uint16(Processor.Registers[a]) + uint16(Processor.Registers[c])) > 0xFF)
			Processor.Registers[a] += Processor.Registers[c]
			return false
		},
	}},
	0x82: {label: "ADD A,D", value: 0x82, steps: []stepFunc{
		func() bool {
			setZeroFlag((Processor.Registers[a] + Processor.Registers[d]) == 0)
			setSubtractFlag(false)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) + (Processor.Registers[d] & 0xF)) & 0x10) == 0x10)
			setCarryFlag((uint16(Processor.Registers[a]) + uint16(Processor.Registers[d])) > 0xFF)
			Processor.Registers[a] += Processor.Registers[d]
			return false
		},
	}},
	0x83: {label: "ADD A,E", value: 0x83, steps: []stepFunc{
		func() bool {
			setZeroFlag((Processor.Registers[a] + Processor.Registers[e]) == 0)
			setSubtractFlag(false)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) + (Processor.Registers[e] & 0xF)) & 0x10) == 0x10)
			setCarryFlag((uint16(Processor.Registers[a]) + uint16(Processor.Registers[e])) > 0xFF)
			Processor.Registers[a] += Processor.Registers[e]
			return false
		},
	}},
	0x84: {label: "ADD A,H", value: 0x84, steps: []stepFunc{
		func() bool {
			setZeroFlag((Processor.Registers[a] + Processor.Registers[h]) == 0)
			setSubtractFlag(false)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) + (Processor.Registers[h] & 0xF)) & 0x10) == 0x10)
			setCarryFlag((uint16(Processor.Registers[a]) + uint16(Processor.Registers[h])) > 0xFF)
			Processor.Registers[a] += Processor.Registers[h]
			return false
		},
	}},
	0x85: {label: "ADD A,L", value: 0x85, steps: []stepFunc{
		func() bool {
			setZeroFlag((Processor.Registers[a] + Processor.Registers[l]) == 0)
			setSubtractFlag(false)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) + (Processor.Registers[l] & 0xF)) & 0x10) == 0x10)
			setCarryFlag((uint16(Processor.Registers[a]) + uint16(Processor.Registers[l])) > 0xFF)
			Processor.Registers[a] += Processor.Registers[l]
			return false
		},
	}},
	0x86: {label: "ADD A,(HL)", value: 0x86, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			setZeroFlag((Processor.Registers[a] + memory.CPU.Read(getRegisterHL())) == 0)
			setSubtractFlag(false)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) + (memory.CPU.Read(getRegisterHL()) & 0xF)) & 0x10) == 0x10)
			setCarryFlag((uint16(Processor.Registers[a]) + uint16(memory.CPU.Read(getRegisterHL()))) > 0xFF)
			Processor.Registers[a] += memory.CPU.Read(getRegisterHL())
			return false
		},
	}},
	0x87: {label: "ADD A,A", value: 0x87, steps: []stepFunc{
		func() bool {
			setZeroFlag((Processor.Registers[a] + Processor.Registers[a]) == 0)
			setSubtractFlag(false)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) + (Processor.Registers[a] & 0xF)) & 0x10) == 0x10)
			setCarryFlag(Processor.Registers[a] > 127)
			Processor.Registers[a] += Processor.Registers[a]
			return false
		},
	}},
	0x88: {label: "ADC A,B", value: 0x88, steps: []stepFunc{
		func() bool {
			CY := getCarryFlag()
			setZeroFlag((Processor.Registers[a] + Processor.Registers[b] + CY) == 0)
			setSubtractFlag(false)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) + (Processor.Registers[b] & 0xF) + CY) & 0x10) == 0x10)
			setCarryFlag((uint16(Processor.Registers[a]) + uint16(Processor.Registers[b]) + uint16(CY)) > 0xFF)
			Processor.Registers[a] += (Processor.Registers[b] + CY)
			return false
		},
	}},
	0x89: {label: "ADC A,C", value: 0x89, steps: []stepFunc{
		func() bool {
			CY := getCarryFlag()
			setZeroFlag((Processor.Registers[a] + Processor.Registers[c] + CY) == 0)
			setSubtractFlag(false)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) + (Processor.Registers[c] & 0xF) + CY) & 0x10) == 0x10)
			setCarryFlag((uint16(Processor.Registers[a]) + uint16(Processor.Registers[c]) + uint16(CY)) > 0xFF)
			Processor.Registers[a] += (Processor.Registers[c] + CY)
			return false
		},
	}},
	0x8A: {label: "ADC A,D", value: 0x8A, steps: []stepFunc{
		func() bool {
			CY := getCarryFlag()
			setZeroFlag((Processor.Registers[a] + Processor.Registers[d] + CY) == 0)
			setSubtractFlag(false)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) + (Processor.Registers[d] & 0xF) + CY) & 0x10) == 0x10)
			setCarryFlag((uint16(Processor.Registers[a]) + uint16(Processor.Registers[d]) + uint16(CY)) > 0xFF)
			Processor.Registers[a] += (Processor.Registers[d] + CY)
			return false
		},
	}},
	0x8B: {label: "ADC A,E", value: 0x8B, steps: []stepFunc{
		func() bool {
			CY := getCarryFlag()
			setZeroFlag((Processor.Registers[a] + Processor.Registers[e] + CY) == 0)
			setSubtractFlag(false)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) + (Processor.Registers[e] & 0xF) + CY) & 0x10) == 0x10)
			setCarryFlag((uint16(Processor.Registers[a]) + uint16(Processor.Registers[e]) + uint16(CY)) > 0xFF)
			Processor.Registers[a] += (Processor.Registers[e] + CY)
			return false
		},
	}},
	0x8C: {label: "ADC A,H", value: 0x8C, steps: []stepFunc{
		func() bool {
			CY := getCarryFlag()
			setZeroFlag((Processor.Registers[a] + Processor.Registers[h] + CY) == 0)
			setSubtractFlag(false)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) + (Processor.Registers[h] & 0xF) + CY) & 0x10) == 0x10)
			setCarryFlag((uint16(Processor.Registers[a]) + uint16(Processor.Registers[h]) + uint16(CY)) > 0xFF)
			Processor.Registers[a] += (Processor.Registers[h] + CY)
			return false
		},
	}},
	0x8D: {label: "ADC A,L", value: 0x8D, steps: []stepFunc{
		func() bool {
			CY := getCarryFlag()
			setZeroFlag((Processor.Registers[a] + Processor.Registers[l] + CY) == 0)
			setSubtractFlag(false)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) + (Processor.Registers[l] & 0xF) + CY) & 0x10) == 0x10)
			setCarryFlag((uint16(Processor.Registers[a]) + uint16(Processor.Registers[l]) + uint16(CY)) > 0xFF)
			Processor.Registers[a] += (Processor.Registers[l] + CY)
			return false
		},
	}},
	0x8E: {label: "ADC A,(HL)", value: 0x8E, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			CY := getCarryFlag()
			setZeroFlag((Processor.Registers[a] + memory.CPU.Read(getRegisterHL()) + CY) == 0)
			setSubtractFlag(false)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) + (memory.CPU.Read(getRegisterHL()) & 0xF) + CY) & 0x10) == 0x10)
			setCarryFlag((uint16(Processor.Registers[a]) + uint16(memory.CPU.Read(getRegisterHL())) + uint16(CY)) > 0xFF)
			Processor.Registers[a] += (memory.CPU.Read(getRegisterHL()) + CY)
			return false
		},
	}},
	0x8F: {label: "ADC A,A", value: 0x8F, steps: []stepFunc{
		func() bool {
			CY := getCarryFlag()
			setZeroFlag((Processor.Registers[a] + Processor.Registers[a] + CY) == 0)
			setSubtractFlag(false)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) + (Processor.Registers[a] & 0xF) + CY) & 0x10) == 0x10)
			setCarryFlag((uint16(Processor.Registers[a]) + uint16(Processor.Registers[a]) + uint16(CY)) > 0xFF)
			Processor.Registers[a] += (Processor.Registers[a] + CY)
			return false
		},
	}},
	0x90: {label: "SUB A,B", value: 0x90, steps: []stepFunc{
		func() bool {
			setZeroFlag((Processor.Registers[a] - Processor.Registers[b]) == 0)
			setSubtractFlag(true)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) - (Processor.Registers[b] & 0xF)) & 0x10) == 0x10)
			setCarryFlag(Processor.Registers[a] < Processor.Registers[b])
			Processor.Registers[a] -= Processor.Registers[b]
			return false
		},
	}},
	0x91: {label: "SUB A,C", value: 0x91, steps: []stepFunc{
		func() bool {
			setZeroFlag((Processor.Registers[a] - Processor.Registers[c]) == 0)
			setSubtractFlag(true)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) - (Processor.Registers[c] & 0xF)) & 0x10) == 0x10)
			setCarryFlag(Processor.Registers[a] < Processor.Registers[c])
			Processor.Registers[a] -= Processor.Registers[c]
			return false
		},
	}},
	0x92: {label: "SUB A,D", value: 0x92, steps: []stepFunc{
		func() bool {
			setZeroFlag((Processor.Registers[a] - Processor.Registers[d]) == 0)
			setSubtractFlag(true)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) - (Processor.Registers[d] & 0xF)) & 0x10) == 0x10)
			setCarryFlag(Processor.Registers[a] < Processor.Registers[d])
			Processor.Registers[a] -= Processor.Registers[d]
			return false
		},
	}},
	0x93: {label: "SUB A,E", value: 0x93, steps: []stepFunc{
		func() bool {
			setZeroFlag((Processor.Registers[a] - Processor.Registers[e]) == 0)
			setSubtractFlag(true)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) - (Processor.Registers[e] & 0xF)) & 0x10) == 0x10)
			setCarryFlag(Processor.Registers[a] < Processor.Registers[e])
			Processor.Registers[a] -= Processor.Registers[e]
			return false
		},
	}},
	0x94: {label: "SUB A,H", value: 0x94, steps: []stepFunc{
		func() bool {
			setZeroFlag((Processor.Registers[a] - Processor.Registers[h]) == 0)
			setSubtractFlag(true)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) - (Processor.Registers[h] & 0xF)) & 0x10) == 0x10)
			setCarryFlag(Processor.Registers[a] < Processor.Registers[h])
			Processor.Registers[a] -= Processor.Registers[h]
			return false
		},
	}},
	0x95: {label: "SUB A,L", value: 0x95, steps: []stepFunc{
		func() bool {
			setZeroFlag((Processor.Registers[a] - Processor.Registers[l]) == 0)
			setSubtractFlag(true)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) - (Processor.Registers[l] & 0xF)) & 0x10) == 0x10)
			setCarryFlag(Processor.Registers[a] < Processor.Registers[l])
			Processor.Registers[a] -= Processor.Registers[l]
			return false
		},
	}},
	0x96: {label: "SUB A,(HL)", value: 0x96, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			setZeroFlag((Processor.Registers[a] - memory.CPU.Read(getRegisterHL())) == 0)
			setSubtractFlag(true)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) - (memory.CPU.Read(getRegisterHL()) & 0xF)) & 0x10) == 0x10)
			setCarryFlag(Processor.Registers[a] < memory.CPU.Read(getRegisterHL()))
			Processor.Registers[a] -= memory.CPU.Read(getRegisterHL())
			return false
		},
	}},
	0x97: {label: "SUB A,A", value: 0x97, steps: []stepFunc{
		func() bool {
			setZeroFlag((Processor.Registers[a] - Processor.Registers[a]) == 0)
			setSubtractFlag(true)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) - (Processor.Registers[a] & 0xF)) & 0x10) == 0x10)
			setCarryFlag(Processor.Registers[a] < Processor.Registers[a])
			Processor.Registers[a] -= Processor.Registers[a]
			return false
		},
	}},
	0x98: {label: "SBC A,B", value: 0x98, steps: []stepFunc{
		func() bool {
			CY := getCarryFlag()
			setZeroFlag((Processor.Registers[a] - (Processor.Registers[b] + CY)) == 0)
			setSubtractFlag(true)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) - (Processor.Registers[b] & 0xF) - CY) & 0x10) == 0x10)
			setCarryFlag((uint16(Processor.Registers[a]) < (uint16(Processor.Registers[b]) + uint16(CY))))
			Processor.Registers[a] = Processor.Registers[a] - (Processor.Registers[b] + CY)
			return false
		},
	}},
	0x99: {label: "SBC A,C", value: 0x99, steps: []stepFunc{
		func() bool {
			CY := getCarryFlag()
			setZeroFlag((Processor.Registers[a] - (Processor.Registers[c] + CY)) == 0)
			setSubtractFlag(true)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) - (Processor.Registers[c] & 0xF) - CY) & 0x10) == 0x10)
			setCarryFlag((uint16(Processor.Registers[a]) < (uint16(Processor.Registers[c]) + uint16(CY))))
			Processor.Registers[a] = Processor.Registers[a] - (Processor.Registers[c] + CY)
			return false
		},
	}},
	0x9A: {label: "SBC A,D", value: 0x9A, steps: []stepFunc{
		func() bool {
			CY := getCarryFlag()
			setZeroFlag((Processor.Registers[a] - (Processor.Registers[d] + CY)) == 0)
			setSubtractFlag(true)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) - (Processor.Registers[d] & 0xF) - CY) & 0x10) == 0x10)
			setCarryFlag((uint16(Processor.Registers[a]) < (uint16(Processor.Registers[d]) + uint16(CY))))
			Processor.Registers[a] = Processor.Registers[a] - (Processor.Registers[d] + CY)
			return false
		},
	}},
	0x9B: {label: "SBC A,E", value: 0x9B, steps: []stepFunc{
		func() bool {
			CY := getCarryFlag()
			setZeroFlag((Processor.Registers[a] - (Processor.Registers[e] + CY)) == 0)
			setSubtractFlag(true)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) - (Processor.Registers[e] & 0xF) - CY) & 0x10) == 0x10)
			setCarryFlag((uint16(Processor.Registers[a]) < (uint16(Processor.Registers[e]) + uint16(CY))))
			Processor.Registers[a] = Processor.Registers[a] - (Processor.Registers[e] + CY)
			return false
		},
	}},
	0x9C: {label: "SBC A,H", value: 0x9C, steps: []stepFunc{
		func() bool {
			CY := getCarryFlag()
			setZeroFlag((Processor.Registers[a] - (Processor.Registers[h] + CY)) == 0)
			setSubtractFlag(true)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) - (Processor.Registers[h] & 0xF) - CY) & 0x10) == 0x10)
			setCarryFlag((uint16(Processor.Registers[a]) < (uint16(Processor.Registers[h]) + uint16(CY))))
			Processor.Registers[a] = Processor.Registers[a] - (Processor.Registers[h] + CY)
			return false
		},
	}},
	0x9D: {label: "SBC A,L", value: 0x9D, steps: []stepFunc{
		func() bool {
			CY := getCarryFlag()
			setZeroFlag((Processor.Registers[a] - (Processor.Registers[l] + CY)) == 0)
			setSubtractFlag(true)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) - (Processor.Registers[l] & 0xF) - CY) & 0x10) == 0x10)
			setCarryFlag((uint16(Processor.Registers[a]) < (uint16(Processor.Registers[l]) + uint16(CY))))
			Processor.Registers[a] = Processor.Registers[a] - (Processor.Registers[l] + CY)
			return false
		},
	}},
	0x9E: {label: "SBC A,(HL)", value: 0x9E, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			CY := getCarryFlag()
			setZeroFlag((Processor.Registers[a] - (memory.CPU.Read(getRegisterHL()) + CY)) == 0)
			setSubtractFlag(true)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) - (memory.CPU.Read(getRegisterHL()) & 0xF) - CY) & 0x10) == 0x10)
			setCarryFlag((uint16(Processor.Registers[a]) < (uint16(memory.CPU.Read(getRegisterHL())) + uint16(CY))))
			Processor.Registers[a] = Processor.Registers[a] - (memory.CPU.Read(getRegisterHL()) + CY)
			return false
		},
	}},
	0x9F: {label: "SBC A,A", value: 0x9F, steps: []stepFunc{
		func() bool {
			CY := getCarryFlag()
			setZeroFlag((Processor.Registers[a] - (Processor.Registers[a] + CY)) == 0)
			setSubtractFlag(true)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) - (Processor.Registers[a] & 0xF) - CY) & 0x10) == 0x10)
			setCarryFlag((uint16(Processor.Registers[a]) < (uint16(Processor.Registers[a]) + uint16(CY))))
			Processor.Registers[a] = Processor.Registers[a] - (Processor.Registers[a] + CY)
			return false
		},
	}},
	0xA0: {label: "AND A,B", value: 0xA0, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] &= Processor.Registers[b]
			setZeroFlag(Processor.Registers[a] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			setCarryFlag(false)
			return false
		},
	}},
	0xA1: {label: "AND A,C", value: 0xA1, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] &= Processor.Registers[c]
			setZeroFlag(Processor.Registers[a] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			setCarryFlag(false)
			return false
		},
	}},
	0xA2: {label: "AND A,D", value: 0xA2, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] &= Processor.Registers[d]
			setZeroFlag(Processor.Registers[a] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			setCarryFlag(false)
			return false
		},
	}},
	0xA3: {label: "AND A,E", value: 0xA3, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] &= Processor.Registers[e]
			setZeroFlag(Processor.Registers[a] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			setCarryFlag(false)
			return false
		},
	}},
	0xA4: {label: "AND A,H", value: 0xA4, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] &= Processor.Registers[h]
			setZeroFlag(Processor.Registers[a] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			setCarryFlag(false)
			return false
		},
	}},
	0xA5: {label: "AND A,L", value: 0xA5, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] &= Processor.Registers[l]
			setZeroFlag(Processor.Registers[a] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			setCarryFlag(false)
			return false
		},
	}},
	0xA6: {label: "AND A,(HL)", value: 0xA6, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.Registers[a] &= memory.CPU.Read(getRegisterHL())
			setZeroFlag(Processor.Registers[a] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			setCarryFlag(false)
			return false
		},
	}},
	0xA7: {label: "AND A,A", value: 0xA7, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] &= Processor.Registers[a]
			setZeroFlag(Processor.Registers[a] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			setCarryFlag(false)
			return false
		},
	}},
	0xA8: {label: "XOR A,B", value: 0xA8, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] ^= Processor.Registers[b]
			setZeroFlag(Processor.Registers[a] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			setCarryFlag(false)
			return false
		},
	}},
	0xA9: {label: "XOR A,C", value: 0xA9, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] ^= Processor.Registers[c]
			setZeroFlag(Processor.Registers[a] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			setCarryFlag(false)
			return false
		},
	}},
	0xAA: {label: "XOR A,D", value: 0xAA, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] ^= Processor.Registers[d]
			setZeroFlag(Processor.Registers[a] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			setCarryFlag(false)
			return false
		},
	}},
	0xAB: {label: "XOR A,E", value: 0xAB, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] ^= Processor.Registers[e]
			setZeroFlag(Processor.Registers[a] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			setCarryFlag(false)
			return false
		},
	}},
	0xAC: {label: "XOR A,H", value: 0xAC, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] ^= Processor.Registers[h]
			setZeroFlag(Processor.Registers[a] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			setCarryFlag(false)
			return false
		},
	}},
	0xAD: {label: "XOR A,L", value: 0xAD, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] ^= Processor.Registers[l]
			setZeroFlag(Processor.Registers[a] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			setCarryFlag(false)
			return false
		},
	}},
	0xAE: {label: "XOR A,(HL)", value: 0xAE, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.Registers[a] ^= memory.CPU.Read(getRegisterHL())
			setZeroFlag(Processor.Registers[a] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			setCarryFlag(false)
			return false
		},
	}},
	0xAF: {label: "XOR A,A", value: 0xAF, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] ^= Processor.Registers[a]
			setZeroFlag(Processor.Registers[a] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			setCarryFlag(false)
			return false
		},
	}},
	0xB0: {label: "OR A,B", value: 0xB0, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] |= Processor.Registers[b]
			setZeroFlag(Processor.Registers[a] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			setCarryFlag(false)
			return false
		},
	}},
	0xB1: {label: "OR A,C", value: 0xB1, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] |= Processor.Registers[c]
			setZeroFlag(Processor.Registers[a] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			setCarryFlag(false)
			return false
		},
	}},
	0xB2: {label: "OR A,D", value: 0xB2, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] |= Processor.Registers[d]
			setZeroFlag(Processor.Registers[a] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			setCarryFlag(false)
			return false
		},
	}},
	0xB3: {label: "OR A,E", value: 0xB3, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] |= Processor.Registers[e]
			setZeroFlag(Processor.Registers[a] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			setCarryFlag(false)
			return false
		},
	}},
	0xB4: {label: "OR A,H", value: 0xB4, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] |= Processor.Registers[h]
			setZeroFlag(Processor.Registers[a] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			setCarryFlag(false)
			return false
		},
	}},
	0xB5: {label: "OR A,L", value: 0xB5, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] |= Processor.Registers[l]
			setZeroFlag(Processor.Registers[a] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			setCarryFlag(false)
			return false
		},
	}},
	0xB6: {label: "OR A,(HL)", value: 0xB6, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.Registers[a] |= memory.CPU.Read(getRegisterHL())
			setZeroFlag(Processor.Registers[a] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			setCarryFlag(false)
			return false
		},
	}},
	0xB7: {label: "OR A,A", value: 0xB7, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] |= Processor.Registers[a]
			setZeroFlag(Processor.Registers[a] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			setCarryFlag(false)
			return false
		},
	}},
	0xB8: {label: "CP A,B", value: 0xB8, steps: []stepFunc{
		func() bool {
			setZeroFlag((Processor.Registers[a] - Processor.Registers[b]) == 0)
			setSubtractFlag(true)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) - (Processor.Registers[b] & 0xF)) & 0x10) == 0x10)
			setCarryFlag(Processor.Registers[a] < Processor.Registers[b])
			return false
		},
	}},
	0xB9: {label: "CP A,C", value: 0xB9, steps: []stepFunc{
		func() bool {
			setZeroFlag((Processor.Registers[a] - Processor.Registers[c]) == 0)
			setSubtractFlag(true)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) - (Processor.Registers[c] & 0xF)) & 0x10) == 0x10)
			setCarryFlag(Processor.Registers[a] < Processor.Registers[c])
			return false
		},
	}},
	0xBA: {label: "CP A,D", value: 0xBA, steps: []stepFunc{
		func() bool {
			setZeroFlag((Processor.Registers[a] - Processor.Registers[d]) == 0)
			setSubtractFlag(true)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) - (Processor.Registers[d] & 0xF)) & 0x10) == 0x10)
			setCarryFlag(Processor.Registers[a] < Processor.Registers[d])
			return false
		},
	}},
	0xBB: {label: "CP A,E", value: 0xBB, steps: []stepFunc{
		func() bool {
			setZeroFlag((Processor.Registers[a] - Processor.Registers[e]) == 0)
			setSubtractFlag(true)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) - (Processor.Registers[e] & 0xF)) & 0x10) == 0x10)
			setCarryFlag(Processor.Registers[a] < Processor.Registers[e])
			return false
		},
	}},
	0xBC: {label: "CP A,H", value: 0xBC, steps: []stepFunc{
		func() bool {
			setZeroFlag((Processor.Registers[a] - Processor.Registers[h]) == 0)
			setSubtractFlag(true)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) - (Processor.Registers[h] & 0xF)) & 0x10) == 0x10)
			setCarryFlag(Processor.Registers[a] < Processor.Registers[h])
			return false
		},
	}},
	0xBD: {label: "CP A,L", value: 0xBD, steps: []stepFunc{
		func() bool {
			setZeroFlag((Processor.Registers[a] - Processor.Registers[l]) == 0)
			setSubtractFlag(true)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) - (Processor.Registers[l] & 0xF)) & 0x10) == 0x10)
			setCarryFlag(Processor.Registers[a] < Processor.Registers[l])
			return false
		},
	}},
	0xBE: {label: "CP A,(HL)", value: 0xBE, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			setZeroFlag((Processor.Registers[a] - memory.CPU.Read(getRegisterHL())) == 0)
			setSubtractFlag(true)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) - (memory.CPU.Read(getRegisterHL()) & 0xF)) & 0x10) == 0x10)
			setCarryFlag(Processor.Registers[a] < memory.CPU.Read(getRegisterHL()))
			return false
		},
	}},
	0xBF: {label: "CP A,A", value: 0xBF, steps: []stepFunc{
		func() bool {
			setZeroFlag((Processor.Registers[a] - Processor.Registers[a]) == 0)
			setSubtractFlag(true)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) - (Processor.Registers[a] & 0xF)) & 0x10) == 0x10)
			setCarryFlag(Processor.Registers[a] < Processor.Registers[a])
			return false
		},
	}},
	0xC0: {label: "RET NZ", value: 0xC0, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			return getZeroFlag() == 0
		},
		func() bool {
			Processor.State.buffer[0] = memory.CPU.Read(Processor.SP)
			Processor.SP++
			return true
		},
		func() bool {
			Processor.State.buffer[1] = memory.CPU.Read(Processor.SP)
			Processor.SP++
			return true
		},
		func() bool {
			Processor.PC = (uint16(Processor.State.buffer[1]) << 8) | uint16(Processor.State.buffer[0])
			return false
		},
	}},
	0xC1: {label: "POP BC", value: 0xC1, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.Registers[c] = memory.CPU.Read(Processor.SP)
			Processor.SP++
			return true
		},
		func() bool {
			Processor.Registers[b] = memory.CPU.Read(Processor.SP)
			Processor.SP++
			return false
		},
	}},
	0xC2: {label: "JP NZ,u16", value: 0xC2, steps: []stepFunc{
		func() bool {
			Processor.State.buffer[0] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return true
		},
		func() bool {
			Processor.State.buffer[1] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return true
		},
		func() bool {
			return getZeroFlag() == 0
		},
		func() bool {
			Processor.PC = (uint16(Processor.State.buffer[1]) << 8) | uint16(Processor.State.buffer[0])
			return false
		},
	}},
	0xC3: {label: "JP u16", value: 0xC3, steps: []stepFunc{
		func() bool {
			Processor.State.buffer[0] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return true
		},
		func() bool {
			Processor.State.buffer[1] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return true
		},
		func() bool {
			Processor.PC = (uint16(Processor.State.buffer[1]) << 8) | uint16(Processor.State.buffer[0])
			return true
		},
		func() bool { // "internal branch decision"
			return false
		},
	}},
	0xC4: {label: "CALL NZ,u16", value: 0xC4, steps: []stepFunc{
		func() bool {
			Processor.State.buffer[0] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return true
		},
		func() bool {
			Processor.State.buffer[1] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return true
		},
		func() bool {
			return getZeroFlag() == 0
		},
		func() bool {
			Processor.SP--
			memory.CPU.Write(Processor.SP, byte((0xFF00&Processor.PC)>>8))
			return true
		},
		func() bool {
			Processor.SP--
			memory.CPU.Write(Processor.SP, byte(0x00FF&Processor.PC))
			return true
		},
		func() bool {
			Processor.PC = (uint16(Processor.State.buffer[1]) << 8) | uint16(Processor.State.buffer[0])
			return false
		},
	}},
	0xC5: {label: "PUSH BC", value: 0xC5, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool { // "internal"
			return true
		},
		func() bool {
			Processor.SP--
			memory.CPU.Write(Processor.SP, Processor.Registers[b])
			return true
		},
		func() bool {
			Processor.SP--
			memory.CPU.Write(Processor.SP, Processor.Registers[c])
			return false
		},
	}},
	0xC6: {label: "ADD A,u8", value: 0xC6, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			setZeroFlag((Processor.Registers[a] + memory.CPU.Read(Processor.PC)) == 0)
			setSubtractFlag(false)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) + (memory.CPU.Read(Processor.PC) & 0xF)) & 0x10) == 0x10)
			setCarryFlag((uint16(Processor.Registers[a]) + uint16(memory.CPU.Read(Processor.PC))) > 0xFF)
			Processor.Registers[a] += memory.CPU.Read(Processor.PC)
			Processor.PC++
			return false
		},
	}},
	0xC7: {label: "RST 00h", value: 0xC7, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.SP--
			memory.CPU.Write(Processor.SP, byte((0xFF00&Processor.PC)>>8))
			return true
		},
		func() bool {
			Processor.SP--
			memory.CPU.Write(Processor.SP, byte(0x00FF&Processor.PC))
			return true
		},
		func() bool {
			Processor.PC = 0x0000
			return false
		},
	}},
	0xC8: {label: "RET Z", value: 0xC8, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			return getZeroFlag() == 1
		},
		func() bool {
			Processor.State.buffer[0] = memory.CPU.Read(Processor.SP)
			Processor.SP++
			return true
		},
		func() bool {
			Processor.State.buffer[1] = memory.CPU.Read(Processor.SP)
			Processor.SP++
			return true
		},
		func() bool {
			Processor.PC = (uint16(Processor.State.buffer[1]) << 8) | uint16(Processor.State.buffer[0])
			return false
		},
	}},
	0xC9: {label: "RET", value: 0xC9, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.State.buffer[0] = memory.CPU.Read(Processor.SP)
			Processor.SP++
			return true
		},
		func() bool {
			Processor.State.buffer[1] = memory.CPU.Read(Processor.SP)
			Processor.SP++
			return true
		},
		func() bool {
			Processor.PC = (uint16(Processor.State.buffer[1]) << 8) | uint16(Processor.State.buffer[0])
			return false
		},
	}},
	0xCA: {label: "JP Z,u16", value: 0xCA, steps: []stepFunc{
		func() bool {
			Processor.State.buffer[0] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return true
		},
		func() bool {
			Processor.State.buffer[1] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return true
		},
		func() bool {
			return getZeroFlag() == 1
		},
		func() bool {
			Processor.PC = (uint16(Processor.State.buffer[1]) << 8) | uint16(Processor.State.buffer[0])
			return false
		},
	}},
	0xCC: {label: "CALL Z,u16", value: 0xCC, steps: []stepFunc{
		func() bool {
			Processor.State.buffer[0] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return true
		},
		func() bool {
			Processor.State.buffer[1] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return true
		},
		func() bool {
			return getZeroFlag() == 1
		},
		func() bool {
			Processor.SP--
			memory.CPU.Write(Processor.SP, byte((0xFF00&Processor.PC)>>8))
			return true
		},
		func() bool {
			Processor.SP--
			memory.CPU.Write(Processor.SP, byte(0x00FF&Processor.PC))
			return true
		},
		func() bool {
			Processor.PC = (uint16(Processor.State.buffer[1]) << 8) | uint16(Processor.State.buffer[0])
			return false
		},
	}},
	0xCD: {label: "CALL u16", value: 0xCD, steps: []stepFunc{
		func() bool {
			Processor.State.buffer[0] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return true
		},
		func() bool {
			Processor.State.buffer[1] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return true
		},
		func() bool { // "internal branch decision"
			return true
		},
		func() bool {
			Processor.SP--
			memory.CPU.Write(Processor.SP, byte((0xFF00&Processor.PC)>>8))
			return true
		},
		func() bool {
			Processor.SP--
			memory.CPU.Write(Processor.SP, byte(0x00FF&Processor.PC))
			return true
		},
		func() bool {
			Processor.PC = (uint16(Processor.State.buffer[1]) << 8) | uint16(Processor.State.buffer[0])
			return false
		},
	}},
	0xCE: {label: "ADC A,u8", value: 0xCE, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			CY := getCarryFlag()
			setZeroFlag((Processor.Registers[a] + memory.CPU.Read(Processor.PC) + CY) == 0)
			setSubtractFlag(false)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) + (memory.CPU.Read(Processor.PC) & 0xF) + CY) & 0x10) == 0x10)
			setCarryFlag((uint16(Processor.Registers[a]) + uint16(memory.CPU.Read(Processor.PC)) + uint16(CY)) > 0xFF)
			Processor.Registers[a] += (memory.CPU.Read(Processor.PC) + CY)
			Processor.PC++
			return false
		},
	}},
	0xCF: {label: "RST 08h", value: 0xCF, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.SP--
			memory.CPU.Write(Processor.SP, byte((0xFF00&Processor.PC)>>8))
			return true
		},
		func() bool {
			Processor.SP--
			memory.CPU.Write(Processor.SP, byte(0x00FF&Processor.PC))
			return true
		},
		func() bool {
			Processor.PC = 0x0008
			return false
		},
	}},
	0xD0: {label: "RET NC", value: 0xD0, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			return getCarryFlag() == 0
		},
		func() bool {
			Processor.State.buffer[0] = memory.CPU.Read(Processor.SP)
			Processor.SP++
			return true
		},
		func() bool {
			Processor.State.buffer[1] = memory.CPU.Read(Processor.SP)
			Processor.SP++
			return true
		},
		func() bool {
			Processor.PC = (uint16(Processor.State.buffer[1]) << 8) | uint16(Processor.State.buffer[0])
			return false
		},
	}},
	0xD1: {label: "POP DE", value: 0xD1, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.Registers[e] = memory.CPU.Read(Processor.SP)
			Processor.SP++
			return true
		},
		func() bool {
			Processor.Registers[d] = memory.CPU.Read(Processor.SP)
			Processor.SP++
			return false
		},
	}},
	0xD2: {label: "JP NC,u16", value: 0xD2, steps: []stepFunc{
		func() bool {
			Processor.State.buffer[0] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return true
		},
		func() bool {
			Processor.State.buffer[1] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return true
		},
		func() bool {
			return getCarryFlag() == 0
		},
		func() bool {
			Processor.PC = (uint16(Processor.State.buffer[1]) << 8) | uint16(Processor.State.buffer[0])
			return false
		},
	}},
	0xD4: {label: "CALL NC,u16", value: 0xD4, steps: []stepFunc{
		func() bool {
			Processor.State.buffer[0] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return true
		},
		func() bool {
			Processor.State.buffer[1] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return true
		},
		func() bool {
			return getCarryFlag() == 0
		},
		func() bool {
			Processor.SP--
			memory.CPU.Write(Processor.SP, byte((0xFF00&Processor.PC)>>8))
			return true
		},
		func() bool {
			Processor.SP--
			memory.CPU.Write(Processor.SP, byte(0x00FF&Processor.PC))
			return true
		},
		func() bool {
			Processor.PC = (uint16(Processor.State.buffer[1]) << 8) | uint16(Processor.State.buffer[0])
			return false
		},
	}},
	0xD5: {label: "PUSH DE", value: 0xD5, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool { // "internal"
			return true
		},
		func() bool {
			Processor.SP--
			memory.CPU.Write(Processor.SP, Processor.Registers[d])
			return true
		},
		func() bool {
			Processor.SP--
			memory.CPU.Write(Processor.SP, Processor.Registers[e])
			return false
		},
	}},
	0xD6: {label: "SUB A,u8", value: 0xD6, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			setZeroFlag((Processor.Registers[a] - memory.CPU.Read(Processor.PC)) == 0)
			setSubtractFlag(true)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) - (memory.CPU.Read(Processor.PC) & 0xF)) & 0x10) == 0x10)
			setCarryFlag(Processor.Registers[a] < memory.CPU.Read(Processor.PC))
			Processor.Registers[a] -= memory.CPU.Read(Processor.PC)
			Processor.PC++
			return false
		},
	}},
	0xD7: {label: "RST 10h", value: 0xD7, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.SP--
			memory.CPU.Write(Processor.SP, byte((0xFF00&Processor.PC)>>8))
			return true
		},
		func() bool {
			Processor.SP--
			memory.CPU.Write(Processor.SP, byte(0x00FF&Processor.PC))
			return true
		},
		func() bool {
			Processor.PC = 0x0010
			return false
		},
	}},
	0xD8: {label: "RET C", value: 0xD8, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			return getCarryFlag() == 1
		},
		func() bool {
			Processor.State.buffer[0] = memory.CPU.Read(Processor.SP)
			Processor.SP++
			return true
		},
		func() bool {
			Processor.State.buffer[1] = memory.CPU.Read(Processor.SP)
			Processor.SP++
			return true
		},
		func() bool {
			Processor.PC = (uint16(Processor.State.buffer[1]) << 8) | uint16(Processor.State.buffer[0])
			return false
		},
	}},
	0xD9: {label: "RETI", value: 0xD9, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.State.buffer[0] = memory.CPU.Read(Processor.SP)
			Processor.SP++
			return true
		},
		func() bool {
			Processor.State.buffer[1] = memory.CPU.Read(Processor.SP)
			Processor.SP++
			return true
		},
		func() bool {
			Processor.PC = (uint16(Processor.State.buffer[1]) << 8) | uint16(Processor.State.buffer[0])
			Processor.ShouldEnableInterrupt++
			return false
		},
	}},
	0xDA: {label: "JP C,u16", value: 0xDA, steps: []stepFunc{
		func() bool {
			Processor.State.buffer[0] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return true
		},
		func() bool {
			Processor.State.buffer[1] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return true
		},
		func() bool {
			return getCarryFlag() == 1
		},
		func() bool {
			Processor.PC = (uint16(Processor.State.buffer[1]) << 8) | uint16(Processor.State.buffer[0])
			return false
		},
	}},
	0xDC: {label: "CALL C,u16", value: 0xDC, steps: []stepFunc{
		func() bool {
			Processor.State.buffer[0] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return true
		},
		func() bool {
			Processor.State.buffer[1] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return true
		},
		func() bool {
			return getCarryFlag() == 1
		},
		func() bool {
			Processor.SP--
			memory.CPU.Write(Processor.SP, byte((0xFF00&Processor.PC)>>8))
			return true
		},
		func() bool {
			Processor.SP--
			memory.CPU.Write(Processor.SP, byte(0x00FF&Processor.PC))
			return true
		},
		func() bool {
			Processor.PC = (uint16(Processor.State.buffer[1]) << 8) | uint16(Processor.State.buffer[0])
			return false
		},
	}},
	0xDE: {label: "SBC A,u8", value: 0xDE, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			CY := getCarryFlag()
			setZeroFlag((Processor.Registers[a] - (memory.CPU.Read(Processor.PC) + CY)) == 0)
			setSubtractFlag(true)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) - (memory.CPU.Read(Processor.PC) & 0xF) - CY) & 0x10) == 0x10)
			setCarryFlag((uint16(Processor.Registers[a]) < (uint16(memory.CPU.Read(Processor.PC)) + uint16(CY))))
			Processor.Registers[a] = Processor.Registers[a] - (memory.CPU.Read(Processor.PC) + CY)
			Processor.PC++
			return false
		},
	}},
	0xDF: {label: "RST 18h", value: 0xDF, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.SP--
			memory.CPU.Write(Processor.SP, byte((0xFF00&Processor.PC)>>8))
			return true
		},
		func() bool {
			Processor.SP--
			memory.CPU.Write(Processor.SP, byte(0x00FF&Processor.PC))
			return true
		},
		func() bool {
			Processor.PC = 0x0018
			return false
		},
	}},
	0xE0: {label: "LD (FF00+u8),A", value: 0xE0, steps: []stepFunc{ // NOTE: WRITE
		func() bool { // "fetch"
			return true
		},
		func() bool { // "read"
			return true
		},
		func() bool {
			memory.CPU.Write(0xFF00|(uint16(memory.CPU.Read(Processor.PC))&0x00FF), Processor.Registers[a])
			Processor.PC++
			return false
		},
	}},
	0xE1: {label: "POP HL", value: 0xE1, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.Registers[l] = memory.CPU.Read(Processor.SP)
			Processor.SP++
			return true
		},
		func() bool {
			Processor.Registers[h] = memory.CPU.Read(Processor.SP)
			Processor.SP++
			return false
		},
	}},
	0xE2: {label: "LD (FF00+C),A", value: 0xE2, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			memory.CPU.Write(0xFF00|(uint16(Processor.Registers[c])&0x00FF), Processor.Registers[a])
			return false
		},
	}},
	0xE5: {label: "PUSH HL", value: 0xE5, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool { // "internal"
			return true
		},
		func() bool {
			Processor.SP--
			memory.CPU.Write(Processor.SP, Processor.Registers[h])
			return true
		},
		func() bool {
			Processor.SP--
			memory.CPU.Write(Processor.SP, Processor.Registers[l])
			return false
		},
	}},
	0xE6: {label: "AND A,u8", value: 0xE6, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			setZeroFlag((Processor.Registers[a] & memory.CPU.Read(Processor.PC)) == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			setCarryFlag(false)
			Processor.Registers[a] &= memory.CPU.Read(Processor.PC)
			Processor.PC++
			return false
		},
	}},
	0xE7: {label: "RST 20h", value: 0xE7, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.SP--
			memory.CPU.Write(Processor.SP, byte((0xFF00&Processor.PC)>>8))
			return true
		},
		func() bool {
			Processor.SP--
			memory.CPU.Write(Processor.SP, byte(0x00FF&Processor.PC))
			return true
		},
		func() bool {
			Processor.PC = 0x0020
			return false
		},
	}},
	0xE8: {label: "ADD SP,i8", value: 0xE8, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool { // "read"
			return true
		},
		func() bool { // "internal"
			return true
		},
		func() bool {
			setZeroFlag(false)
			setSubtractFlag(false)
			setHalfCarryFlag((byte(Processor.SP&0x0F) + (byte(int8(memory.CPU.Read(Processor.PC))) & 0x0F)) > 0x0F)
			setCarryFlag(((Processor.SP & 0xFF) + (uint16(int8(memory.CPU.Read(Processor.PC))) & 0xFF)) > 0xFF)
			Processor.SP = Processor.SP + uint16(int8(memory.CPU.Read(Processor.PC)))
			Processor.PC++
			return false
		},
	}},
	0xE9: {label: "JP HL", value: 0xE9, steps: []stepFunc{
		func() bool {
			Processor.PC = getRegisterHL()
			return false
		},
	}},
	0xEA: {label: "LD (u16),A", value: 0xEA, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.State.buffer[0] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return true
		},
		func() bool {
			Processor.State.buffer[1] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return true
		},
		func() bool {
			memory.CPU.Write((uint16(Processor.State.buffer[1])<<8)|uint16(Processor.State.buffer[0]), Processor.Registers[a])
			return false
		},
	}},
	0xEE: {label: "XOR A,u8", value: 0xEE, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.Registers[a] ^= memory.CPU.Read(Processor.PC)
			setZeroFlag(Processor.Registers[a] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			setCarryFlag(false)
			Processor.PC++
			return false
		},
	}},
	0xEF: {label: "RST 28H", value: 0xEF, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.SP--
			memory.CPU.Write(Processor.SP, byte((0xFF00&Processor.PC)>>8))
			return true
		},
		func() bool {
			Processor.SP--
			memory.CPU.Write(Processor.SP, byte(0x00FF&Processor.PC))
			return true
		},
		func() bool {
			Processor.PC = 0x0028
			return false
		},
	}},
	0xF0: {label: "LD A,(FF00+u8)", value: 0xF0, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool { // "read"
			return true
		},
		func() bool {
			Processor.Registers[a] = memory.CPU.Read(0xFF00 | (uint16(memory.CPU.Read(Processor.PC)) & 0x00FF))
			Processor.PC++
			return false
		},
	}},
	0xF1: {label: "POP AF", value: 0xF1, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.Registers[f] = (memory.CPU.Read(Processor.SP) & 0xF0)
			Processor.SP++
			return true
		},
		func() bool {
			Processor.Registers[a] = memory.CPU.Read(Processor.SP)
			Processor.SP++
			return false
		},
	}},
	0xF2: {label: "LD A,(FF00+C)", value: 0xF2, steps: []stepFunc{
		func() bool {
			return true
		},
		func() bool {
			Processor.Registers[a] = memory.CPU.Read(0xFF00 | (uint16(Processor.Registers[c]) & 0x00FF))
			return false
		},
	}},
	0xF3: {label: "DI", value: 0xF3, steps: []stepFunc{
		func() bool {
			Processor.Ime = 0
			return false
		},
	}},
	0xF5: {label: "PUSH AF", value: 0xF5, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool { // "internal"
			return true
		},
		func() bool {
			Processor.SP--
			memory.CPU.Write(Processor.SP, Processor.Registers[a])
			return true
		},
		func() bool {
			Processor.SP--
			memory.CPU.Write(Processor.SP, Processor.Registers[f])
			return false
		},
	}},
	0xF6: {label: "OR A,u8", value: 0xF6, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.Registers[a] |= memory.CPU.Read(Processor.PC)
			setZeroFlag(Processor.Registers[a] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			setCarryFlag(false)
			Processor.PC++
			return false
		},
	}},
	0xF7: {label: "RST 30h", value: 0xF7, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.SP--
			memory.CPU.Write(Processor.SP, byte((0xFF00&Processor.PC)>>8))
			return true
		},
		func() bool {
			Processor.SP--
			memory.CPU.Write(Processor.SP, byte(0x00FF&Processor.PC))
			return true
		},
		func() bool {
			Processor.PC = 0x0030
			return false
		},
	}},
	0xF8: {label: "LD HL,SP+i8", value: 0xF8, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool { // "read"
			return true
		},
		func() bool {
			setZeroFlag(false)
			setSubtractFlag(false)
			setHalfCarryFlag((byte(Processor.SP&0x0F) + (byte(int8(memory.CPU.Read(Processor.PC))) & 0x0F)) > 0x0F)
			setCarryFlag(((Processor.SP & 0xFF) + (uint16(int8(memory.CPU.Read(Processor.PC))) & 0xFF)) > 0xFF)
			setRegisterHL(Processor.SP + uint16(int8(memory.CPU.Read(Processor.PC))))
			Processor.PC++
			return false
		},
	}},
	0xF9: {label: "LD SP,HL", value: 0xF9, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.SP = getRegisterHL()
			return false
		},
	}},
	0xFA: {label: "LD A,(u16)", value: 0xFA, steps: []stepFunc{
		func() bool { // fetch
			return true
		},
		func() bool {
			Processor.State.buffer[0] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return true
		},
		func() bool {
			Processor.State.buffer[1] = memory.CPU.Read(Processor.PC)
			Processor.PC++
			return true
		},
		func() bool {
			Processor.Registers[a] = memory.CPU.Read((uint16(Processor.State.buffer[1]) << 8) | uint16(Processor.State.buffer[0]))
			return false
		},
	}},
	0xFB: {label: "EI", value: 0xFB, steps: []stepFunc{
		func() bool {
			Processor.ShouldEnableInterrupt++
			return false
		},
	}},
	0xFE: {label: "CP A,u8", value: 0xFE, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			setZeroFlag((Processor.Registers[a] - memory.CPU.Read(Processor.PC)) == 0)
			setSubtractFlag(true)
			setHalfCarryFlag((((Processor.Registers[a] & 0xF) - (memory.CPU.Read(Processor.PC) & 0xF)) & 0x10) == 0x10)
			setCarryFlag(Processor.Registers[a] < memory.CPU.Read(Processor.PC))
			Processor.PC++
			return false
		},
	}},
	0xFF: {label: "RST 38h", value: 0xFF, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			Processor.SP--
			memory.CPU.Write(Processor.SP, byte((0xFF00&Processor.PC)>>8))
			return true
		},
		func() bool {
			Processor.SP--
			memory.CPU.Write(Processor.SP, byte(0x00FF&Processor.PC))
			return true
		},
		func() bool {
			Processor.PC = 0x0038
			return false
		},
	}},

	0xCB: {label: "<PREFIX>", value: 0xCB, isPrefix: true},
}
