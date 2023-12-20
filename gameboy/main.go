package main

import (
	"syscall/js"

	"github.com/ysawyers/emufun/gameboy/core"
	"github.com/ysawyers/emufun/gameboy/core/memory"
)

func initializeBootFile(this js.Value, args []js.Value) interface{} {
	var file []byte

	for i := 0; i < len(args); i++ {
		file = append(file, byte(args[i].Int()))
	}

	memory.InitializeBootFile(file)

	return nil
}

func initializeCartridge(this js.Value, args []js.Value) interface{} {
	var file []byte

	for i := 0; i < len(args); i++ {
		file = append(file, byte(args[i].Int()))
	}

	memory.InitializeCartridge(file)

	return nil
}

func fetchNextGameState(this js.Value, args []js.Value) interface{} {
	var screen [23184]interface{}

	lcd := core.Tick()
	i := 0

	for row := 0; row < len(lcd); row++ {
		for col := 0; col < len(lcd[row]); col++ {
			screen[i] = lcd[row][col]
			i++
		}
		screen[i] = -1
		i++
	}

	return screen[:]
}

func main() {
	block := make(chan bool)
	js.Global().Set("initializeBootFile", js.FuncOf(initializeBootFile))
	js.Global().Set("initializeCartridge", js.FuncOf(initializeCartridge))
	js.Global().Set("fetchNextGameState", js.FuncOf(fetchNextGameState))
	<-block
}
