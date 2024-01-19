import init, { Emulator } from "./pkg/gb.js";

const frameTimer = new Worker("frame_timer.js");

const REQUEST_FRAME = "REQUEST";
const RENDER_FRAME = "RENDER";
const WAIT_FOR_FRAME = "WAIT";

let currentKeyPressed = -1;

class Display {
  constructor(canvas, currentGame, canvasScale) {
    this.currentGame = currentGame;
    this.ctx = canvas.getContext("2d");
    this.canvasScale = canvasScale;
    this.isPaused = false;
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

// ["#9bbc0f", "#8bac0f", "#306230", "#0f380f"] ["#FFFFFF", "#AAAAAA", "#555555", "#000000"]
class Gameboy extends Display {
  constructor(canvas, currentGame, canvasScale) {
    super(canvas, currentGame, canvasScale);
    super.changeCanvasDimensions(160, 144);
    this.emulator = Emulator.new();
    this.colorPallete = ["#FFFFFF", "#AAAAAA", "#555555", "#000000"];

    window.addEventListener("keydown", (e) => {
      switch (e.code) {
        case "ArrowUp":
          currentKeyPressed = 1;
          break;
        case "ArrowLeft":
          currentKeyPressed = 2;
          break;
        case "ArrowDown":
          currentKeyPressed = 3;
          break;
        case "ArrowRight":
          currentKeyPressed = 4;
          break;
        case "KeyQ": // A
          currentKeyPressed = 5;
          break;
        case "KeyW": // B
          currentKeyPressed = 6;
          break;
        case "Enter": // START
          currentKeyPressed = 7;
          break;
        case "Escape": // SELECT
          currentKeyPressed = 8;
          break;
        default:
        // console.log(`Invalid key input: ${e.code}`);
      }
    });
  }

  pause() {
    super.isPaused = true;
  }

  resume() {
    super.isPaused = false;
    frameTimer.postMessage(REQUEST_FRAME);
  }

  run(cartridge) {
    fetch("binaries/DMG_ROM.bin")
      .then((res) => res.arrayBuffer())
      .then((boot) => {
        this.emulator.load_bootrom(new Uint8Array(boot));
        this.emulator.load_catridge(new Uint8Array(cartridge));

        let emulator = this.emulator;
        let ctx = this.ctx;
        let colorPallete = this.colorPallete;
        let canvasScale = this.canvasScale;

        frameTimer.postMessage(REQUEST_FRAME);

        frameTimer.onmessage = (e) => {
          if (e.data === RENDER_FRAME) {
            if (!super.isPaused) {
              let display = emulator.render(currentKeyPressed);
              for (let row = 0; row < 144; row++) {
                for (let col = 0; col < 160; col++) {
                  ctx.fillStyle = colorPallete[display[row * 160 + col]];
                  ctx.fillRect(col * canvasScale, row * canvasScale, canvasScale, canvasScale);
                }
              }
              frameTimer.postMessage(REQUEST_FRAME);
            }
          } else if (e.data === WAIT_FOR_FRAME) {
            frameTimer.postMessage(REQUEST_FRAME);
          }
        };
      });
  }
}

init().then(() => {
  const canvas = document.getElementById("emulator");

  const gameboy = new Gameboy(canvas, null, 4);

  const romUpload = document.getElementById("rom-upload");
  romUpload.addEventListener("change", function (e) {
    var reader = new FileReader();

    reader.onload = function () {
      var arrayBuffer = this.result;
      gameboy.run(arrayBuffer);
    };
    reader.readAsArrayBuffer(this.files[0]);
  });

  const loadSave = document.getElementById("load-save");
  loadSave.addEventListener("change", function (e) {
    var reader = new FileReader();

    reader.onload = function () {
      var arrayBuffer = this.result;
      gameboy.emulator.load_save_file(new Uint8Array(arrayBuffer));
    };
    reader.readAsArrayBuffer(this.files[0]);
  });

  const saveButton = document.getElementById("save-button");
  saveButton.addEventListener("click", function (e) {
    const state = gameboy.emulator.save_file();

    const blob = new Blob([state]);
    const link = document.createElement("a");
    link.href = URL.createObjectURL(blob);
    link.download = `${new Date().getTime()}.sav`;

    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  });
});

window.addEventListener("keyup", () => {
  currentKeyPressed = -1;
});
