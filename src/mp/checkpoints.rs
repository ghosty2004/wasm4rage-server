use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

pub struct Options {
    pub dimension: i32,
    pub visible: bool,
}

#[wasm_bindgen(js_namespace = ["mp", "checkpoints"])]
extern "C" {
    pub fn new(checkpoint_type: i32);
}
