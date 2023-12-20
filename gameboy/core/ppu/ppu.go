package ppu

import (
	"log"

	"github.com/ysawyers/emufun/gameboy/core/utils"
)

type ppu struct {
	lcd               [144][160]byte
	xPos              int
	windowLineCounter int
	fetcherX          int
	isFetchingSprite  bool
	spriteBuffer      []sprite
	backgroundFIFO    []byte
	spriteFIFO        []byte
	state             ppuCycleState
}

type ppuCycleState struct {
	oamCursor     int
	tileNumber    byte
	tileDataLow   byte
	tileDataHigh  byte
	currentStep   int
	isNewScanline bool
}

type sprite struct {
	yPos       byte
	xPos       byte
	tileNumber byte
	flags      byte
}

var display ppu
var scanlineTimeline int
var isNewFrame bool = true

// Each call represents 1/2 cycle (2 dots)
func TickPPU() {
	if getSCX() > 0 {
		log.Fatal("Not Implemented Yet!")
	}

	if isPPUEnabled() == 1 {
		scanlineTimeline += 2

		switch getMode() {
		case utils.PPU_MODE_HBLANK:
			hBlank()
		case utils.PPU_MODE_VBLANK:
			vBlank()
		case utils.PPU_MODE_OAM_SCAN:
			scanOAM()
		case utils.PPU_MODE_DRAW:
			backgroundPixelFetcher()
			if isSpritesEnabled() == 1 {
				spritePixelFetcher()
			}
			pixelMixer()
		}

		if scanlineTimeline == 456 {
			scanlineTimeline = 0
		}
	}
}

func GetScreen() [144][160]byte {
	return display.lcd
}
