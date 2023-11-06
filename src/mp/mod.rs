use wasm_bindgen::prelude::wasm_bindgen;

pub mod blips;
pub mod checkpoints;
pub mod colshapes;
pub mod config;
pub mod events;
pub mod labels;
pub mod markers;
pub mod objects;
pub mod pickups;
pub mod players;
pub mod vehicles;

#[wasm_bindgen(js_namespace = mp)]
extern "C" {
    pub fn joaat(str: String) -> f64;
}

#[wasm_bindgen(js_namespace = mp, js_name = Vector3)]
extern "C" {
    #[derive(Debug)]
    pub type Vector3;

    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64, z: f64) -> Vector3;

    #[wasm_bindgen(method, getter)]
    pub fn x(this: &Vector3) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_x(this: &Vector3, x: f64);

    #[wasm_bindgen(method, getter)]
    pub fn y(this: &Vector3) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_y(this: &Vector3, y: f64);

    #[wasm_bindgen(method, getter)]
    pub fn z(this: &Vector3) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_z(this: &Vector3, z: f64);
}
