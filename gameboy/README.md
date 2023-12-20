## To compile

```
GOARCH=wasm GOOS=js go build -o ../emulator/binaries/gb.wasm -ldflags="-s -w"
```
