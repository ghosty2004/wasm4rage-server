use wasm_bindgen::prelude::wasm_bindgen;

#[allow(non_snake_case)]
pub struct Options {
    pub dimension: i32,
    pub drawDistance: i32,
    pub font: i32,
    pub los: bool,
}

#[wasm_bindgen(js_namespace = ["mp", "labels"])]
extern "C" {}
