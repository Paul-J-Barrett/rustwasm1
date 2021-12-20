use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
    pub fn confirm(s: &str);
}

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    let s = &JsValue::from_str(&format!("{}", add2(20, 22).to_string()));
    //alert(&format!("{}", add(20, 22).to_string()));
    //confirm(&format!("{}", add2(20, 22).to_string()));
    // Your code goes here!
    //console::log_1(&JsValue::from_str("Hello world!"));
    console::log_1(s);

    Ok(())
}

#[wasm_bindgen]
pub fn add2(a: i32, b: i32) -> i32 {
    a + b
}
