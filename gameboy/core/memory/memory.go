package memory

import (
	"log"

	"github.com/ysawyers/emufun/gameboy/core/utils"
)

type Component int

const (
	CPU Component = iota
	PPU
	Core
)

var memory [0x10000]byte
var bootRom [256]byte
var BootRomIsMounted bool

func (c Component) Read(a uint16) byte {
	/* GAMEBOY DOCTOR LY READ */
	if utils.DEBUG_MODE && a == 0xFF44 {
		return 0x90
	}

	if BootRomIsMounted && a >= 0x00 && a <= 0xFF {
		return bootRom[a]
	}

	if a >= 0x8000 && a <= 0x9FFF {
		if (memory[utils.STAT_REGISTER]&0x3) == utils.PPU_MODE_DRAW || (memory[utils.STAT_REGISTER]&0x3) == utils.PPU_MODE_OAM_SCAN {
			if c == PPU || c == Core {
				return memory[a]
			}
			return 0xFF // VRAM access is restricted by external components on pixel transfer or OAM scan
		}
	}
	if a >= 0xFE00 && a <= 0xFE9F {
		if (memory[utils.STAT_REGISTER] & 0x3) == utils.PPU_MODE_OAM_SCAN {
			if c == PPU || c == Core {
				return memory[a]
			}
			return 0xFF // OAM access is restricted by external components on OAM scan
		}
	}
	return memory[a]
}

func (c Component) Write(a uint16, v byte) {
	if a == 0xFF50 { // successful boot
		BootRomIsMounted = false
	}

	if a <= 0x7FFF { /* ROM MAPPING */
		log.Fatal("Memory Banking not implemented yet!")
	}

	if a >= 0x8000 && a <= 0x9FFF { /* VRAM MAPPING */
		if (memory[utils.STAT_REGISTER]&0x3) == utils.PPU_MODE_DRAW || (memory[utils.STAT_REGISTER]&0x3) == utils.PPU_MODE_OAM_SCAN {
			if c == PPU || c == Core {
				memory[a] = v
			}
		} else {
			memory[a] = v
		}
	} else if a >= 0xFE00 && a <= 0xFE9F { /* OAM MAPPING */
		if (memory[utils.STAT_REGISTER] & 0x3) == utils.PPU_MODE_OAM_SCAN {
			if c == PPU || c == Core {
				memory[a] = v
			}
		} else {
			memory[a] = v
		}
	} else {
		memory[a] = v
	}
}

func InitializeBootFile(bootFile []byte) {
	BootRomIsMounted = true
	for i := range bootFile {
		bootRom[i] = bootFile[i]
	}
}

func InitializeCartridge(cartridge []byte) {
	for i := range cartridge {
		memory[i] = cartridge[i]
	}
}
