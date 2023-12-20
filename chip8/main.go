package main

import (
	"syscall/js"

	"github.com/ysawyers/emufun/chip8/interpreter"
)

func initializeGame(this js.Value, args []js.Value) interface{} {
	var file []byte

	for i := 0; i < len(args); i++ {
		file = append(file, byte(args[i].Int()))
	}

	interpreter.InitializeGame(file)

	return nil
}

func fetchNextGameState(this js.Value, args []js.Value) interface{} {
	var screen []interface{}

	disp, _ := interpreter.FetchNextGameState(args[0].Int())

	for row := 0; row < len(disp); row++ {
		for col := 0; col < len(disp[row]); col++ {
			if disp[row][col] {
				screen = append(screen, 1)
			} else {
				screen = append(screen, 0)
			}
		}
		screen = append(screen, -1) // DELIMETER FOR NEXT ROW
	}

	return screen
}

func main() {
	block := make(chan bool)
	js.Global().Set("initializeGame", js.FuncOf(initializeGame))
	js.Global().Set("fetchNextGameState", js.FuncOf(fetchNextGameState))
	<-block
}
