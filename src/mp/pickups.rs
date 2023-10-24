use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(js_namespace = ["mp", "pickups"])]
extern "C" {
    pub fn new();
}
