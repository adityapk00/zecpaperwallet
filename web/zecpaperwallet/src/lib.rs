mod utils;

use zecpaperlib::paper::generate_wallet;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() -> String {
    let w = generate_wallet(false, false, 1, 0, &[]);

    // alert(&format!("Hello, zecpaperwallet! {}\n", w));
    return w;
}
