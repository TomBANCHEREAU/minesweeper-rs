// pub mod controller;
pub mod handles;
pub mod image;
pub mod socket;
pub mod utils;
pub mod viewport;

use utils::set_panic_hook;
use viewport::Viewport;
use viewport::ViewportOptions;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn start(canvas_id: String, lobby_id: String) {
    set_panic_hook();
    Viewport::new(ViewportOptions {
        canvas_id,
        lobby: lobby_id,
    })
    .unwrap();
}
