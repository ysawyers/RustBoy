import init, { Emulator } from "./pkg/gb.js";

const frameTimer = new Worker("frame_timer.js");

const REQUEST_FRAME = "REQUEST";
const RENDER_FRAME = "RENDER";
const WAIT_FOR_FRAME = "WAIT";

let debugMode = false; // when true debug panel is open
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
    this.emulator.load_catridge(new Uint8Array(cartridge));

    let debugPanelContainer = document.getElementById("debug-frame");

    frameTimer.postMessage(REQUEST_FRAME);

    frameTimer.onmessage = (e) => {
      if (e.data === RENDER_FRAME) {
        if (!super.isPaused) {
          let display = this.emulator.render(currentKeyPressed);
          if (debugMode) {
            debugPanelContainer.innerHTML = "";

            let debugPanel = this.emulator.debug_panel();
            for (let scanline = 0; scanline < 144; scanline++) {
              let offset = scanline * 3;

              let scanlineContainer = document.createElement("div");
              scanlineContainer.style.display = "flex";
              scanlineContainer.style.flexDirection = "row";

              let oamScanLength = document.createElement("div");
              oamScanLength.style.height = "1.5px";
              oamScanLength.style.width = `${(debugPanel[offset] / 456) * 100}%`;
              oamScanLength.style.backgroundColor = "blue";

              scanlineContainer.appendChild(oamScanLength);

              let drawLength = document.createElement("div");
              drawLength.style.height = "1.5px";
              drawLength.style.width = `${
                ((debugPanel[offset + 1] - debugPanel[offset]) / 456) * 100
              }%`;
              drawLength.style.backgroundColor = "green";

              scanlineContainer.appendChild(drawLength);

              let hblankLength = document.createElement("div");
              hblankLength.style.height = "1.5px";
              hblankLength.style.width = `${
                ((debugPanel[offset + 2] - debugPanel[offset + 1]) / 456) * 100
              }%`;
              hblankLength.style.backgroundColor = "black";

              scanlineContainer.appendChild(hblankLength);

              debugPanelContainer.appendChild(scanlineContainer);
            }
          }
          for (let row = 0; row < 144; row++) {
            for (let col = 0; col < 160; col++) {
              this.ctx.fillStyle = this.colorPallete[display[row * 160 + col]];
              this.ctx.fillRect(
                col * this.canvasScale,
                row * this.canvasScale,
                this.canvasScale,
                this.canvasScale
              );
            }
          }
          frameTimer.postMessage(REQUEST_FRAME);
        }
      } else if (e.data === WAIT_FOR_FRAME) {
        frameTimer.postMessage(REQUEST_FRAME);
      }
    };
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

  const toggleDebugPanelButton = document.getElementById("toggle-debug-panel-button");
  toggleDebugPanelButton.addEventListener("click", function (e) {
    let panel = document.getElementById("debug-panel");

    debugMode = !debugMode;
    if (debugMode) {
      panel.style.display = "block";
    } else {
      panel.style.display = "none";
    }
  });
});

window.addEventListener("keyup", () => {
  currentKeyPressed = -1;
});
