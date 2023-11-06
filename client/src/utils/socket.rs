use std::marker::PhantomData;

use gloo::{events::EventListener, utils::window};
use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::JsCast;
use web_sys::{BinaryType, Event, MessageEvent, WebSocket};
use yew::Callback;

pub struct Socket<In: Serialize, Out: DeserializeOwned> {
    socket: WebSocket,
    _listener: EventListener,
    _in: PhantomData<In>,
    _out: PhantomData<Out>,
}

impl<In: Serialize + 'static, Out: DeserializeOwned + 'static> Socket<In, Out> {
    pub fn new(pathname: &str, callback: Callback<Out>) -> Self {
        let url = web_sys::Url::new(window().location().href().unwrap().as_str()).unwrap();
        url.set_protocol(url.protocol().replace("http", "ws").as_str());
        url.set_pathname(pathname);
        let socket = WebSocket::new(url.href().as_str()).unwrap();
        socket.set_binary_type(BinaryType::Arraybuffer);
        let listener = EventListener::new(&socket, "message", move |event: &Event| {
            let event = event.dyn_ref::<MessageEvent>().unwrap();
            #[cfg(debug_assertions)]
            callback.emit(
                serde_json::from_str::<Out>(event.data().as_string().unwrap().as_str()).unwrap(),
            );
            #[cfg(not(debug_assertions))]
            callback.emit(
                bitcode::deserialize::<Out>(
                    js_sys::Uint8Array::new(&event.data()).to_vec().as_ref(),
                )
                .unwrap(),
            );
        });

        Self {
            socket,
            _listener: listener,
            _in: Default::default(),
            _out: Default::default(),
        }
    }
    pub fn send(&mut self, payload: In) {
        #[cfg(debug_assertions)]
        self.socket
            .send_with_str(serde_json::to_string(&payload).unwrap().as_str())
            .unwrap();
        #[cfg(not(debug_assertions))]
        self.socket
            .send_with_u8_array(bitcode::serialize(&payload).unwrap().as_ref())
            .unwrap();
    }
}
