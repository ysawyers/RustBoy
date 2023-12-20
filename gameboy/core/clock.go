package core

import (
	"github.com/ysawyers/emufun/gameboy/core/cpu"
	"github.com/ysawyers/emufun/gameboy/core/ppu"
)

const _CYCLES_PER_FRAME int = 69905

func Tick() [144][160]byte {
	for i := 0; i < _CYCLES_PER_FRAME; i++ {
		cpu.TickCPU()
		ppu.TickPPU()
		ppu.TickPPU()
	}
	return ppu.GetScreen()
}
