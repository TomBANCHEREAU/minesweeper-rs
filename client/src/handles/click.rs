use std::fmt::Debug;

use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{HtmlElement, MouseEvent};

pub struct ClickHandle {
    closure: Closure<dyn Fn(MouseEvent)>,
    element: HtmlElement,
}

impl ClickHandle {
    pub fn new<E: JsCast + Debug, F: Fn(MouseEvent) + 'static>(element: E, function: F) -> Self {
        let closure = Closure::new(function);
        let element = element.dyn_into::<HtmlElement>().unwrap();
        element.set_onclick(Some(closure.as_ref().unchecked_ref()));
        ClickHandle { closure, element }
    }
}

impl Drop for ClickHandle {
    fn drop(&mut self) {
        self.element
            .remove_event_listener_with_callback("click", &self.closure.as_ref().unchecked_ref())
            .unwrap();
    }
}
