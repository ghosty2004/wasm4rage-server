use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(js_namespace = ["mp", "colshapes"])]
extern "C" {
    pub fn newCircle(x: i32, y: i32, range: i32);
    pub fn newCuboid(x: i32, y: i32, z: i32, width: i32, depth: i32, height: i32);
    pub fn newRectangle(x: i32, y: i32, width: i32, height: i32);
    pub fn newSphere(x: i32, y: i32, z: i32, range: i32);
    pub fn newTube(x: i32, y: i32, z: i32, range: i32, height: i32);
}
