let currentKeypress = -1;

function formatLinearScreen(screen) {
  let display = [];

  let currentRow = [];
  for (let i = 0; i < screen.length; i++) {
    if (screen[i] !== -1) {
      currentRow.push(screen[i]);
    } else {
      display.push(currentRow);
      currentRow = [];
    }
  }

  return display;
}

class Emulator {
  constructor(canvas, currentGame, canvasScale) {
    this.currentGame = currentGame;
    this.ctx = canvas.getContext("2d");
    this.currentKeypress = -1;
    this.canvasScale = canvasScale;
  }

  changeGame(currentGame) {
    if (this.currentGame) {
      cancelAnimationFrame(currentGame);
    }
    this.currentGame = currentGame;
  }

  changeCanvasDimensions(width, height) {
    this.ctx.canvas.width = width * this.canvasScale;
    this.ctx.canvas.height = height * this.canvasScale;
  }
}

class Gameboy extends Emulator {
  constructor(canvas, currentGame, canvasScale) {
    super(canvas, currentGame, canvasScale);
    super.changeCanvasDimensions(160, 144);
    this.colorPallete = ["#FFFFFF", "#AAAAAA", "#555555", "#000000"];
  }

  run(cartridge) {
    fetch("binaries/DMG_ROM.bin")
      .then((res) => res.arrayBuffer())
      .then(async (buffer) => {
        initializeCartridge(...cartridge);
        initializeBootFile(...new Uint8Array(buffer));

        let ctx = this.ctx;
        let colorPallete = this.colorPallete;
        let canvasScale = this.canvasScale;
        let currentGame;

        function animate() {
          // let start = new Date().getTime();
          let display = formatLinearScreen(fetchNextGameState());
          // console.log("TOOK", new Date().getTime() - start, "MILLISECONDS TO RENDER FRAME!");
          for (let row = 0; row < display.length; row++) {
            for (let col = 0; col < display[row].length; col++) {
              ctx.fillStyle = colorPallete[display[row][col]];
              ctx.fillRect(col * canvasScale, row * canvasScale, canvasScale, canvasScale);
            }
          }
          currentGame = requestAnimationFrame(animate);
        }
        animate();

        this.changeGame(currentGame);
      });
  }
}

class Chip8 extends Emulator {
  constructor(canvas, currentGame, canvasScale) {
    super(canvas, currentGame, canvasScale);
    super.changeCanvasDimensions(64, 32);

    window.onkeydown = function (e) {
      if (e.keyCode === 49) currentKeypress = 0; // 1
      if (e.keyCode === 50) currentKeypress = 1; // 2
      if (e.keyCode === 51) currentKeypress = 2; // 3
      if (e.keyCode === 52) currentKeypress = 3; // 4
      if (e.keyCode === 81) currentKeypress = 4; // Q
      if (e.keyCode === 87) currentKeypress = 5; // W
      if (e.keyCode === 69) currentKeypress = 6; // E
      if (e.keyCode === 82) currentKeypress = 7; // R
      if (e.keyCode === 65) currentKeypress = 8; // A
      if (e.keyCode === 83) currentKeypress = 9; // S
      if (e.keyCode === 68) currentKeypress = 10; // D
      if (e.keyCode === 70) currentKeypress = 11; // F
      if (e.keyCode === 90) currentKeypress = 12; // Z
      if (e.keyCode === 88) currentKeypress = 13; // X
      if (e.keyCode === 67) currentKeypress = 14; // C
      if (e.keyCode === 86) currentKeypress = 15; // V
    };
  }

  run(bytes) {
    initializeGame(...bytes);

    let ctx = this.ctx;
    let canvasScale = this.canvasScale;

    let currentGame;

    function animate() {
      for (let i = 0; i < 7; i++) {
        let display = formatLinearScreen(fetchNextGameState(currentKeypress));
        for (let row = 0; row < display.length; row++) {
          for (let col = 0; col < display[row].length; col++) {
            if (display[row][col]) {
              ctx.fillStyle = "rgb(255, 255, 255)";
            } else {
              ctx.fillStyle = "rgb(0, 0, 0)";
            }
            ctx.fillRect(col * canvasScale, row * canvasScale, canvasScale, canvasScale);
          }
        }
      }
      currentGame = requestAnimationFrame(animate);
    }
    currentGame = requestAnimationFrame(animate);

    this.changeGame(currentGame);
  }
}

if (WebAssembly) {
  if (!WebAssembly.instantiateStreaming) {
    WebAssembly.instantiateStreaming = async (resp, importObject) => {
      const source = await (await resp).arrayBuffer();
      return await WebAssembly.instantiate(source, importObject);
    };
  }

  const canvas = document.getElementById("emulator");

  let emulatorSelection = document.getElementById("emulator-select");
  emulatorSelection.addEventListener("change", function (e) {
    const emulatorPrefix = e.target.value;

    initializeRuntime(emulatorPrefix)
      .then((wasm) => {
        go.run(wasm);

        let emulator;
        if (emulatorPrefix === "chip8") {
          emulator = new Chip8(canvas, null, 15); // TODO: pass in wasm object
        } else if (emulatorPrefix === "gb") {
          emulator = new Gameboy(canvas, null, 3); // TODO: pass in wasm object
        }

        let romSelection = document.getElementById(`rom-select-${emulatorPrefix}`);
        romSelection.style.removeProperty("display");

        romSelection.addEventListener("change", function (e) {
          const game = e.target.value;

          fetch(`roms/${emulatorPrefix}/${game}`)
            .then((res) => res.arrayBuffer())
            .then(async (buffer) => {
              emulator.run(new Uint8Array(buffer));
            })
            .catch((err) => {
              console.log("FILE DOES NOT EXIST", err);
            });
        });
      })
      .catch((err) => {
        console.log(err);
      });
  });
}

window.onkeyup = function (e) {
  currentKeypress = -1;
};
