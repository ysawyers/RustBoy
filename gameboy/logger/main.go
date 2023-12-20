package main

import (
	"fmt"
	"log"
	"os"

	"github.com/ysawyers/emufun/gameboy/core/cpu"
	"github.com/ysawyers/emufun/gameboy/core/memory"
)

func main() {
	cart, err := os.ReadFile("roms/T1.gb")
	if err != nil {
		log.Fatal(err)
	}
	memory.InitializeCartridge(cart)

	/* GAMEBOY DOCTOR INITIAL STATE */
	cpu.Processor.Registers[0] = 0x01
	cpu.Processor.Registers[1] = 0x00
	cpu.Processor.Registers[2] = 0x13
	cpu.Processor.Registers[3] = 0x00
	cpu.Processor.Registers[4] = 0xD8
	cpu.Processor.Registers[5] = 0xB0
	cpu.Processor.Registers[6] = 0x01
	cpu.Processor.Registers[7] = 0x4D
	cpu.Processor.SP = 0xFFFE
	cpu.Processor.PC = 0x0100

	f, err := os.Create("log.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	for {
		cpuState := fmt.Sprintf("A:%02X F:%02X B:%02X C:%02X D:%02X E:%02X H:%02X L:%02X SP:%04X PC:%04X PCMEM:%02X,%02X,%02X,%02X\n",
			cpu.Processor.Registers[0], cpu.Processor.Registers[5], cpu.Processor.Registers[1], cpu.Processor.Registers[2], cpu.Processor.Registers[3],
			cpu.Processor.Registers[4], cpu.Processor.Registers[6], cpu.Processor.Registers[7], cpu.Processor.SP, cpu.Processor.PC, memory.CPU.Read(cpu.Processor.PC),
			memory.CPU.Read(cpu.Processor.PC+1), memory.CPU.Read(cpu.Processor.PC+2), memory.CPU.Read(cpu.Processor.PC+3),
		)

		cpu.TickCPU() // fetch
		for cpu.Processor.State.Opcode != nil {
			cpu.TickCPU()
		}

		f.WriteString(cpuState)
	}
}
