use std::{fmt::Debug, sync::Arc};

use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{HtmlElement, MouseEvent};

use crate::viewport::MutexedViewport;

pub struct ClickHandle {
    closure: Closure<dyn Fn(MouseEvent)>,
    element: HtmlElement,
}

impl ClickHandle {
    pub fn new<E: JsCast + Debug>(viewport: &MutexedViewport, element: E) -> Self {
        let viewport = Arc::clone(viewport);
        let closure = Closure::new(move |event: MouseEvent| {
            event.prevent_default();
            viewport.lock().unwrap().on_click(event);
        });
        let element = element.dyn_into::<HtmlElement>().unwrap();
        element.set_onclick(Some(closure.as_ref().unchecked_ref()));
        element.set_oncontextmenu(Some(closure.as_ref().unchecked_ref()));
        ClickHandle { closure, element }
    }
}

impl Drop for ClickHandle {
    fn drop(&mut self) {
        self.element
            .remove_event_listener_with_callback("click", &self.closure.as_ref().unchecked_ref())
            .unwrap();
        self.element
            .remove_event_listener_with_callback(
                "contextmenu",
                &self.closure.as_ref().unchecked_ref(),
            )
            .unwrap();
    }
}
