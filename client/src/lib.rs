mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;
use web_sys::console::log;
use web_sys::console::log_1;

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
    let window = web_sys::window().expect("Could not get Window");
    let document = window.document().expect("Could not get Document");
    let canvas = document
        .get_element_by_id(&canvas_id)
        .expect(format!("Could not get element with id: \"{}\"", canvas_id).as_str())
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect(format!("Element with id: \"{}\" is not a canvas", canvas_id).as_str());

    let context = canvas
        .get_context("2d")
        .expect("Could not get the 2d context")
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.fill_rect(0., 0., 100., 100.);
    log_1(&JsValue::from_f64(canvas.width() as f64));
    // let game_state:
}
