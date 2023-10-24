# Web Assembly for RAGE:MP runtime
### Welcome to RAGE:MP, dear rust.

With this library you can write rage:mp gamemodes directly in rust with the help of web assembly.
The project is under construction, so feel free to contribuite.

#### Getting started:
1. `git clone https://github.com/ghosty2004/wasm-for-rage.git`
2. `cargo install wasm-pack`
3. `wasm-pack build --target nodejs --release`
5. create an empty resources in your ragemp server (for example `packages/rust/index.js` with the following content:
```js
const wasm = require("./wasm_for_rage.js");
mp.events.add('packagesLoaded', () => {
  console.log(wasm.main()); // you can view the return value of `wasm.main()` from `/src/lib.rs` (it return a String)
});
```
6. build the rust library with the following command `wasm-pack build --target nodejs --release`
7. move `wasm_for_rage_bg.wasm` and `wasm_for_rage.js` in your resources path `packages/rust/`
8. start server

#### Todo
- bind all methods from ragemp (embedded nodejs) to rust

##### You are probable asking why I started this proiect, so the first reason is that I'm currently learning rust, so this is my first project. The second one is that you can write your ragemp gamemode in rust or only a part of your gamemode.
