mod utils;

use zecpaperlib::paper::generate_wallet;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub fn get_wallet() -> String {
    let w = generate_wallet(false, false, 2, 0, &[]);
    return w;
}
