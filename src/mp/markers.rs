use wasm_bindgen::prelude::wasm_bindgen;

pub struct Options {
    pub dimension: i32,
    pub visible: bool,
}

#[wasm_bindgen(js_namespace = ["mp", "markers"])]
extern "C" {}
