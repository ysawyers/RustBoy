use wasm_bindgen::prelude::*;
use crate::internal::core::component::CPU;

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

#[wasm_bindgen]
struct Emulator {
    core: CPU
}

#[wasm_bindgen]
impl Emulator {
    pub fn new() -> Emulator {   
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

    pub fn render(&mut self) -> Vec<u8> {
        return self.core.next_frame(CYCLES_PER_FRAME).to_vec();
    }
}