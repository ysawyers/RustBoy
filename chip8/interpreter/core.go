package interpreter

import (
	"log"
	"math/rand"
)

type env struct {
	Display    [32][64]bool
	Keys       [16]bool
	Memory     [4096]byte
	Stack      stack[uint16]
	PC         uint16
	I          uint16
	V          [16]byte
	DelayTimer byte
	SoundTimer byte
}

var vm env

func FetchNextGameState(key int) ([32][64]bool, bool) {
	var op uint16 = uint16(vm.Memory[vm.PC])<<8 | uint16(vm.Memory[vm.PC+1])

	var X byte = byte((op >> 8)) & 0xF
	var Y byte = byte((op & 0xFF)) >> 4
	var NN byte = byte(op & 0xFF)
	var NNN uint16 = op & 0xFFF

	vm.PC += 2

	switch op >> 12 {
	case 0x0:
		switch op & 0xFF {
		case 0xE0: // 00E0: Clears the screen.
			vm.Display = [32][64]bool{}

		case 0xEE: // 00EE: Returns from a subroutine.
			address, err := vm.Stack.pop()
			if err != nil {
				log.Panic(err.Error())
			}
			vm.PC = *address
		}
	case 0x1: // 1NNN: Jumps to address NNN.
		vm.PC = NNN
	case 0x2: // 2NNN: Calls subroutine at NNN.
		vm.Stack.push(vm.PC)
		vm.PC = NNN
	case 0x3: // 3XNN: Skips the next instruction if VX equals NN (usually the next instruction is a jump to skip a code block).
		if vm.V[X] == NN {
			vm.PC += 2
		}
	case 0x4: // 4XNN: Skips the next instruction if VX does not equal NN (usually the next instruction is a jump to skip a code block).
		if vm.V[X] != NN {
			vm.PC += 2
		}
	case 0x5: // 5XY0: Skips the next instruction if VX equals VY (usually the next instruction is a jump to skip a code block).
		if vm.V[X] == vm.V[Y] {
			vm.PC += 2
		}
	case 0x6: // 6XNN: Sets VX to NN.
		vm.V[X] = NN
	case 0x7: // 7XNN: Adds NN to VX (carry flag is not changed).
		vm.V[X] += NN
	case 0x8:
		switch 0xF & op {
		case 0x0: // 8XY0: Sets VX to the value of VY.
			vm.V[X] = vm.V[Y]
		case 0x1: // 8XY1: Sets VX to VX or VY. (bitwise OR operation)
			vm.V[X] |= vm.V[Y]
		case 0x2: // 8XY2: Sets VX to VX and VY. (bitwise AND operation)
			vm.V[X] &= vm.V[Y]
		case 0x3: // 8XY3: Sets VX to VX xor VY. (bitwise XOR operation)
			vm.V[X] ^= vm.V[Y]
		case 0x4: // 8XY4: Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there is not.
			vm.V[X] += vm.V[Y]
			if vm.V[Y] > vm.V[X] {
				vm.V[0xF] = 1
			} else {
				vm.V[0xF] = 0
			}
		case 0x5: // 8XY5: VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there is not.
			if vm.V[Y] > vm.V[X] {
				vm.V[0xF] = 0
			} else {
				vm.V[0xF] = 1
			}
			vm.V[X] -= vm.V[Y]
		case 0x6: // Stores the least significant bit of VX in VF and then shifts VX to the right by 1.
			vm.V[0xF] = vm.V[X] & 0x1
			vm.V[X] >>= 1
		case 0x7: // Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there is not.
			if vm.V[X] > vm.V[Y] {
				vm.V[0xF] = 0
			} else {
				vm.V[0xF] = 1
			}
			vm.V[X] = vm.V[Y] - vm.V[X]
		case 0xE: // Stores the most significant bit of VX in VF and then shifts VX to the left by 1.
			vm.V[0xF] = vm.V[X] >> 7
			vm.V[X] <<= 1
		}
	case 0x9: // 9XY0: Skips the next instruction if VX does not equal VY. (Usually the next instruction is a jump to skip a code block).
		if vm.V[X] != vm.V[Y] {
			vm.PC += 2
		}
	case 0xA: // ANNN: Sets I to the address NNN.
		vm.I = NNN
	case 0xB: // BNNN: Jumps to the address NNN plus VX. (QUIRK)
		vm.PC = NNN + uint16(vm.V[X])
	case 0xC: // CXNN: Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN.
		vm.V[X] = byte(rand.Intn(256)) & NN

	case 0xD: // DXYN: Draws at (Vx, Vy) 8 wide, N high
		Vx := vm.V[(op>>8)&0xF] % 64
		Vy := vm.V[(op&0xFF)>>4] % 32
		N := int(op & 0xF)

		vm.V[0xF] = 0

		for row := 0; row < N; row++ {
			if Vy+byte(row) >= 32 {
				break
			}

			NthByte := vm.Memory[int(vm.I)+row]

			for col := 0; col < 8; col++ {
				if Vx+byte(col) >= 64 {
					break
				}

				if (0x80>>col)&NthByte != 0 {
					if vm.Display[Vy+byte(row)][Vx+byte(col)] {
						vm.Display[Vy+byte(row)][Vx+byte(col)] = false
						vm.V[0xF] = 1
					} else {
						vm.Display[Vy+byte(row)][Vx+byte(col)] = true
					}
				}
			}
		}
	case 0xE:
		switch op & 0xFF {
		case 0x9E: // EX9E: Skips the next instruction if the key stored in VX is pressed.
			if key != -1 && vm.V[X] == byte(key) {
				vm.PC += 2
			}
		case 0xA1: // EXA1: Skips the next instruction if the key stored in VX is not pressed.
			if key != -1 && vm.V[X] != byte(key) {
				vm.PC += 2
			}
		}
	case 0xF:
		switch op & 0xFF {
		case 0x07: // FX07: Sets VX to the value of the delay timer.
			vm.V[X] = vm.DelayTimer
		case 0x0A: // FX0A: A key press is awaited, and then stored in VX (blocking operation, all instruction halted until next key event).
			vm.PC -= 2
			if key != -1 {
				vm.V[X] = byte(key)
				vm.PC += 2
			}
		case 0x15: // FX15: Sets the delay timer to VX.
			vm.DelayTimer = vm.V[X]
		case 0x18: // FX18: Sets the sound timer to VX.
			vm.SoundTimer = vm.V[X]
		case 0x1E: // FX1E: Adds VX to I. (QUIRK)
			if vm.I+uint16(vm.V[X]) > 0xFFF {
				vm.V[0xF] = 1
			} else {
				vm.V[0xF] = 0
			}
			vm.I += uint16(vm.V[X])
		case 0x29:
			// FX29: Sets I to the location of the sprite for the character in VX.
			// Characters 0-F (in hexadecimal) are represented by a 4x5 font.
			vm.I = uint16(vm.V[X] * 0x5)
		case 0x33:
			// FX33: Stores the binary-coded decimal representation of VX,
			// with the hundreds digit in memory at location in I, the tens digit at location I+1,
			// and the ones digit at location I+2
			vm.Memory[vm.I] = vm.V[X] / 100         // hundreds digit
			vm.Memory[vm.I+1] = (vm.V[X] / 10) % 10 // tens digit
			vm.Memory[vm.I+2] = vm.V[X] % 10        // ones digit
		case 0x55: // (QUIRK)
			// FX55: Stores from V0 to VX (including VX) in memory, starting at address I.
			// The offset from I is increased by 1 for each value written, but I itself is left unmodified
			for i := 0; i <= int(X); i++ {
				vm.Memory[vm.I+uint16(i)] = vm.V[i]
			}
		case 0x65: // (QUIRK)
			// FX65: Fills from V0 to VX (including VX) with values from memory, starting at address I.
			// The offset from I is increased by 1 for each value read, but I itself is left unmodified.
			for i := 0; i <= int(X); i++ {
				vm.V[i] = vm.Memory[vm.I+uint16(i)]
			}
		}
	}

	shouldBeep := false

	if vm.DelayTimer > 0 {
		vm.DelayTimer--
	}

	if vm.SoundTimer > 0 {
		shouldBeep = true
		vm.SoundTimer--
	}

	return vm.Display, shouldBeep
}

func InitializeGame(file []byte) {
	vm = env{
		PC: 0x200,
	}

	fontset := [80]byte{
		0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
		0x20, 0x60, 0x20, 0x20, 0x70, // 1
		0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
		0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
		0x90, 0x90, 0xF0, 0x10, 0x10, // 4
		0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
		0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
		0xF0, 0x10, 0x20, 0x40, 0x40, // 7
		0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
		0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
		0xF0, 0x90, 0xF0, 0x90, 0x90, // A
		0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
		0xF0, 0x80, 0x80, 0x80, 0xF0, // C
		0xE0, 0x90, 0x90, 0x90, 0xE0, // D
		0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
		0xF0, 0x80, 0xF0, 0x80, 0x80, // F
	}

	for idx := range fontset {
		vm.Memory[idx] = fontset[idx]
	}

	for idx := 0; idx < len(file); idx++ {
		vm.Memory[idx+0x200] = file[idx]
	}
}
