# Web Assembly for RAGE:MP runtime
### Welcome to RAGE:MP, dear rust.

With this library you can write rage:mp gamemodes directly in rust with the help of web assembly.
The project is under construction, so feel free to contribuite.

#### Getting started:
1. `git clone https://github.com/ghosty2004/wasm-for-rage.git`
2. `cargo install wasm-pack`
3. `wasm-pack build --target nodejs --release`
5. create an empty resources in your ragemp server (for example `packages/rust/index.js`) with the following content:
```js
const wasm = require("./wasm_for_rage.js");
mp.events.add('packagesLoaded', () => {
  // create some vehicles to test rust's forEach :)
  for(let i = 0; i < 10; i++) mp.vehicles.new("infernus2", new mp.Vector3(0, 0, 0));

  // initialize the main of rust lib
  console.log(wasm.main()); // you can view the return value of `wasm.main()` from `/src/lib.rs` (it return a String)
});
```
6. build the rust library with the following command `wasm-pack build --target nodejs --release`
7. move `wasm_for_rage_bg.wasm` and `wasm_for_rage.js` in your resources path `packages/rust/`
8. start server

#### Todo
- bind all methods from ragemp (embedded nodejs) to rust

#### Example of code in rust
```rust
use wasm_bindgen::prelude::*;
use mp::Vector3;

#[allow(unused)]
mod mp;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn console_log(value: String);
}

#[wasm_bindgen]
pub fn main() -> String {
    // create a vehicle
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

    String::from("initialized") // this will be returned to node runtime (in our case the node from ragemp where it's embedded)
}
```

##### You are probable asking why I started this proiect, so the first reason is that I'm currently learning rust. The second one is that you can write your ragemp gamemode in rust or only a part of your gamemode.
