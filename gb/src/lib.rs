use wasm_bindgen::prelude::*;
use crate::internal::core::component::CPU;
extern crate console_error_panic_hook;
use std::panic;

mod internal;

const CYCLES_PER_FRAME: usize = 17556;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[macro_export]
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub fn u64_to_little_endian(val: u64) -> [u8; 4] {
    [(val & 0xFF) as u8, ((val & 0xFF00) >> 8) as u8, ((val & 0xFF0000) >> 16) as u8, ((val & 0xFF000000) >> 32) as u8]
}

pub fn u32_to_little_endian(val: u32) -> [u8; 3] {
    [(val & 0xFF) as u8, ((val & 0xFF00) >> 8) as u8, ((val & 0xFF0000) >> 16) as u8]
}

#[wasm_bindgen]
struct Emulator {
    core: CPU
}

#[wasm_bindgen]
impl Emulator {
    pub fn new() -> Emulator {   
        console_error_panic_hook::set_once();
        Emulator {
            core: CPU::default()
        }
    }

    pub fn load_bootrom(&mut self, bytes: Vec<u8>) {
        self.core.bus.mount_bootrom(bytes);
    }

    pub fn load_catridge(&mut self, bytes: Vec<u8>) {
        self.core.bus.load_cartridge(bytes);
    }

    pub fn render(&mut self, keypress: i8) -> Vec<u8> {
        self.core.next_frame(CYCLES_PER_FRAME, keypress).to_vec()
    }

    pub fn save_file(&mut self) -> Vec<u8> {
        self.core.create_save_file()
    }

    pub fn load_save_file(&mut self, bess_encoding: Vec<u8>) {
        self.core.load_save_file(bess_encoding);
    }
}