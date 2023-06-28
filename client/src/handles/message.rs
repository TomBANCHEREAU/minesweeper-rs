use core::messages::GenericServerMessage;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{MessageEvent, WebSocket};

pub struct MessageHandle {
    _closure: Closure<dyn Fn(MessageEvent)>,
}

impl MessageHandle {
    pub fn new<F: Fn(GenericServerMessage) + 'static>(socket: &WebSocket, function: F) -> Self {
        let closure = Closure::new(move |event: MessageEvent| {
            function(serde_json::from_str(&event.data().as_string().unwrap().as_str()).unwrap());
        });
        socket.set_onmessage(Some(closure.as_ref().unchecked_ref()));
        MessageHandle { _closure: closure }
    }
}

// impl Drop for MessageHandle {
//     fn drop(&mut self) {
//         get_window()
//             .remove_event_listener_with_callback("resize", &self.closure.as_ref().unchecked_ref())
//             .unwrap();
//     }
// }
