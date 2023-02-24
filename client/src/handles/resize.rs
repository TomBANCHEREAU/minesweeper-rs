use wasm_bindgen::{prelude::Closure, JsCast};

use crate::utils::get_window;

pub struct ResizeHandle {
    closure: Closure<dyn Fn()>,
}

impl ResizeHandle {
    pub fn new<F: Fn() + 'static>(function: F) -> Self {
        let closure = Closure::new(function);
        get_window().set_onresize(Some(closure.as_ref().unchecked_ref()));
        ResizeHandle { closure }
    }
}

impl Drop for ResizeHandle {
    fn drop(&mut self) {
        get_window()
            .remove_event_listener_with_callback("resize", &self.closure.as_ref().unchecked_ref())
            .unwrap();
    }
}
