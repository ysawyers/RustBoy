package ppu

import (
	"github.com/ysawyers/emufun/gameboy/core/memory"
	"github.com/ysawyers/emufun/gameboy/core/utils"
)

const _LCDC_REGISTER uint16 = 0xFF40

const _LCDC_FLAG_DISPLAY_ENABLE byte = 7
const _LCDC_FLAG_WINDOW_MAP_SELECT byte = 6
const _LCDC_FLAG_WINDOW_ENABLE byte = 5
const _LCDC_FLAG_TILE_ADDRESSING_METHOD byte = 4
const _LCDC_FLAG_BG_MAP_SELECT byte = 3
const _LCDC_FLAG_SPRITE_SIZE byte = 2
const _LCDC_FLAG_SPRITE_ENABLE byte = 1
const _LCDC_FLAG_WINDOW_OR_BG_ENABLE byte = 0

func getMode() byte {
	return memory.PPU.Read(utils.STAT_REGISTER) & 0x03
}

// if 1 8000 is used otherwise 8800 method
func getAddressingMethod() byte {
	return ((memory.PPU.Read(_LCDC_REGISTER) >> _LCDC_FLAG_TILE_ADDRESSING_METHOD) & 0x01)
}

// if 1 use $9C00-$9FFF otherwise $9800-$9BFF
func getBackgroundTileMap() byte {
	return ((memory.PPU.Read(_LCDC_REGISTER) >> _LCDC_FLAG_BG_MAP_SELECT) & 0x01)
}

// if 1 use $9C00-$9FFF otherwise $9800-$9BFF
func getWindowTileMap() byte {
	return ((memory.PPU.Read(_LCDC_REGISTER) >> _LCDC_FLAG_WINDOW_MAP_SELECT) & 0x01)
}

// if 1 PPU is operational
func isPPUEnabled() byte {
	return ((memory.PPU.Read(_LCDC_REGISTER) >> _LCDC_FLAG_DISPLAY_ENABLE) & 0x01)
}

// if 0 everything window-related should be ignored
func isWindowEnabled() byte {
	return ((memory.PPU.Read(_LCDC_REGISTER) >> _LCDC_FLAG_WINDOW_ENABLE) & 0x01)
}

// if 0 background and window tiles are not drawn, pixels are drawn as white except for sprites
func isWindowOrBackgroundEnabled() byte {
	return ((memory.PPU.Read(_LCDC_REGISTER) >> _LCDC_FLAG_WINDOW_OR_BG_ENABLE) & 0x01)
}

// if 0 hide all sprites
func isSpritesEnabled() byte {
	return ((memory.PPU.Read(_LCDC_REGISTER) >> _LCDC_FLAG_SPRITE_ENABLE) & 0x01)
}

// if 1 TALL otherwise NORMAL
func getSpriteSize() byte {
	return ((memory.PPU.Read(_LCDC_REGISTER) >> _LCDC_FLAG_SPRITE_SIZE) & 0x01)
}

func getLY() byte {
	return memory.PPU.Read(0xFF44)
}

func getLYC() byte {
	return memory.PPU.Read(0xFF45)
}

func getSCY() byte {
	return memory.PPU.Read(0xFF42)
}

func getSCX() byte {
	return memory.PPU.Read(0xFF43)
}

func getWY() byte {
	return memory.PPU.Read(0xFF4A)
}

func getWX() byte {
	return memory.PPU.Read(0xFF4B)
}

func setLY(v byte) {
	memory.PPU.Write(0xFF44, v)
}

func setMode(mode byte) {
	statRegister := memory.PPU.Read(utils.STAT_REGISTER)

	switch mode {
	case utils.PPU_MODE_HBLANK:
		memory.PPU.Write(utils.STAT_REGISTER, (statRegister & 0b11111100))
	case utils.PPU_MODE_VBLANK:
		memory.PPU.Write(utils.STAT_REGISTER, ((statRegister & 0b11111100) | 0b00000001))
	case utils.PPU_MODE_OAM_SCAN:
		memory.PPU.Write(utils.STAT_REGISTER, ((statRegister & 0b11111100) | 0b00000010))
	case utils.PPU_MODE_DRAW:
		memory.PPU.Write(utils.STAT_REGISTER, ((statRegister & 0b11111100) | 0b00000011))
	}
}
