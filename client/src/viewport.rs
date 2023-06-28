use core::{
    game::{GameAction, GameEvent},
    grid::vec_grid::VecGrid,
    messages::{GenericClientMessage, GenericServerMessage},
    tile::TileState,
};
use std::{
    default,
    sync::{Arc, Mutex},
};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    console::log_1, window, CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent, WebSocket,
};

use crate::{
    handles::{
        animation::AnimationHandle, click::ClickHandle, message::MessageHandle,
        resize::ResizeHandle,
    },
    image::{ImageManager, Sprite},
    utils::get_window,
};

pub struct ViewportOptions {
    pub canvas_id: String,
    pub lobby: String,
}
// pub enum LobbyOption {
//     Create(),
//     Join(String),
// }
pub type MutexedViewport = Arc<Mutex<Viewport>>;
pub struct Viewport {
    grid: VecGrid<TileState>,
    redraw: bool,
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    image_manager: ImageManager,
    resize_handle: Option<ResizeHandle>,
    animation_handle: Option<AnimationHandle>,
    click_handle: Option<ClickHandle>,
    message_handle: Option<MessageHandle>,
    socket: WebSocket,
}

impl Viewport {
    pub fn new(options: ViewportOptions) -> Result<(), String> {
        let canvas = get_canvas(&options.canvas_id)?;
        let context = get_context(&canvas)?;
        let url = web_sys::Url::new(window().unwrap().location().href().unwrap().as_str()).unwrap();
        url.set_protocol(url.protocol().replace("http", "ws").as_str());
        url.set_pathname(format!("/lobby/{}", options.lobby).as_str());
        let socket = WebSocket::new((url.href()).as_str()).map_err(|op| {
            op.as_string()
                .unwrap_or("Could not create socket".to_string())
        })?;
        let mutexed_viewport = Arc::new(Mutex::new(Viewport {
            canvas,
            context,
            resize_handle: None,
            image_manager: ImageManager::new(),
            animation_handle: None,
            click_handle: None,
            message_handle: None,
            socket,
            grid: Default::default(),
            redraw: true,
        }));
        let mut viewport = mutexed_viewport.lock().unwrap();

        let cloned_viewport = mutexed_viewport.clone();
        viewport.on_resize();
        viewport.resize_handle = Some(ResizeHandle::new(move || {
            cloned_viewport.lock().unwrap().on_resize();
        }));

        let cloned_viewport = mutexed_viewport.clone();
        viewport.message_handle = Some(MessageHandle::new(&viewport.socket, move |event| {
            log_1(&JsValue::from_str("on_message2"));
            cloned_viewport.lock().unwrap().on_message(event);
        }));

        let cloned_viewport = mutexed_viewport.clone();

        viewport.animation_handle = Some(AnimationHandle::new(&cloned_viewport));
        viewport.click_handle = Some(ClickHandle::new(
            &cloned_viewport,
            get_canvas(&options.canvas_id)?,
        ));
        Ok(())
    }
    fn on_message(&mut self, event: GenericServerMessage) {
        log_1(&JsValue::from_str("on_message"));
        match event {
            GenericServerMessage::GameEvent(game_event) => match game_event {
                GameEvent::TileStateUpdate { x, y, state } => {
                    *self.grid.get_mut(x, y).unwrap() = state;
                }
                GameEvent::GameOver {} => todo!(),
                GameEvent::GameStart { grid } => self.grid = grid,
            },
        }
        self.redraw = true;
    }
    pub fn on_click(&mut self, event: MouseEvent) {
        let x = event.x() / 16;
        let y = event.y() / 16;
        if let Some(tile) = self.grid.get(x, y) {
            match event.button() {
                0 => self
                    .socket
                    .send_with_str(
                        serde_json::to_string(&GenericClientMessage::GameAction(
                            GameAction::Discover { x, y },
                        ))
                        .unwrap()
                        .as_str(),
                    )
                    .unwrap(),
                2 => match tile {
                    TileState::Untouched => self
                        .socket
                        .send_with_str(
                            serde_json::to_string(&GenericClientMessage::GameAction(
                                GameAction::PlaceFlag { x, y },
                            ))
                            .unwrap()
                            .as_str(),
                        )
                        .unwrap(),
                    TileState::Flagged => self
                        .socket
                        .send_with_str(
                            serde_json::to_string(&GenericClientMessage::GameAction(
                                GameAction::RemoveFlag { x, y },
                            ))
                            .unwrap()
                            .as_str(),
                        )
                        .unwrap(),
                    TileState::Discovered(_) => (),
                },
                _ => (),
            };
        }
        log_1(&JsValue::from_str("on_click"))
    }
    pub fn on_resize(&mut self) {
        self.canvas.set_width(self.canvas.client_width() as u32);
        self.canvas.set_height(self.canvas.client_height() as u32);
        self.redraw = true;
    }
    pub fn on_animation_frame(&mut self) {
        if !self.redraw {
            return;
        }
        for x in 0..20 {
            for y in 0..20 {
                let sprite = Sprite::from(self.grid.get(x, y).unwrap());
                self.image_manager.draw_sprite(
                    &self.context,
                    sprite,
                    f64::from(x) * 16.,
                    f64::from(y) * 16.,
                    16.,
                    16.,
                );
            }
        }
        self.redraw = false;
    }
}

fn get_canvas(canvas_id: &String) -> Result<HtmlCanvasElement, String> {
    let window = get_window();
    let document = window.document().ok_or("Could not get Document")?;
    document
        .get_element_by_id(&canvas_id)
        .ok_or(format!("Could not get element with id: \"{}\"", canvas_id))?
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .ok()
        .ok_or(format!(
            "Element with id: \"{}\" is not a canvas",
            canvas_id
        ))
}

fn get_context(canvas: &HtmlCanvasElement) -> Result<CanvasRenderingContext2d, String> {
    canvas
        .get_context("2d")
        .ok()
        .ok_or("Could not get the 2d context")?
        .ok_or("Could not get the 2d context")?
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .ok()
        .ok_or("Could not get the 2d context".to_string())
}
