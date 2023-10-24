use wasm_bindgen::prelude::wasm_bindgen;

pub struct Options {
    pub alpha: i32,
    pub dimension: i32,
}

#[wasm_bindgen(js_namespace = ["mp", "objects"])]
extern "C" {
    pub fn new(model: String);
}
