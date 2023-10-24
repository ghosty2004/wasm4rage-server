use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen(js_namespace = ["mp", "players"])]
extern "C" {
    pub fn broadcast(str: String);
    pub fn at(index: i32) -> JsValue;
}
