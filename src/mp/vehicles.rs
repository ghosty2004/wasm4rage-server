use js_sys::{Array, ArrayBuffer, ArrayIter, Function, Int32Array};
use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsObject, JsValue,
};

use super::Vector3;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    pub type VehicleMp;

    #[wasm_bindgen(method, getter)]
    pub fn alpha(this: &VehicleMp) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn data(this: &VehicleMp) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn dimension(this: &VehicleMp) -> i32;

    #[wasm_bindgen(method, setter)]
    pub fn set_dimension(this: &VehicleMp, dimension: i32);

    #[wasm_bindgen(method, getter)]
    pub fn model(this: &VehicleMp) -> i32;

    #[wasm_bindgen(method, setter)]
    pub fn set_model(this: &VehicleMp, model: i32) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn position(this: &VehicleMp) -> crate::Vector3;

    #[wasm_bindgen(method, setter)]
    pub fn set_position(this: &VehicleMp, position: crate::Vector3);

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &VehicleMp) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn getVariable(this: &VehicleMp, key: String) -> Option<String>;

    #[wasm_bindgen(method)]
    pub fn setVariable(this: &VehicleMp, key: String, value: JsValue);

    #[wasm_bindgen(method, getter)]
    pub fn destroy(this: &VehicleMp);

    #[wasm_bindgen(method, getter)]
    pub fn dist(this: &VehicleMp, position: crate::Vector3) -> f32;

    #[wasm_bindgen(method, getter)]
    pub fn distSquared(this: &VehicleMp, position: crate::Vector3) -> f32;

    #[wasm_bindgen(method, getter)]
    pub fn bodyHealth(this: &VehicleMp) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn brake(this: &VehicleMp) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn engine(this: &VehicleMp) -> bool;

    #[wasm_bindgen(method, setter)]
    pub fn set_engine(this: &VehicleMp, engine: bool);

    #[wasm_bindgen(method, getter)]
    pub fn engineHealth(this: &VehicleMp) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn dashboardColor(this: &VehicleMp) -> i32;

    #[wasm_bindgen(method, setter)]
    pub fn set_dashboardColor(this: &VehicleMp, color: i32);

    #[wasm_bindgen(method, getter)]
    pub fn dead(this: &VehicleMp) -> bool;

    #[wasm_bindgen(method, setter)]
    pub fn set_dead(this: &VehicleMp, dead: bool);

    #[wasm_bindgen(method, getter)]
    pub fn highbeams(this: &VehicleMp) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn horn(this: &VehicleMp) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn livery(this: &VehicleMp) -> i32;

    #[wasm_bindgen(method, setter)]
    pub fn set_livery(this: &VehicleMp, livery: i32);

    #[wasm_bindgen(method, getter)]
    pub fn locked(this: &VehicleMp) -> bool;

    #[wasm_bindgen(method, setter)]
    pub fn set_locked(this: &VehicleMp, locked: bool);

    #[wasm_bindgen(method, getter)]
    pub fn movable(this: &VehicleMp) -> bool;

    #[wasm_bindgen(method, setter)]
    pub fn set_movable(this: &VehicleMp, movable: bool);

    #[wasm_bindgen(method, getter)]
    pub fn neonEnabled(this: &VehicleMp) -> bool;

    #[wasm_bindgen(method, setter)]
    pub fn set_neonEnabled(this: &VehicleMp, neonEnabled: bool);

    #[wasm_bindgen(method, getter)]
    pub fn numberPlate(this: &VehicleMp) -> String;

    #[wasm_bindgen(method, setter)]
    pub fn set_numberPlate(this: &VehicleMp, numberPlate: String);

    #[wasm_bindgen(method, getter)]
    pub fn numberPlateType(this: &VehicleMp) -> i32;

    #[wasm_bindgen(method, setter)]
    pub fn set_numberPlateType(this: &VehicleMp, numberPlateType: i32);

    #[wasm_bindgen(method, getter)]
    pub fn pearlescentColor(this: &VehicleMp) -> i32;

    #[wasm_bindgen(method, setter)]
    pub fn set_pearlescentColor(this: &VehicleMp, pearlescentColor: i32);

    #[wasm_bindgen(method, getter)]
    pub fn rocketBoost(this: &VehicleMp) -> bool;

    // #[wasm_bindgen(method, getter)]
    // pub fn rotation(this: &VehicleMp) -> [f32; 3];

    // #[wasm_bindgen(method, setter)]
    // pub fn set_rotation(this: &VehicleMp, rotation: [f32; 3]);

    #[wasm_bindgen(method, getter)]
    pub fn siren(this: &VehicleMp) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn steerAngle(this: &VehicleMp) -> f32;

    #[wasm_bindgen(method, getter)]
    pub fn taxiLights(this: &VehicleMp) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn trimColor(this: &VehicleMp) -> i32;

    // #[wasm_bindgen(method, getter)]
    // pub fn velocity(this: &VehicleMp) -> [f32; 3];

    #[wasm_bindgen(method, getter)]
    pub fn wheelColor(this: &VehicleMp) -> i32;

    #[wasm_bindgen(method, setter)]
    pub fn set_wheelColor(this: &VehicleMp, wheelColor: i32);

    #[wasm_bindgen(method, getter)]
    pub fn wheelType(this: &VehicleMp) -> i32;

    #[wasm_bindgen(method, setter)]
    pub fn set_wheelType(this: &VehicleMp, wheelType: i32);

    #[wasm_bindgen(method, getter)]
    pub fn windowTint(this: &VehicleMp) -> i32;

    #[wasm_bindgen(method, setter)]
    pub fn set_windowTint(this: &VehicleMp, windowTint: i32);

    #[wasm_bindgen(method, getter)]
    pub fn extras(this: &VehicleMp) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn heading(this: &VehicleMp) -> f32;

    #[wasm_bindgen(method, getter)]
    pub fn mods(this: &VehicleMp) -> Int32Array;

    #[wasm_bindgen(method, getter)]
    pub fn quaternion(this: &VehicleMp) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn streamedPlayers(this: &VehicleMp) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn trailer(this: &VehicleMp) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn traileredBy(this: &VehicleMp) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn explode(this: &VehicleMp);

    #[wasm_bindgen(method, getter)]
    pub fn getColor(this: &VehicleMp, id: i32) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn getColorRGB(this: &VehicleMp, id: i32) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn getExtra(this: &VehicleMp, index: i32) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn getMod(this: &VehicleMp, modType: i32) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn getNeonColor(this: &VehicleMp) -> Int32Array;

    #[wasm_bindgen(method, getter)]
    pub fn getOccupant(this: &VehicleMp, seat: i32) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn getOccupants(this: &VehicleMp) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn getPaint(this: &VehicleMp, id: i32) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn isStreamed(this: &VehicleMp, player: JsValue) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn playScenario(this: &VehicleMp, scenario: String);

    #[wasm_bindgen(method, getter)]
    pub fn repair(this: &VehicleMp);

    #[wasm_bindgen(method, getter)]
    pub fn setColor(this: &VehicleMp, primary: i32, secondary: i32);

    #[wasm_bindgen(method, getter)]
    pub fn setColorRGB(
        this: &VehicleMp,
        red1: i32,
        green1: i32,
        blue1: i32,
        red2: i32,
        green2: i32,
        blue2: i32,
    );

    #[wasm_bindgen(method, getter)]
    pub fn setExtra(this: &VehicleMp, index: i32, value: bool);

    #[wasm_bindgen(method, getter)]
    pub fn setMod(this: &VehicleMp, modType: i32, modIndex: i32);

    #[wasm_bindgen(method, getter)]
    pub fn setNeonColor(this: &VehicleMp, red: i32, green: i32, blue: i32);

    #[wasm_bindgen(method, getter)]
    pub fn setPaint(
        this: &VehicleMp,
        primaryType: i32,
        primaryColor: i32,
        secondaryType: i32,
        secondaryColor: i32,
    );

    #[wasm_bindgen(method, getter)]
    pub fn setOccupant(this: &VehicleMp, seat: i32, player: JsValue);

    #[wasm_bindgen(method, getter)]
    pub fn spawn(this: &VehicleMp, position: JsValue, heading: f32);
}

#[wasm_bindgen(js_namespace = ["mp", "vehicles"])]
extern "C" {
    pub fn new(model: String, position: crate::Vector3) -> Option<VehicleMp>;
    pub fn at(index: i32) -> VehicleMp;
    pub fn exists(vehicle: VehicleMp) -> bool;

    fn forEach(callback: &Closure<dyn Fn(&VehicleMp)>);
    fn forEachInRange(
        position: JsValue,
        range: f64,
        dimension: i32,
        callback: &Closure<dyn Fn(&VehicleMp)>,
    );
    fn forEachInDimension(dimension: i32, callback: &Closure<dyn Fn(&VehicleMp)>);

    #[wasm_bindgen(js_name = "getClosest")]
    pub fn get_closest(position: Vector3) -> Option<VehicleMp>;

    #[wasm_bindgen(js_name = "toArray")]
    pub fn to_array() -> Array;
}

pub fn for_each<F>(mut callback: F)
where
    F: Fn(&VehicleMp) + 'static,
{
    let closure = Closure::wrap(
        Box::new(move |vehicle: &VehicleMp| callback(vehicle)) as Box<dyn Fn(&VehicleMp)>
    );
    forEach(&closure);
    closure.forget();
}

pub fn for_each_in_range<F>(position: JsValue, range: f64, dimension: i32, mut callback: F)
where
    F: Fn(&VehicleMp) + 'static,
{
    let closure = Closure::wrap(
        Box::new(move |vehicle: &VehicleMp| callback(vehicle)) as Box<dyn Fn(&VehicleMp)>
    );
    forEachInRange(position, range, dimension, &closure);
    closure.forget();
}

pub fn for_each_in_dimension<F>(dimension: i32, mut callback: F)
where
    F: Fn(&VehicleMp) + 'static,
{
    let closure = Closure::wrap(
        Box::new(move |vehicle: &VehicleMp| callback(vehicle)) as Box<dyn Fn(&VehicleMp)>
    );
    forEachInDimension(dimension, &closure);
    closure.forget();
}
