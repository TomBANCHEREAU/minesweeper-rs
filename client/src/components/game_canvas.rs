use core::{
    game::{GameAction, GameEvent},
    grid::{vec_grid::VecGrid, Grid},
    messages::{GenericClientMessage, GenericServerMessage},
    tile::TileState,
};
use std::rc::Rc;

use gloo::{
    console::log,
    render::{request_animation_frame, AnimationFrame},
};
use model::Lobby;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};
use yew::{html, Component, Html, NodeRef, Properties};

use crate::{
    contexts::user::User,
    image::{draw_sprite, Sprite},
    utils::{
        mouse_event_handler::{handle_mouse_event, MouseButton, MouseEvent},
        socket::Socket,
    },
};

pub trait CanvasRenderer: PartialEq {
    type RenderContext;
    fn render(canvas: HtmlCanvasElement, context: Self::RenderContext);
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub lobby: Rc<Lobby>,
    pub user: User,
}

pub struct GameCanvas {
    canvas_node_ref: NodeRef,
    canvas_element: Option<HtmlCanvasElement>,
    canvas_context: Option<CanvasRenderingContext2d>,

    image_node_ref: NodeRef,
    image_element: Option<HtmlImageElement>,

    animation_frame_handle: Option<AnimationFrame>,

    socket: Socket<GenericClientMessage, GenericServerMessage>,
    grid: VecGrid<TileState>,
}

#[derive(Debug)]
pub enum Message {
    RenderRequest,
    Render,
    Click(MouseEvent),
    SocketMessage(GenericServerMessage),
    SendMessage(GameAction),
}

impl Component for GameCanvas {
    type Message = Message;

    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let socket = Socket::<GenericClientMessage, GenericServerMessage>::new(
            format!("/api/lobby/{}/ws", ctx.props().lobby.id).as_str(),
            ctx.link().callback(Message::SocketMessage),
        );

        Self {
            canvas_node_ref: NodeRef::default(),
            canvas_element: None,
            canvas_context: None,
            image_node_ref: NodeRef::default(),
            image_element: None,
            animation_frame_handle: None,
            socket,
            grid: VecGrid::empty(),
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        log!(format!("{msg:?}"));
        match msg {
            Message::RenderRequest => {
                self.animation_frame_handle.get_or_insert_with(|| {
                    let link = ctx.link().clone();
                    request_animation_frame(move |_| link.send_message(Message::Render))
                });
            }
            Message::Render => {
                self.animation_frame_handle = None;
                if let Some((context, image_element)) = self
                    .canvas_context
                    .as_ref()
                    .zip(self.image_element.as_ref())
                {
                    for x in 0..self.grid.grid.get(0).map_or(0, |vec| vec.len()) {
                        for y in 0..self.grid.grid.len() {
                            let sprite = Sprite::from(self.grid.get(x as i32, y as i32).unwrap());
                            draw_sprite(
                                image_element,
                                context,
                                sprite,
                                f64::from(x as i32) * 16.,
                                f64::from(y as i32) * 16.,
                                16.,
                                16.,
                            );
                        }
                    }
                }
            }
            Message::Click(event) => {
                handle_mouse_event(event, &self.grid, ctx.link().callback(Message::SendMessage));
            }
            Message::SocketMessage(event) => {
                match event {
                    GenericServerMessage::GameEvent(game_event) => match game_event {
                        GameEvent::TileStateUpdate { x, y, state } => {
                            *self.grid.get_mut(x, y).unwrap() = state;
                            ctx.link().send_message(Message::RenderRequest)
                        }
                        GameEvent::GameOver {} => (),
                        GameEvent::GameStart { grid } => {
                            self.grid = grid;
                            if let Some(canvas_element) = &self.canvas_element {
                                canvas_element.set_height(self.grid.grid.len() as u32 * 16);
                                canvas_element.set_width(self.grid.grid[0].len() as u32 * 16);
                            }
                            ctx.link().send_message(Message::RenderRequest)
                        }
                    },
                }
                ctx.link().send_message(Message::RenderRequest);
            }
            Message::SendMessage(msg) => {
                self.socket.send(GenericClientMessage::GameAction(msg));
            }
        };
        false
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let onclick = ctx.link().batch_callback(|event: web_sys::MouseEvent| {
            event.prevent_default();
            let button = match event.buttons() {
                1 => MouseButton::Left,
                2 => MouseButton::Right,
                3 => MouseButton::Double,
                4 => MouseButton::Middle,
                _ => return None,
            };

            return Some(Message::Click(MouseEvent {
                x: event.offset_x().div_euclid(16),
                y: event.offset_y().div_euclid(16),
                button,
            }));
        });
        let onload = ctx.link().callback(|_| Message::RenderRequest);
        html! {
            <>
                <p>{&ctx.props().user.username}</p>
                <canvas
                    ref={self.canvas_node_ref.clone()}
                    onmousedown={onclick}
                    oncontextmenu={|event: web_sys::MouseEvent| event.prevent_default()}
                />
                <img
                    ref={self.image_node_ref.clone()}
                    onload={onload}
                    src="/images/sprites.png"
                    style="visibility: hidden"
                />
            </>
        }
    }

    fn rendered(&mut self, ctx: &yew::Context<Self>, _first_render: bool) {
        self.canvas_element = Some(self.canvas_node_ref.cast::<HtmlCanvasElement>().unwrap());
        self.canvas_context = Some(
            self.canvas_element
                .as_ref()
                .unwrap()
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into()
                .unwrap(),
        );
        self.image_element = Some(self.image_node_ref.cast::<HtmlImageElement>().unwrap());
        ctx.link().send_message(Message::RenderRequest)
    }

    fn changed(&mut self, _ctx: &yew::Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn prepare_state(&self) -> Option<String> {
        None
    }
}
