package ppu

import (
	"log"

	"github.com/ysawyers/emufun/gameboy/core/memory"
	"github.com/ysawyers/emufun/gameboy/core/utils"
)

var vBlankTimeline int

func windowInView() bool {
	return false
}

// Takes a constant 20 cycles (80 dots)
func scanOAM() {
	if len(display.spriteBuffer) < 10 {
		objectID := 0xFE00 + uint16((display.state.oamCursor * 4))

		yPos := memory.PPU.Read(objectID)
		xPos := memory.PPU.Read(objectID + 1)
		tileNumber := memory.PPU.Read(objectID + 2)
		flags := memory.PPU.Read(objectID + 3)

		heightLimit := yPos + 8

		if getSpriteSize() == 1 {
			heightLimit += 8
		}

		if xPos > 0 && getLY()+16 >= yPos && getLY()+16 < heightLimit {
			log.Fatal("Appending to Sprite Buffer Not Implemented")
			display.spriteBuffer = append(display.spriteBuffer, sprite{
				yPos:       yPos,
				xPos:       xPos,
				tileNumber: tileNumber,
				flags:      flags,
			})
		}
	}

	if display.state.oamCursor < 39 {
		display.state.oamCursor++
	} else {
		setMode(utils.PPU_MODE_DRAW)
	}
}

func backgroundPixelFetcher() {
	if isWindowOrBackgroundEnabled() == 0 { // all bg/window pixels are treated as blank
		if len(display.backgroundFIFO) <= 8 {
			for j := 0; j < 8; j++ {
				display.backgroundFIFO = append(display.backgroundFIFO, 0)
			}
		}
		return
	}

	if display.state.currentStep < 1 {
		var activeTileMap uint16 = 0x9800
		var tileX uint16
		var tileY uint16

		if windowInView() && isWindowEnabled() == 1 {
			if getWindowTileMap() == 1 {
				activeTileMap = 0x9C00
			}
			log.Fatal("NOT IMPLEMENTED YET!")
		} else {
			if getBackgroundTileMap() == 1 {
				activeTileMap = 0x9C00
			}
			tileX = (uint16(display.fetcherX) + uint16(getSCX()/8)) & 0x1F
			tileY = 32 * (((uint16(getLY()) + uint16(getSCY())) & 0xFF) / 8)
		}

		display.state.tileNumber = memory.PPU.Read(activeTileMap + ((tileX + tileY) & 0x3FF))
		display.state.currentStep++
	} else if display.state.currentStep < 2 {
		offset := 2 * ((uint16(getLY()) + uint16(getSCY())) % 8)
		var tile uint16

		if getAddressingMethod() == 1 {
			tile = 0x8000 + (uint16(display.state.tileNumber) * 16)
		} else {
			var signedTileNumber int8 = int8(display.state.tileNumber)
			if signedTileNumber < 0 {
				tile = 0x9000 - uint16(signedTileNumber*-1)*16
			} else {
				tile = 0x9000 + uint16(signedTileNumber)*16
			}
		}

		display.state.tileDataLow = memory.PPU.Read(tile + offset)
		display.state.currentStep++
	} else if display.state.currentStep < 3 {
		offset := 2 * ((uint16(getLY()) + uint16(getSCY())) % 8)
		var tile uint16

		if getAddressingMethod() == 1 {
			tile = 0x8000 + (uint16(display.state.tileNumber) * 16)
		} else {
			var signedTileNumber int8 = int8(display.state.tileNumber)
			if signedTileNumber < 0 {
				tile = 0x9000 - uint16(signedTileNumber*-1)*16
			} else {
				tile = 0x9000 + uint16(signedTileNumber)*16
			}
		}

		display.state.tileDataHigh = memory.PPU.Read(tile + offset + 1)
		display.state.currentStep++
	} else if display.state.isNewScanline {
		display.state.currentStep = 0
		display.state.isNewScanline = false
	} else {
		if len(display.backgroundFIFO) <= 8 {
			for j := 0; j < 8; j++ {
				pixel := (((display.state.tileDataHigh >> (7 - j)) & 0x1) << 1) | ((display.state.tileDataLow >> (7 - j)) & 0x1)
				display.backgroundFIFO = append(display.backgroundFIFO, pixel)
			}
			display.state.currentStep = 0
		}
		display.fetcherX++
	}
}

func lookupSprite() {}

func spritePixelFetcher() {}

// TODO: Figure out how to handle scrolling with pixel FIFO
func pixelMixer() {
	for i := 0; i < 2; i++ { // can push twice for one pixel fetcher step
		if len(display.backgroundFIFO) > 8 {
			pixelToPush := display.backgroundFIFO[0]
			display.backgroundFIFO = display.backgroundFIFO[1:]
			display.lcd[getLY()][display.xPos] = pixelToPush
			display.xPos++
		}

		if display.xPos > 159 {
			setMode(utils.PPU_MODE_HBLANK)
			break
		}
	}
}

func hBlank() {
	display.spriteFIFO = display.spriteFIFO[:0]
	display.backgroundFIFO = display.spriteFIFO[:0]
	display.xPos = 0
	display.fetcherX = 0
	display.state.oamCursor = 0
	display.state.currentStep = 0
	display.state.isNewScanline = true

	if scanlineTimeline == 456 {
		if isNewFrame {
			isNewFrame = false
			setMode(utils.PPU_MODE_OAM_SCAN)
		} else {
			setLY(getLY() + 1)
			if getLY() > 143 {
				setMode(utils.PPU_MODE_VBLANK)
			} else {
				setMode(utils.PPU_MODE_OAM_SCAN)
			}
		}
	}
}

// Takes a constant 1140 clocks (4560 dots)
func vBlank() {
	vBlankTimeline += 2

	if vBlankTimeline == 4560 {
		vBlankTimeline = 0
		setLY(0)
		setMode(utils.PPU_MODE_OAM_SCAN)
	} else {
		if (vBlankTimeline % 456) == 0 {
			setLY(getLY() + 1)
		}
	}
}
