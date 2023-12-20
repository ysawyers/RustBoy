package cpu

const _ZERO_FLAG_BYTE_POSITION byte = 7       // zero
const _SUBTRACT_FLAG_BYTE_POSITION byte = 6   // underflow
const _HALF_CARRY_FLAG_BYTE_POSITION byte = 5 // nibble-overflow
const _CARRY_FLAG_BYTE_POSITION byte = 4      // byte overflow

type Register byte

const (
	a Register = iota
	b
	c
	d
	e
	f
	h
	l
)

func getRegisterAF() uint16 {
	return (uint16(Processor.Registers[a]) << 8) | uint16(Processor.Registers[f])
}

func getRegisterBC() uint16 {
	return (uint16(Processor.Registers[b]) << 8) | uint16(Processor.Registers[c])
}

func getRegisterDE() uint16 {
	return (uint16(Processor.Registers[d]) << 8) | uint16(Processor.Registers[e])
}

func getRegisterHL() uint16 {
	return (uint16(Processor.Registers[h]) << 8) | uint16(Processor.Registers[l])
}

func getZeroFlag() byte {
	return (Processor.Registers[f] >> _ZERO_FLAG_BYTE_POSITION) & 0x1
}

func getSubtractFlag() byte {
	return (Processor.Registers[f] >> _SUBTRACT_FLAG_BYTE_POSITION) & 0x1
}

func getHalfCarryFlag() byte {
	return (Processor.Registers[f] >> _HALF_CARRY_FLAG_BYTE_POSITION) & 0x1
}

func getCarryFlag() byte {
	return (Processor.Registers[f] >> _CARRY_FLAG_BYTE_POSITION) & 0x1
}

func setRegisterAF(v uint16) {
	Processor.Registers[a] = byte((v & 0xFF00) >> 8)
	Processor.Registers[f] = byte(v & 0xFF)
}

func setRegisterBC(v uint16) {
	Processor.Registers[b] = byte((v & 0xFF00) >> 8)
	Processor.Registers[c] = byte(v & 0xFF)
}

func setRegisterDE(v uint16) {
	Processor.Registers[d] = byte((v & 0xFF00) >> 8)
	Processor.Registers[e] = byte(v & 0xFF)
}

func setRegisterHL(v uint16) {
	Processor.Registers[h] = byte((v & 0xFF00) >> 8)
	Processor.Registers[l] = byte(v & 0xFF)
}

func setZeroFlag(v bool) {
	if v {
		Processor.Registers[f] |= 0b10000000
	} else {
		Processor.Registers[f] &= 0b01110000
	}
}

func setSubtractFlag(v bool) {
	if v {
		Processor.Registers[f] |= 0b01000000
	} else {
		Processor.Registers[f] &= 0b10110000
	}
}

func setHalfCarryFlag(v bool) {
	if v {
		Processor.Registers[f] |= 0b00100000
	} else {
		Processor.Registers[f] &= 0b11010000
	}
}

func setCarryFlag(v bool) {
	if v {
		Processor.Registers[f] |= 0b00010000
	} else {
		Processor.Registers[f] &= 0b11100000
	}
}

func rotateLeft(v byte) byte {
	CY := getCarryFlag()
	setCarryFlag(((v >> 7) & 0x1) == 0x1)
	v <<= 1
	if CY == 0x1 {
		v |= 0b00000001
	} else {
		v &= 0b11111110
	}
	return v
}

func rotateLeftC(v byte) byte {
	B7 := (v >> 7) & 0x1
	setCarryFlag(B7 == 0x1)
	v <<= 1
	if B7 == 0x1 {
		v |= 0b00000001
	} else {
		v &= 0b11111110
	}
	return v
}

func rotateRight(v byte) byte {
	CY := getCarryFlag()
	setCarryFlag((v & 0x1) == 0x1)
	v >>= 1
	if CY == 0x1 {
		v |= 0b10000000
	} else {
		v &= 0b01111111
	}
	return v
}

func rotateRightC(v byte) byte {
	B0 := v & 0x1
	setCarryFlag(B0 == 0x1)
	v >>= 1
	if B0 == 0x1 {
		v |= 0b10000000
	} else {
		v &= 0b01111111
	}
	return v
}

func shiftLeft(v byte) byte {
	setCarryFlag(((v >> 7) & 0x1) == 0x1)
	v <<= 1
	return v
}

func shiftRightA(v byte) byte {
	setCarryFlag((v & 0x1) == 0x1)
	v >>= 1
	if ((v >> 6) & 0x1) == 0x1 {
		v |= 0b10000000
	} else {
		v &= 0b01111111
	}
	return v
}

func shiftRightL(v byte) byte {
	setCarryFlag((v & 0x1) == 0x1)
	v >>= 1
	return v
}

func swap(v byte) byte {
	lower := (v & 0b00001111)
	upper := (v & 0b11110000) >> 4
	return (lower << 4) | upper
}
