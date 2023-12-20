package cpu

import "github.com/ysawyers/emufun/gameboy/core/memory"

var opcodeTablePrefix = map[byte]opcode{
	0x00: {label: "RLC B", value: 0x00, steps: []stepFunc{
		func() bool {
			Processor.Registers[b] = rotateLeftC(Processor.Registers[b])
			setZeroFlag(Processor.Registers[b] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x01: {label: "RLC C", value: 0x01, steps: []stepFunc{
		func() bool {
			Processor.Registers[c] = rotateLeftC(Processor.Registers[c])
			setZeroFlag(Processor.Registers[c] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x02: {label: "RLC D", value: 0x02, steps: []stepFunc{
		func() bool {
			Processor.Registers[d] = rotateLeftC(Processor.Registers[d])
			setZeroFlag(Processor.Registers[d] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x03: {label: "RLC E", value: 0x03, steps: []stepFunc{
		func() bool {
			Processor.Registers[e] = rotateLeftC(Processor.Registers[e])
			setZeroFlag(Processor.Registers[e] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x04: {label: "RLC H", value: 0x04, steps: []stepFunc{
		func() bool {
			Processor.Registers[h] = rotateLeftC(Processor.Registers[h])
			setZeroFlag(Processor.Registers[h] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x05: {label: "RLC L", value: 0x05, steps: []stepFunc{
		func() bool {
			Processor.Registers[l] = rotateLeftC(Processor.Registers[l])
			setZeroFlag(Processor.Registers[l] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x06: {label: "RLC (HL)", value: 0x06, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool { // "read"
			return true
		},
		func() bool {
			memory.CPU.Write(getRegisterHL(), rotateLeftC(memory.CPU.Read(getRegisterHL())))
			setZeroFlag(memory.CPU.Read(getRegisterHL()) == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x07: {label: "RLC A", value: 0x07, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] = rotateLeftC(Processor.Registers[a])
			setZeroFlag(Processor.Registers[a] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x08: {label: "RRC B", value: 0x08, steps: []stepFunc{
		func() bool {
			Processor.Registers[b] = rotateRightC(Processor.Registers[b])
			setZeroFlag(Processor.Registers[b] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x09: {label: "RRC C", value: 0x09, steps: []stepFunc{
		func() bool {
			Processor.Registers[c] = rotateRightC(Processor.Registers[c])
			setZeroFlag(Processor.Registers[c] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x0A: {label: "RRC D", value: 0x0A, steps: []stepFunc{
		func() bool {
			Processor.Registers[d] = rotateRightC(Processor.Registers[d])
			setZeroFlag(Processor.Registers[d] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x0B: {label: "RRC E", value: 0x0B, steps: []stepFunc{
		func() bool {
			Processor.Registers[e] = rotateRightC(Processor.Registers[e])
			setZeroFlag(Processor.Registers[e] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x0C: {label: "RRC H", value: 0x0C, steps: []stepFunc{
		func() bool {
			Processor.Registers[h] = rotateRightC(Processor.Registers[h])
			setZeroFlag(Processor.Registers[h] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x0D: {label: "RRC L", value: 0x0D, steps: []stepFunc{
		func() bool {
			Processor.Registers[l] = rotateRightC(Processor.Registers[l])
			setZeroFlag(Processor.Registers[l] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x0E: {label: "RRC (HL)", value: 0x0E, steps: []stepFunc{ // MIGHT HAVE FUCKED UP AT
		func() bool { // "fetch"
			return true
		},
		func() bool { // "read"
			return true
		},
		func() bool {
			memory.CPU.Write(getRegisterHL(), rotateRightC(memory.CPU.Read(getRegisterHL())))
			setZeroFlag(memory.CPU.Read(getRegisterHL()) == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x0F: {label: "RRC A", value: 0x0F, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] = rotateRightC(Processor.Registers[a])
			setZeroFlag(Processor.Registers[a] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x10: {label: "RL B", value: 0x10, steps: []stepFunc{
		func() bool {
			Processor.Registers[b] = rotateLeft(Processor.Registers[b])
			setZeroFlag(Processor.Registers[b] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x11: {label: "RL C", value: 0x11, steps: []stepFunc{
		func() bool {
			Processor.Registers[c] = rotateLeft(Processor.Registers[c])
			setZeroFlag(Processor.Registers[c] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x12: {label: "RL D", value: 0x12, steps: []stepFunc{
		func() bool {
			Processor.Registers[d] = rotateLeft(Processor.Registers[d])
			setZeroFlag(Processor.Registers[d] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x13: {label: "RL E", value: 0x13, steps: []stepFunc{
		func() bool {
			Processor.Registers[e] = rotateLeft(Processor.Registers[e])
			setZeroFlag(Processor.Registers[e] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x14: {label: "RL H", value: 0x14, steps: []stepFunc{
		func() bool {
			Processor.Registers[h] = rotateLeft(Processor.Registers[h])
			setZeroFlag(Processor.Registers[h] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x15: {label: "RL L", value: 0x15, steps: []stepFunc{
		func() bool {
			Processor.Registers[l] = rotateLeft(Processor.Registers[l])
			setZeroFlag(Processor.Registers[l] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x16: {label: "RL (HL)", value: 0x16, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool { // "read"
			return true
		},
		func() bool {
			memory.CPU.Write(getRegisterHL(), rotateLeft(memory.CPU.Read(getRegisterHL())))
			setZeroFlag(memory.CPU.Read(getRegisterHL()) == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x17: {label: "RL A", value: 0x17, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] = rotateLeft(Processor.Registers[a])
			setZeroFlag(Processor.Registers[a] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x18: {label: "RR B", value: 0x18, steps: []stepFunc{
		func() bool {
			Processor.Registers[b] = rotateRight(Processor.Registers[b])
			setZeroFlag(Processor.Registers[b] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x19: {label: "RR C", value: 0x19, steps: []stepFunc{
		func() bool {
			Processor.Registers[c] = rotateRight(Processor.Registers[c])
			setZeroFlag(Processor.Registers[c] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x1A: {label: "RR D", value: 0x1A, steps: []stepFunc{
		func() bool {
			Processor.Registers[d] = rotateRight(Processor.Registers[d])
			setZeroFlag(Processor.Registers[d] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x1B: {label: "RR E", value: 0x1B, steps: []stepFunc{
		func() bool {
			Processor.Registers[e] = rotateRight(Processor.Registers[e])
			setZeroFlag(Processor.Registers[e] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x1C: {label: "RR H", value: 0x1C, steps: []stepFunc{
		func() bool {
			Processor.Registers[h] = rotateRight(Processor.Registers[h])
			setZeroFlag(Processor.Registers[h] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x1D: {label: "RR L", value: 0x1D, steps: []stepFunc{
		func() bool {
			Processor.Registers[l] = rotateRight(Processor.Registers[l])
			setZeroFlag(Processor.Registers[l] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x1E: {label: "RR (HL)", value: 0x1E, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool { // "read"
			return true
		},
		func() bool {
			memory.CPU.Write(getRegisterHL(), rotateRight(memory.CPU.Read(getRegisterHL())))
			setZeroFlag(memory.CPU.Read(getRegisterHL()) == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x1F: {label: "RR A", value: 0x1F, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] = rotateRight(Processor.Registers[a])
			setZeroFlag(Processor.Registers[a] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x20: {label: "SLA B", value: 0x20, steps: []stepFunc{
		func() bool {
			Processor.Registers[b] = shiftLeft(Processor.Registers[b])
			setZeroFlag(Processor.Registers[b] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x21: {label: "SLA C", value: 0x21, steps: []stepFunc{
		func() bool {
			Processor.Registers[c] = shiftLeft(Processor.Registers[c])
			setZeroFlag(Processor.Registers[c] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x22: {label: "SLA D", value: 0x22, steps: []stepFunc{
		func() bool {
			Processor.Registers[d] = shiftLeft(Processor.Registers[d])
			setZeroFlag(Processor.Registers[d] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x23: {label: "SLA E", value: 0x23, steps: []stepFunc{
		func() bool {
			Processor.Registers[e] = shiftLeft(Processor.Registers[e])
			setZeroFlag(Processor.Registers[e] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},

	0x24: {label: "SLA H", value: 0x24, steps: []stepFunc{
		func() bool {
			Processor.Registers[h] = shiftLeft(Processor.Registers[h])
			setZeroFlag(Processor.Registers[h] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x25: {label: "SLA L", value: 0x25, steps: []stepFunc{
		func() bool {
			Processor.Registers[l] = shiftLeft(Processor.Registers[l])
			setZeroFlag(Processor.Registers[l] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x26: {label: "SLA (HL)", value: 0x26, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool { // "read"
			return true
		},
		func() bool {
			memory.CPU.Write(getRegisterHL(), shiftLeft(memory.CPU.Read(getRegisterHL())))
			setZeroFlag(memory.CPU.Read(getRegisterHL()) == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x27: {label: "SLA A", value: 0x27, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] = shiftLeft(Processor.Registers[a])
			setZeroFlag(Processor.Registers[a] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x28: {label: "SRA B", value: 0x28, steps: []stepFunc{
		func() bool {
			Processor.Registers[b] = shiftRightA(Processor.Registers[b])
			setZeroFlag(Processor.Registers[b] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x29: {label: "SRA C", value: 0x29, steps: []stepFunc{
		func() bool {
			Processor.Registers[c] = shiftRightA(Processor.Registers[c])
			setZeroFlag(Processor.Registers[c] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x2A: {label: "SRA D", value: 0x2A, steps: []stepFunc{
		func() bool {
			Processor.Registers[d] = shiftRightA(Processor.Registers[d])
			setZeroFlag(Processor.Registers[d] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x2B: {label: "SRA E", value: 0x2B, steps: []stepFunc{
		func() bool {
			Processor.Registers[e] = shiftRightA(Processor.Registers[e])
			setZeroFlag(Processor.Registers[e] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x2C: {label: "SRA H", value: 0x2C, steps: []stepFunc{
		func() bool {
			Processor.Registers[h] = shiftRightA(Processor.Registers[h])
			setZeroFlag(Processor.Registers[h] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x2D: {label: "SRA L", value: 0x2D, steps: []stepFunc{
		func() bool {
			Processor.Registers[l] = shiftRightA(Processor.Registers[l])
			setZeroFlag(Processor.Registers[l] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x2E: {label: "SRA (HL)", value: 0x2E, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool { // "read"
			return true
		},
		func() bool {
			memory.CPU.Write(getRegisterHL(), shiftRightA(memory.CPU.Read(getRegisterHL())))
			setZeroFlag(memory.CPU.Read(getRegisterHL()) == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x2F: {label: "SRA A", value: 0x2F, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] = shiftRightA(Processor.Registers[a])
			setZeroFlag(Processor.Registers[a] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x30: {label: "SWAP B", value: 0x30, steps: []stepFunc{
		func() bool {
			Processor.Registers[b] = swap(Processor.Registers[b])
			setZeroFlag(Processor.Registers[b] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			setCarryFlag(false)
			return false
		},
	}},
	0x31: {label: "SWAP C", value: 0x31, steps: []stepFunc{
		func() bool {
			Processor.Registers[c] = swap(Processor.Registers[c])
			setZeroFlag(Processor.Registers[c] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			setCarryFlag(false)
			return false
		},
	}},
	0x32: {label: "SWAP D", value: 0x32, steps: []stepFunc{
		func() bool {
			Processor.Registers[d] = swap(Processor.Registers[d])
			setZeroFlag(Processor.Registers[d] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			setCarryFlag(false)
			return false
		},
	}},
	0x33: {label: "SWAP E", value: 0x33, steps: []stepFunc{
		func() bool {
			Processor.Registers[e] = swap(Processor.Registers[e])
			setZeroFlag(Processor.Registers[e] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			setCarryFlag(false)
			return false
		},
	}},
	0x34: {label: "SWAP H", value: 0x34, steps: []stepFunc{
		func() bool {
			Processor.Registers[h] = swap(Processor.Registers[h])
			setZeroFlag(Processor.Registers[h] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			setCarryFlag(false)
			return false
		},
	}},
	0x35: {label: "SWAP L", value: 0x35, steps: []stepFunc{
		func() bool {
			Processor.Registers[l] = swap(Processor.Registers[l])
			setZeroFlag(Processor.Registers[l] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			setCarryFlag(false)
			return false
		},
	}},
	0x36: {label: "SWAP (HL)", value: 0x36, steps: []stepFunc{
		func() bool {
			return true
		},
		func() bool {
			return true
		},
		func() bool {
			memory.CPU.Write(getRegisterHL(), swap(memory.CPU.Read(getRegisterHL())))
			setZeroFlag(memory.CPU.Read(getRegisterHL()) == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			setCarryFlag(false)
			return false
		},
	}},
	0x37: {label: "SWAP A", value: 0x37, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] = swap(Processor.Registers[a])
			setZeroFlag(Processor.Registers[a] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			setCarryFlag(false)
			return false
		},
	}},
	0x38: {label: "SRL B", value: 0x38, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			setCarryFlag((Processor.Registers[b] & 0x1) == 0x1)
			Processor.Registers[b] >>= 1
			setZeroFlag(Processor.Registers[b] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x39: {label: "SRL C", value: 0x39, steps: []stepFunc{
		func() bool {
			Processor.Registers[c] = shiftRightL(Processor.Registers[c])
			setZeroFlag(Processor.Registers[c] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x3A: {label: "SRL D", value: 0x3A, steps: []stepFunc{
		func() bool {
			Processor.Registers[d] = shiftRightL(Processor.Registers[d])
			setZeroFlag(Processor.Registers[d] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x3B: {label: "SRL E", value: 0x3B, steps: []stepFunc{
		func() bool {
			Processor.Registers[e] = shiftRightL(Processor.Registers[e])
			setZeroFlag(Processor.Registers[e] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x3C: {label: "SRL H", value: 0x3C, steps: []stepFunc{
		func() bool {
			Processor.Registers[h] = shiftRightL(Processor.Registers[h])
			setZeroFlag(Processor.Registers[h] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x3D: {label: "SRL L", value: 0x3D, steps: []stepFunc{
		func() bool {
			Processor.Registers[l] = shiftRightL(Processor.Registers[l])
			setZeroFlag(Processor.Registers[l] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x3E: {label: "SRL (HL)", value: 0x3E, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool { // "read"
			return true
		},
		func() bool {
			memory.CPU.Write(getRegisterHL(), shiftRightL(memory.CPU.Read(getRegisterHL())))
			setZeroFlag(memory.CPU.Read(getRegisterHL()) == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x3F: {label: "SRL A", value: 0x3F, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] = shiftRightL(Processor.Registers[a])
			setZeroFlag(Processor.Registers[a] == 0)
			setSubtractFlag(false)
			setHalfCarryFlag(false)
			return false
		},
	}},
	0x40: {label: "BIT 0,B", value: 0x40, steps: []stepFunc{
		func() bool {
			setZeroFlag(!((Processor.Registers[b] & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x41: {label: "BIT 0,C", value: 0x41, steps: []stepFunc{
		func() bool {
			setZeroFlag(!((Processor.Registers[c] & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x42: {label: "BIT 0,D", value: 0x42, steps: []stepFunc{
		func() bool {
			setZeroFlag(!((Processor.Registers[d] & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x43: {label: "BIT 0,E", value: 0x43, steps: []stepFunc{
		func() bool {
			setZeroFlag(!((Processor.Registers[e] & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x44: {label: "BIT 0,H", value: 0x44, steps: []stepFunc{
		func() bool {
			setZeroFlag(!((Processor.Registers[h] & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x45: {label: "BIT 0,L", value: 0x45, steps: []stepFunc{
		func() bool {
			setZeroFlag(!((Processor.Registers[l] & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x46: {label: "BIT 0,(HL)", value: 0x46, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			setZeroFlag(!((memory.CPU.Read(getRegisterHL()) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x47: {label: "BIT 0,A", value: 0x47, steps: []stepFunc{
		func() bool {
			setZeroFlag(!((Processor.Registers[a] & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x48: {label: "BIT 1,B", value: 0x48, steps: []stepFunc{
		func() bool {
			setZeroFlag(!((Processor.Registers[b] & 0x2) == 0x2))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x49: {label: "BIT 1,C", value: 0x49, steps: []stepFunc{
		func() bool {
			setZeroFlag(!((Processor.Registers[c] & 0x2) == 0x2))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x4A: {label: "BIT 1,D", value: 0x4A, steps: []stepFunc{
		func() bool {
			setZeroFlag(!((Processor.Registers[d] & 0x2) == 0x2))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x4B: {label: "BIT 1,E", value: 0x4B, steps: []stepFunc{
		func() bool {
			setZeroFlag(!((Processor.Registers[e] & 0x2) == 0x2))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x4C: {label: "BIT 1,H", value: 0x4C, steps: []stepFunc{
		func() bool {
			setZeroFlag(!((Processor.Registers[h] & 0x2) == 0x2))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x4D: {label: "BIT 1,L", value: 0x4D, steps: []stepFunc{
		func() bool {
			setZeroFlag(!((Processor.Registers[l] & 0x2) == 0x2))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x4E: {label: "BIT 1,(HL)", value: 0x4E, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			setZeroFlag(!(((memory.CPU.Read(getRegisterHL()) >> 1) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x4F: {label: "BIT 1,A", value: 0x4F, steps: []stepFunc{
		func() bool {
			setZeroFlag(!((Processor.Registers[a] & 0x2) == 0x2))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x50: {label: "BIT 2,B", value: 0x50, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[b] >> 2) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x51: {label: "BIT 2,C", value: 0x51, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[c] >> 2) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x52: {label: "BIT 2,D", value: 0x52, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[d] >> 2) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x53: {label: "BIT 2,E", value: 0x53, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[e] >> 2) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x54: {label: "BIT 2,H", value: 0x54, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[h] >> 2) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x55: {label: "BIT 2,L", value: 0x55, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[l] >> 2) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x56: {label: "BIT 2,(HL)", value: 0x56, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			setZeroFlag(!(((memory.CPU.Read(getRegisterHL()) >> 2) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x57: {label: "BIT 2,A", value: 0x57, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[a] >> 2) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x58: {label: "BIT 3,B", value: 0x58, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[b] >> 3) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x59: {label: "BIT 3,C", value: 0x59, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[c] >> 3) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x5A: {label: "BIT 3,D", value: 0x5A, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[d] >> 3) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x5B: {label: "BIT 3,E", value: 0x5B, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[e] >> 3) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x5C: {label: "BIT 3,H", value: 0x5C, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[h] >> 3) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x5D: {label: "BIT 3,L", value: 0x5D, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[l] >> 3) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x5E: {label: "BIT 3,(HL)", value: 0x5E, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			setZeroFlag(!(((memory.CPU.Read(getRegisterHL()) >> 3) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x5F: {label: "BIT 3,A", value: 0x5F, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[a] >> 3) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x60: {label: "BIT 4,B", value: 0x60, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[b] >> 4) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x61: {label: "BIT 4,C", value: 0x61, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[c] >> 4) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x62: {label: "BIT 4,D", value: 0x62, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[d] >> 4) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x63: {label: "BIT 4,E", value: 0x63, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[e] >> 4) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x64: {label: "BIT 4,H", value: 0x64, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[h] >> 4) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x65: {label: "BIT 4,L", value: 0x65, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[l] >> 4) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x66: {label: "BIT 4,(HL)", value: 0x66, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			setZeroFlag(!(((memory.CPU.Read(getRegisterHL()) >> 4) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x67: {label: "BIT 4,A", value: 0x67, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[a] >> 4) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x68: {label: "BIT 5,B", value: 0x68, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[b] >> 5) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x69: {label: "BIT 5,C", value: 0x69, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[c] >> 5) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x6A: {label: "BIT 5,D", value: 0x6A, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[d] >> 5) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x6B: {label: "BIT 5,E", value: 0x6B, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[e] >> 5) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x6C: {label: "BIT 5,H", value: 0x6C, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[h] >> 5) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x6D: {label: "BIT 5,L", value: 0x6D, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[l] >> 5) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x6E: {label: "BIT 5,(HL)", value: 0x6E, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			setZeroFlag(!(((memory.CPU.Read(getRegisterHL()) >> 5) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x6F: {label: "BIT 5,A", value: 0x6F, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[a] >> 5) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x70: {label: "BIT 6,B", value: 0x70, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[b] >> 6) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x71: {label: "BIT 6,C", value: 0x71, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[c] >> 6) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x72: {label: "BIT 6,D", value: 0x72, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[d] >> 6) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x73: {label: "BIT 6,E", value: 0x73, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[e] >> 6) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x74: {label: "BIT 6,H", value: 0x74, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[h] >> 6) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x75: {label: "BIT 6,L", value: 0x75, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[l] >> 6) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x76: {label: "BIT 6,(HL)", value: 0x76, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			setZeroFlag(!(((memory.CPU.Read(getRegisterHL()) >> 6) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x77: {label: "BIT 6,A", value: 0x77, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[a] >> 6) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x78: {label: "BIT 7,B", value: 0x78, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[b] >> 7) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x79: {label: "BIT 7,C", value: 0x79, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[c] >> 7) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x7A: {label: "BIT 7,D", value: 0x7A, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[d] >> 7) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x7B: {label: "BIT 7,E", value: 0x7B, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[e] >> 7) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x7C: {label: "BIT 7,H", value: 0x7C, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[h] >> 7) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x7D: {label: "BIT 7,L", value: 0x7D, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[l] >> 7) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x7E: {label: "BIT 7,(HL)", value: 0x7E, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool {
			setZeroFlag(!(((memory.CPU.Read(getRegisterHL()) >> 7) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x7F: {label: "BIT 7,A", value: 0x7F, steps: []stepFunc{
		func() bool {
			setZeroFlag(!(((Processor.Registers[a] >> 7) & 0x1) == 0x1))
			setSubtractFlag(false)
			setHalfCarryFlag(true)
			return false
		},
	}},
	0x80: {label: "RES 0,B", value: 0x80, steps: []stepFunc{
		func() bool {
			Processor.Registers[b] &= 0b11111110
			return false
		},
	}},
	0x81: {label: "RES 0,C", value: 0x81, steps: []stepFunc{
		func() bool {
			Processor.Registers[c] &= 0b11111110
			return false
		},
	}},
	0x82: {label: "RES 0,D", value: 0x82, steps: []stepFunc{
		func() bool {
			Processor.Registers[d] &= 0b11111110
			return false
		},
	}},
	0x83: {label: "RES 0,E", value: 0x83, steps: []stepFunc{
		func() bool {
			Processor.Registers[e] &= 0b11111110
			return false
		},
	}},
	0x84: {label: "RES 0,H", value: 0x84, steps: []stepFunc{
		func() bool {
			Processor.Registers[h] &= 0b11111110
			return false
		},
	}},
	0x85: {label: "RES 0,L", value: 0x85, steps: []stepFunc{
		func() bool {
			Processor.Registers[l] &= 0b11111110
			return false
		},
	}},
	0x86: {label: "RES 0,(HL)", value: 0x86, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool { // "read"
			return true
		},
		func() bool {
			memory.CPU.Write(getRegisterHL(), memory.CPU.Read(getRegisterHL())&0b11111110)
			return false
		},
	}},
	0x87: {label: "RES 0,A", value: 0x87, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] &= 0b11111110
			return false
		},
	}},
	0x88: {label: "RES 1,B", value: 0x88, steps: []stepFunc{
		func() bool {
			Processor.Registers[b] &= 0b11111101
			return false
		},
	}},
	0x89: {label: "RES 1,C", value: 0x89, steps: []stepFunc{
		func() bool {
			Processor.Registers[c] &= 0b11111101
			return false
		},
	}},
	0x8A: {label: "RES 1,D", value: 0x8A, steps: []stepFunc{
		func() bool {
			Processor.Registers[d] &= 0b11111101
			return false
		},
	}},
	0x8B: {label: "RES 1,E", value: 0x8B, steps: []stepFunc{
		func() bool {
			Processor.Registers[e] &= 0b11111101
			return false
		},
	}},
	0x8C: {label: "RES 1,H", value: 0x8C, steps: []stepFunc{
		func() bool {
			Processor.Registers[h] &= 0b11111101
			return false
		},
	}},
	0x8D: {label: "RES 1,L", value: 0x8D, steps: []stepFunc{
		func() bool {
			Processor.Registers[l] &= 0b11111101
			return false
		},
	}},
	0x8E: {label: "RES 1,(HL)", value: 0x8E, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool { // "read"
			return true
		},
		func() bool {
			memory.CPU.Write(getRegisterHL(), memory.CPU.Read(getRegisterHL())&0b11111101)
			return false
		},
	}},
	0x8F: {label: "RES 1,A", value: 0x8F, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] &= 0b11111101
			return false
		},
	}},
	0x90: {label: "RES 2,B", value: 0x90, steps: []stepFunc{
		func() bool {
			Processor.Registers[b] &= 0b11111011
			return false
		},
	}},
	0x91: {label: "RES 2,C", value: 0x91, steps: []stepFunc{
		func() bool {
			Processor.Registers[c] &= 0b11111011
			return false
		},
	}},
	0x92: {label: "RES 2,D", value: 0x92, steps: []stepFunc{
		func() bool {
			Processor.Registers[d] &= 0b11111011
			return false
		},
	}},
	0x93: {label: "RES 2,E", value: 0x93, steps: []stepFunc{
		func() bool {
			Processor.Registers[e] &= 0b11111011
			return false
		},
	}},
	0x94: {label: "RES 2,H", value: 0x94, steps: []stepFunc{
		func() bool {
			Processor.Registers[h] &= 0b11111011
			return false
		},
	}},
	0x95: {label: "RES 2,L", value: 0x95, steps: []stepFunc{
		func() bool {
			Processor.Registers[l] &= 0b11111011
			return false
		},
	}},
	0x96: {label: "RES 2,(HL)", value: 0x96, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool { // "read"
			return true
		},
		func() bool {
			memory.CPU.Write(getRegisterHL(), memory.CPU.Read(getRegisterHL())&0b11111011)
			return false
		},
	}},
	0x97: {label: "RES 2,A", value: 0x97, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] &= 0b11111011
			return false
		},
	}},
	0x98: {label: "RES 3,B", value: 0x98, steps: []stepFunc{
		func() bool {
			Processor.Registers[b] &= 0b11110111
			return false
		},
	}},
	0x99: {label: "RES 3,C", value: 0x99, steps: []stepFunc{
		func() bool {
			Processor.Registers[c] &= 0b11110111
			return false
		},
	}},
	0x9A: {label: "RES 3,D", value: 0x9A, steps: []stepFunc{
		func() bool {
			Processor.Registers[d] &= 0b11110111
			return false
		},
	}},
	0x9B: {label: "RES 3,E", value: 0x9B, steps: []stepFunc{
		func() bool {
			Processor.Registers[e] &= 0b11110111
			return false
		},
	}},
	0x9C: {label: "RES 3,H", value: 0x9C, steps: []stepFunc{
		func() bool {
			Processor.Registers[h] &= 0b11110111
			return false
		},
	}},
	0x9D: {label: "RES 3,L", value: 0x9D, steps: []stepFunc{
		func() bool {
			Processor.Registers[l] &= 0b11110111
			return false
		},
	}},
	0x9E: {label: "RES 3,(HL)", value: 0x9E, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool { // "read"
			return true
		},
		func() bool {
			memory.CPU.Write(getRegisterHL(), memory.CPU.Read(getRegisterHL())&0b11110111)
			return false
		},
	}},
	0x9F: {label: "RES 3,A", value: 0x9F, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] &= 0b11110111
			return false
		},
	}},
	0xA0: {label: "RES 4,B", value: 0xA0, steps: []stepFunc{
		func() bool {
			Processor.Registers[b] &= 0b11101111
			return false
		},
	}},
	0xA1: {label: "RES 4,C", value: 0xA1, steps: []stepFunc{
		func() bool {
			Processor.Registers[c] &= 0b11101111
			return false
		},
	}},
	0xA2: {label: "RES 4,D", value: 0xA2, steps: []stepFunc{
		func() bool {
			Processor.Registers[d] &= 0b11101111
			return false
		},
	}},
	0xA3: {label: "RES 4,E", value: 0xA3, steps: []stepFunc{
		func() bool {
			Processor.Registers[e] &= 0b11101111
			return false
		},
	}},
	0xA4: {label: "RES 4,H", value: 0xA4, steps: []stepFunc{
		func() bool {
			Processor.Registers[h] &= 0b11101111
			return false
		},
	}},
	0xA5: {label: "RES 4,L", value: 0xA5, steps: []stepFunc{
		func() bool {
			Processor.Registers[l] &= 0b11101111
			return false
		},
	}},
	0xA6: {label: "RES 4,(HL)", value: 0xA6, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool { // "read"
			return true
		},
		func() bool {
			memory.CPU.Write(getRegisterHL(), memory.CPU.Read(getRegisterHL())&0b11101111)
			return false
		},
	}},
	0xA7: {label: "RES 4,A", value: 0xA7, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] &= 0b11101111
			return false
		},
	}},
	0xA8: {label: "RES 5,B", value: 0xA8, steps: []stepFunc{
		func() bool {
			Processor.Registers[b] &= 0b11011111
			return false
		},
	}},
	0xA9: {label: "RES 5,C", value: 0xA9, steps: []stepFunc{
		func() bool {
			Processor.Registers[c] &= 0b11011111
			return false
		},
	}},
	0xAA: {label: "RES 5,D", value: 0xAA, steps: []stepFunc{
		func() bool {
			Processor.Registers[d] &= 0b11011111
			return false
		},
	}},
	0xAB: {label: "RES 5,E", value: 0xAB, steps: []stepFunc{
		func() bool {
			Processor.Registers[e] &= 0b11011111
			return false
		},
	}},
	0xAC: {label: "RES 5,H", value: 0xAC, steps: []stepFunc{
		func() bool {
			Processor.Registers[h] &= 0b11011111
			return false
		},
	}},
	0xAD: {label: "RES 5,L", value: 0xAD, steps: []stepFunc{
		func() bool {
			Processor.Registers[l] &= 0b11011111
			return false
		},
	}},
	0xAE: {label: "RES 5,(HL)", value: 0xAE, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool { // "read"
			return true
		},
		func() bool {
			memory.CPU.Write(getRegisterHL(), memory.CPU.Read(getRegisterHL())&0b11011111)
			return false
		},
	}},
	0xAF: {label: "RES 5,A", value: 0xAF, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] &= 0b11011111
			return false
		},
	}},
	0xB0: {label: "RES 6,B", value: 0xB0, steps: []stepFunc{
		func() bool {
			Processor.Registers[b] &= 0b10111111
			return false
		},
	}},
	0xB1: {label: "RES 6,C", value: 0xB1, steps: []stepFunc{
		func() bool {
			Processor.Registers[c] &= 0b10111111
			return false
		},
	}},
	0xB2: {label: "RES 6,D", value: 0xB2, steps: []stepFunc{
		func() bool {
			Processor.Registers[d] &= 0b10111111
			return false
		},
	}},
	0xB3: {label: "RES 6,E", value: 0xB3, steps: []stepFunc{
		func() bool {
			Processor.Registers[e] &= 0b10111111
			return false
		},
	}},
	0xB4: {label: "RES 6,H", value: 0xB4, steps: []stepFunc{
		func() bool {
			Processor.Registers[h] &= 0b10111111
			return false
		},
	}},
	0xB5: {label: "RES 6,L", value: 0xB5, steps: []stepFunc{
		func() bool {
			Processor.Registers[l] &= 0b10111111
			return false
		},
	}},
	0xB6: {label: "RES 6,(HL)", value: 0xB6, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool { // "read"
			return true
		},
		func() bool {
			memory.CPU.Write(getRegisterHL(), memory.CPU.Read(getRegisterHL())&0b10111111)
			return false
		},
	}},
	0xB7: {label: "RES 6,A", value: 0xB7, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] &= 0b10111111
			return false
		},
	}},
	0xB8: {label: "RES 7,B", value: 0xB8, steps: []stepFunc{
		func() bool {
			Processor.Registers[b] &= 0b01111111
			return false
		},
	}},
	0xB9: {label: "RES 7,C", value: 0xB9, steps: []stepFunc{
		func() bool {
			Processor.Registers[c] &= 0b01111111
			return false
		},
	}},
	0xBA: {label: "RES 7,D", value: 0xBA, steps: []stepFunc{
		func() bool {
			Processor.Registers[d] &= 0b01111111
			return false
		},
	}},
	0xBB: {label: "RES 7,E", value: 0xBB, steps: []stepFunc{
		func() bool {
			Processor.Registers[e] &= 0b01111111
			return false
		},
	}},
	0xBC: {label: "RES 7,H", value: 0xBC, steps: []stepFunc{
		func() bool {
			Processor.Registers[h] &= 0b01111111
			return false
		},
	}},
	0xBD: {label: "RES 7,L", value: 0xBD, steps: []stepFunc{
		func() bool {
			Processor.Registers[l] &= 0b01111111
			return false
		},
	}},
	0xBE: {label: "RES 7,(HL)", value: 0xBE, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool { // "read"
			return true
		},
		func() bool {
			memory.CPU.Write(getRegisterHL(), memory.CPU.Read(getRegisterHL())&0b01111111)
			return false
		},
	}},
	0xBF: {label: "RES 7,A", value: 0xBF, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] &= 0b01111111
			return false
		},
	}},
	0xC0: {label: "SET 0,B", value: 0xC0, steps: []stepFunc{
		func() bool {
			Processor.Registers[b] |= 0b00000001
			return false
		},
	}},
	0xC1: {label: "SET 0,C", value: 0xC1, steps: []stepFunc{
		func() bool {
			Processor.Registers[c] |= 0b00000001
			return false
		},
	}},
	0xC2: {label: "SET 0,D", value: 0xC2, steps: []stepFunc{
		func() bool {
			Processor.Registers[d] |= 0b00000001
			return false
		},
	}},
	0xC3: {label: "SET 0,E", value: 0xC3, steps: []stepFunc{
		func() bool {
			Processor.Registers[e] |= 0b00000001
			return false
		},
	}},
	0xC4: {label: "SET 0,H", value: 0xC4, steps: []stepFunc{
		func() bool {
			Processor.Registers[h] |= 0b00000001
			return false
		},
	}},
	0xC5: {label: "SET 0,L", value: 0xC5, steps: []stepFunc{
		func() bool {
			Processor.Registers[l] |= 0b00000001
			return false
		},
	}},
	0xC6: {label: "SET 0,(HL)", value: 0xC6, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool { // "read"
			return true
		},
		func() bool {
			memory.CPU.Write(getRegisterHL(), memory.CPU.Read(getRegisterHL())|0b00000001)
			return false
		},
	}},
	0xC7: {label: "SET 0,A", value: 0xC7, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] |= 0b00000001
			return false
		},
	}},
	0xC8: {label: "SET 1,B", value: 0xC8, steps: []stepFunc{
		func() bool {
			Processor.Registers[b] |= 0b00000010
			return false
		},
	}},
	0xC9: {label: "SET 1,C", value: 0xC9, steps: []stepFunc{
		func() bool {
			Processor.Registers[c] |= 0b00000010
			return false
		},
	}},
	0xCA: {label: "SET 1,D", value: 0xCA, steps: []stepFunc{
		func() bool {
			Processor.Registers[d] |= 0b00000010
			return false
		},
	}},
	0xCB: {label: "SET 1,E", value: 0xCB, steps: []stepFunc{
		func() bool {
			Processor.Registers[e] |= 0b00000010
			return false
		},
	}},
	0xCC: {label: "SET 1,H", value: 0xCC, steps: []stepFunc{
		func() bool {
			Processor.Registers[h] |= 0b00000010
			return false
		},
	}},
	0xCD: {label: "SET 1,L", value: 0xCD, steps: []stepFunc{
		func() bool {
			Processor.Registers[l] |= 0b00000010
			return false
		},
	}},
	0xCE: {label: "SET 1, (HL)", value: 0xCE, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool { // "read"
			return true
		},
		func() bool {
			memory.CPU.Write(getRegisterHL(), memory.CPU.Read(getRegisterHL())|0b00000010)
			return false
		},
	}},
	0xCF: {label: "SET 1,A", value: 0xCF, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] |= 0b00000010
			return false
		},
	}},
	0xD0: {label: "SET 2,B", value: 0xD0, steps: []stepFunc{
		func() bool {
			Processor.Registers[b] |= 0b00000100
			return false
		},
	}},
	0xD1: {label: "SET 2,C", value: 0xD1, steps: []stepFunc{
		func() bool {
			Processor.Registers[c] |= 0b00000100
			return false
		},
	}},
	0xD2: {label: "SET 2,D", value: 0xD2, steps: []stepFunc{
		func() bool {
			Processor.Registers[d] |= 0b00000100
			return false
		},
	}},
	0xD3: {label: "SET 2,E", value: 0xD3, steps: []stepFunc{
		func() bool {
			Processor.Registers[e] |= 0b00000100
			return false
		},
	}},
	0xD4: {label: "SET 2,H", value: 0xD4, steps: []stepFunc{
		func() bool {
			Processor.Registers[h] |= 0b00000100
			return false
		},
	}},
	0xD5: {label: "SET 2,L", value: 0xD5, steps: []stepFunc{
		func() bool {
			Processor.Registers[l] |= 0b00000100
			return false
		},
	}},
	0xD6: {label: "SET 2, (HL)", value: 0xD6, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool { // "read"
			return true
		},
		func() bool {
			memory.CPU.Write(getRegisterHL(), memory.CPU.Read(getRegisterHL())|0b00000100)
			return false
		},
	}},
	0xD7: {label: "SET 2,A", value: 0xD7, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] |= 0b00000100
			return false
		},
	}},
	0xD8: {label: "SET 3,B", value: 0xD8, steps: []stepFunc{
		func() bool {
			Processor.Registers[b] |= 0b00001000
			return false
		},
	}},
	0xD9: {label: "SET 3,C", value: 0xD9, steps: []stepFunc{
		func() bool {
			Processor.Registers[c] |= 0b00001000
			return false
		},
	}},
	0xDA: {label: "SET 3,D", value: 0xDE, steps: []stepFunc{
		func() bool {
			Processor.Registers[d] |= 0b00001000
			return false
		},
	}},
	0xDB: {label: "SET 3,E", value: 0xDB, steps: []stepFunc{
		func() bool {
			Processor.Registers[e] |= 0b00001000
			return false
		},
	}},
	0xDC: {label: "SET 3,H", value: 0xDC, steps: []stepFunc{
		func() bool {
			Processor.Registers[h] |= 0b00001000
			return false
		},
	}},
	0xDD: {label: "SET 3,L", value: 0xDD, steps: []stepFunc{
		func() bool {
			Processor.Registers[l] |= 0b00001000
			return false
		},
	}},
	0xDE: {label: "SET 3, (HL)", value: 0xDE, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool { // "read"
			return true
		},
		func() bool {
			memory.CPU.Write(getRegisterHL(), memory.CPU.Read(getRegisterHL())|0b00001000)
			return false
		},
	}},
	0xDF: {label: "SET 3,A", value: 0xDF, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] |= 0b00001000
			return false
		},
	}},
	0xE0: {label: "SET 4,B", value: 0xE0, steps: []stepFunc{
		func() bool {
			Processor.Registers[b] |= 0b00010000
			return false
		},
	}},
	0xE1: {label: "SET 4,C", value: 0xE1, steps: []stepFunc{
		func() bool {
			Processor.Registers[c] |= 0b00010000
			return false
		},
	}},
	0xE2: {label: "SET 4,D", value: 0xE2, steps: []stepFunc{
		func() bool {
			Processor.Registers[d] |= 0b00010000
			return false
		},
	}},
	0xE3: {label: "SET 4,E", value: 0xE3, steps: []stepFunc{
		func() bool {
			Processor.Registers[e] |= 0b00010000
			return false
		},
	}},
	0xE4: {label: "SET 4,H", value: 0xE4, steps: []stepFunc{
		func() bool {
			Processor.Registers[h] |= 0b00010000
			return false
		},
	}},
	0xE5: {label: "SET 4,L", value: 0xE5, steps: []stepFunc{
		func() bool {
			Processor.Registers[l] |= 0b00010000
			return false
		},
	}},
	0xE6: {label: "SET 4, (HL)", value: 0xE6, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool { // "read"
			return true
		},
		func() bool {
			memory.CPU.Write(getRegisterHL(), memory.CPU.Read(getRegisterHL())|0b00010000)
			return false
		},
	}},
	0xE7: {label: "SET 4,A", value: 0xE7, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] |= 0b00010000
			return false
		},
	}},
	0xE8: {label: "SET 5,B", value: 0xE8, steps: []stepFunc{
		func() bool {
			Processor.Registers[b] |= 0b00100000
			return false
		},
	}},
	0xE9: {label: "SET 5,C", value: 0xE9, steps: []stepFunc{
		func() bool {
			Processor.Registers[c] |= 0b00100000
			return false
		},
	}},
	0xEA: {label: "SET 5,D", value: 0xEA, steps: []stepFunc{
		func() bool {
			Processor.Registers[d] |= 0b00100000
			return false
		},
	}},
	0xEB: {label: "SET 5,E", value: 0xEB, steps: []stepFunc{
		func() bool {
			Processor.Registers[e] |= 0b00100000
			return false
		},
	}},
	0xEC: {label: "SET 5,H", value: 0xEC, steps: []stepFunc{
		func() bool {
			Processor.Registers[h] |= 0b00100000
			return false
		},
	}},
	0xED: {label: "SET 5,L", value: 0xED, steps: []stepFunc{
		func() bool {
			Processor.Registers[l] |= 0b00100000
			return false
		},
	}},
	0xEE: {label: "SET 5, (HL)", value: 0xEE, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool { // "read"
			return true
		},
		func() bool {
			memory.CPU.Write(getRegisterHL(), memory.CPU.Read(getRegisterHL())|0b00100000)
			return false
		},
	}},
	0xEF: {label: "SET 5,A", value: 0xEF, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] |= 0b00100000
			return false
		},
	}},
	0xF0: {label: "SET 6,B", value: 0xF0, steps: []stepFunc{
		func() bool {
			Processor.Registers[b] |= 0b01000000
			return false
		},
	}},
	0xF1: {label: "SET 6,C", value: 0xF1, steps: []stepFunc{
		func() bool {
			Processor.Registers[c] |= 0b01000000
			return false
		},
	}},
	0xF2: {label: "SET 6,D", value: 0xF2, steps: []stepFunc{
		func() bool {
			Processor.Registers[d] |= 0b01000000
			return false
		},
	}},
	0xF3: {label: "SET 6,E", value: 0xF3, steps: []stepFunc{
		func() bool {
			Processor.Registers[e] |= 0b01000000
			return false
		},
	}},
	0xF4: {label: "SET 6,H", value: 0xF4, steps: []stepFunc{
		func() bool {
			Processor.Registers[h] |= 0b01000000
			return false
		},
	}},
	0xF5: {label: "SET 6,L", value: 0xF5, steps: []stepFunc{
		func() bool {
			Processor.Registers[l] |= 0b01000000
			return false
		},
	}},
	0xF6: {label: "SET 6, (HL)", value: 0xF6, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool { // "read"
			return true
		},
		func() bool {
			memory.CPU.Write(getRegisterHL(), memory.CPU.Read(getRegisterHL())|0b01000000)
			return false
		},
	}},
	0xF7: {label: "SET 6,A", value: 0xF7, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] |= 0b01000000
			return false
		},
	}},
	0xF8: {label: "SET 7,B", value: 0xF8, steps: []stepFunc{
		func() bool {
			Processor.Registers[b] |= 0b10000000
			return false
		},
	}},
	0xF9: {label: "SET 7,C", value: 0xF9, steps: []stepFunc{
		func() bool {
			Processor.Registers[c] |= 0b10000000
			return false
		},
	}},
	0xFA: {label: "SET 7,D", value: 0xFA, steps: []stepFunc{
		func() bool {
			Processor.Registers[d] |= 0b10000000
			return false
		},
	}},
	0xFB: {label: "SET 7,E", value: 0xFB, steps: []stepFunc{
		func() bool {
			Processor.Registers[e] |= 0b10000000
			return false
		},
	}},
	0xFC: {label: "SET 7,H", value: 0xFC, steps: []stepFunc{
		func() bool {
			Processor.Registers[h] |= 0b10000000
			return false
		},
	}},
	0xFD: {label: "SET 7,L", value: 0xFD, steps: []stepFunc{
		func() bool {
			Processor.Registers[l] |= 0b10000000
			return false
		},
	}},
	0xFE: {label: "SET 7, (HL)", value: 0xFE, steps: []stepFunc{
		func() bool { // "fetch"
			return true
		},
		func() bool { // "read"
			return true
		},
		func() bool {
			memory.CPU.Write(getRegisterHL(), memory.CPU.Read(getRegisterHL())|0b10000000)
			return false
		},
	}},
	0xFF: {label: "SET 7,A", value: 0xFF, steps: []stepFunc{
		func() bool {
			Processor.Registers[a] |= 0b10000000
			return false
		},
	}},
}
