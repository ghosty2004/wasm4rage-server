use mp::Vector3;
use wasm_bindgen::prelude::*;

#[allow(unused)]
mod mp;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn console_log(value: String);
}

#[wasm_bindgen]
pub fn main() -> String {
    if let Some(veh) = mp::vehicles::new(String::from("infernus2"), Vector3::new(0.0, 0.0, 0.0)) {
        console_log(format!("Created vehicle {}", veh.id()));
    } else {
        console_log(String::from("Failed to create vehicle"));
    }

    mp::vehicles::for_each(|vehicle| {
        console_log(format!("Vehicle position x: {:?}", vehicle.position().x()));
    });

    // get closest vehicle
    console_log(format!(
        "Closest to position 0,0,0: {:?}",
        mp::vehicles::get_closest(Vector3::new(0.0, 0.0, 0.0)),
    ));

    String::from("initialized")
}

//TODO: fix bug for vehicle create
