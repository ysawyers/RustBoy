use wasm_bindgen::prelude::*;
use crate::internal::core::component::CPU;

mod internal;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}