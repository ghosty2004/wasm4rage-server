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

    mp::vehicles::forEach(&Closure::wrap(
        Box::new(|vehicle: &mp::vehicles::VehicleMp| {
            vehicle.set_numberPlate(String::from("RUST"));
            console_log(format!("Vehicle numberPlate {}", vehicle.numberPlate()));
        }) as Box<dyn FnMut(&mp::vehicles::VehicleMp)>,
    ));

    String::from("initialized")
}

//TODO: fix bug for vehicle create
