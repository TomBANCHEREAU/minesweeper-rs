pub mod handles;
pub mod image;
pub mod utils;
pub mod viewport;

use utils::set_panic_hook;
use viewport::Viewport;
use viewport::ViewportOptions;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }

#[wasm_bindgen]
pub fn start(canvas_id: String) {
    set_panic_hook();
    let _viewport = Viewport::new(ViewportOptions { canvas_id });
    // log_1(&JsValue::from_f64(canvas.width() as f64));
    // let game_state:
}
