package cpu

import (
	"log"

	"github.com/ysawyers/emufun/gameboy/core/memory"
)

type Cpu struct {
	Registers             [8]byte
	Ime                   int
	ShouldEnableInterrupt int
	SP                    uint16
	PC                    uint16
	State                 cpuTickState
}

type cpuTickState struct {
	Opcode      opcode
	buffer      [2]byte
	currentStep int
	isNotNull   bool
}

type opcode struct {
	label    string
	value    byte
	steps    []stepFunc
	isPrefix bool
}

type stepFunc = func() bool

var Processor Cpu

func fetch() opcode {
	opcode, ok := opcodeTable[memory.CPU.Read(Processor.PC)]
	if !ok {
		log.Fatalf("Not implemented 0x%02X", memory.CPU.Read(Processor.PC))
	}
	Processor.PC++
	return opcode
}

func fetchPrefixed() opcode {
	opcode, ok := opcodeTablePrefix[memory.CPU.Read(Processor.PC)]
	if !ok {
		log.Fatalf("Not implemented CB -> 0x%02X", memory.CPU.Read(Processor.PC))
	}
	Processor.PC++
	return opcode
}

func TickCPU() {
	if !Processor.State.isNotNull {
		Processor.State.Opcode = fetch()
		Processor.State.isNotNull = true
	}

	if Processor.State.Opcode.isPrefix {
		Processor.State.Opcode = fetchPrefixed()
		return // early return to add additional cycle to prefixed instruction
	}

	if !Processor.State.Opcode.steps[Processor.State.currentStep]() {
		if Processor.ShouldEnableInterrupt == 1 { // delay EI by 1 instruction
			Processor.ShouldEnableInterrupt++
		} else if Processor.ShouldEnableInterrupt == 2 {
			Processor.Ime = 1
			Processor.ShouldEnableInterrupt = 0
		}
		Processor.State.currentStep = 0
		Processor.State.isNotNull = false
	} else {
		Processor.State.currentStep++
	}
}
