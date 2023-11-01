use std::marker::PhantomData;

use gloo::{events::EventListener, utils::window};
use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::JsCast;
use web_sys::{Event, MessageEvent, WebSocket};
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
        let listener = EventListener::new(&socket, "message", move |event: &Event| {
            let event = event.dyn_ref::<MessageEvent>().unwrap();
            callback.emit(
                serde_json::from_str::<Out>(event.data().as_string().unwrap().as_str()).unwrap(),
            )
        });

        Self {
            socket,
            _listener: listener,
            _in: Default::default(),
            _out: Default::default(),
        }
    }
    pub fn send(&mut self, payload: In) {
        self.socket
            .send_with_str(serde_json::to_string(&payload).unwrap().as_str())
            .unwrap();
    }
}
