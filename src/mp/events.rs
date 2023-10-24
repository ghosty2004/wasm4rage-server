use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(js_namespace = ["mp", "events"])]
extern "C" {
    pub fn remove(eventName: String);
    pub fn reset();
}
