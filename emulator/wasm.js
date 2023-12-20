const go = new Go();

/**
 * Returns wasm instance for chosen emulator
 * @constructor
 * @param {string} emulator - Device being emulated.
 * @throws Will through an error if no support for Web Assembly.
 */
async function initializeRuntime(emulator) {
  if (WebAssembly) {
    if (!WebAssembly.instantiateStreaming) {
      const bytes = await (await fetch(`binaries/${emulator}.wasm`)).arrayBuffer();
      const obj = await WebAssembly.instantiate(bytes, go.importObject);
      return obj.instance;
    }
    const obj = await WebAssembly.instantiateStreaming(
      fetch(`binaries/${emulator}.wasm`),
      go.importObject
    );
    return obj.instance;
  }
  throw new Error("Browser does not support Web Assembly!");
}
