use std::fmt::Debug;

use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::HtmlImageElement;

pub struct LoadHandle {
    closure: Closure<dyn Fn()>,
    element: HtmlImageElement,
}

impl LoadHandle {
    pub fn new<E: JsCast + Debug, F: Fn() + 'static>(element: E, function: F) -> Self {
        let closure = Closure::new(function);
        let element = element.dyn_into::<HtmlImageElement>().unwrap();
        element.set_onload(Some(closure.as_ref().unchecked_ref()));
        LoadHandle { closure, element }
    }
}

impl Drop for LoadHandle {
    fn drop(&mut self) {
        self.element
            .remove_event_listener_with_callback("load", &self.closure.as_ref().unchecked_ref())
            .unwrap();
    }
}
