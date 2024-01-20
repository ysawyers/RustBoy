/* tslint:disable */
/* eslint-disable */
/**
*/
export class Emulator {
  free(): void;
/**
* @returns {Emulator}
*/
  static new(): Emulator;
/**
* @param {Uint8Array} bytes
*/
  load_bootrom(bytes: Uint8Array): void;
/**
* @param {Uint8Array} bytes
*/
  load_catridge(bytes: Uint8Array): void;
/**
* @param {number} keypress
* @returns {Uint8Array}
*/
  render(keypress: number): Uint8Array;
/**
* @returns {Uint32Array}
*/
  debug_panel(): Uint32Array;
/**
* @returns {Uint8Array}
*/
  save_file(): Uint8Array;
/**
* @param {Uint8Array} bess_encoding
*/
  load_save_file(bess_encoding: Uint8Array): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_emulator_free: (a: number) => void;
  readonly emulator_new: () => number;
  readonly emulator_load_bootrom: (a: number, b: number, c: number) => void;
  readonly emulator_load_catridge: (a: number, b: number, c: number) => void;
  readonly emulator_render: (a: number, b: number, c: number) => void;
  readonly emulator_debug_panel: (a: number, b: number) => void;
  readonly emulator_save_file: (a: number, b: number) => void;
  readonly emulator_load_save_file: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
