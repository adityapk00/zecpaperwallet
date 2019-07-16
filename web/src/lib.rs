mod utils;

use zecpaperlib::paper::{generate_wallet, double_sha256};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub fn get_wallet(zaddrs: u32, taddrs: u32, entropy: String) -> String {    
    let w = generate_wallet(false, false, zaddrs, taddrs, &double_sha256(entropy.as_bytes()));
    return w;
}
