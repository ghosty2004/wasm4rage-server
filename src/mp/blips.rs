use wasm_bindgen::prelude::wasm_bindgen;

#[allow(non_snake_case)]
pub struct Options {
    pub alpha: i32,
    pub color: i32,
    pub dimension: i32,
    pub drawDistance: i32,
    pub name: String,
    pub rotation: i32,
    pub scale: i32,
    pub shortRange: bool,
}

#[wasm_bindgen(js_namespace = ["mp", "blips"])]
extern "C" {
    pub fn new(sprite: i32);
}
