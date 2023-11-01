use std::ops::Deref;

use gloo_net::http::Request;
use model::{CreateLobbyBody, Lobby};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::html::onchange;
use yew::{function_component, html, use_state, Html};
use yew_router::prelude::*;

use crate::app::Route;

#[function_component(PageLobbyNew)]
pub fn page_lobby_new() -> Html {
    let navigator = use_navigator().unwrap();

    let grid_width = use_state(|| 15u8);
    let grid_height = use_state(|| 15u8);
    let sent = use_state(|| false);
    let on_grid_width_change = {
        let grid_width = grid_width.clone();
        move |event: onchange::Event| {
            grid_width.set(
                event
                    .target()
                    .and_then(|element| {
                        element
                            .dyn_into::<HtmlInputElement>()
                            .ok()?
                            .value()
                            .parse::<u8>()
                            .ok()
                    })
                    .unwrap_or(*grid_width.deref()),
            )
        }
    };
    let on_grid_height_change = {
        let grid_height = grid_height.clone();
        move |event: onchange::Event| {
            grid_height.set(
                event
                    .target()
                    .and_then(|element| {
                        element
                            .dyn_into::<HtmlInputElement>()
                            .ok()?
                            .value()
                            .parse::<u8>()
                            .ok()
                    })
                    .unwrap_or(*grid_height.deref()),
            )
        }
    };
    let on_form_submit = {
        let grid_width = grid_width.clone();
        let grid_height = grid_height.clone();
        let sent = sent.clone();
        move |_event| {
            sent.set(true);
            let grid_width = grid_width.clone();
            let grid_height = grid_height.clone();
            let navigator = navigator.clone();
            let sent = sent.clone();
            spawn_local(async move {
                let request = Request::post("/api/lobby")
                    .json(&CreateLobbyBody {
                        grid_height: *grid_height,
                        grid_width: *grid_width,
                    })
                    .unwrap();
                let resp = request.send().await;
                match resp {
                    Ok(response) => {
                        if let Ok(Lobby { id }) = response.json().await {
                            navigator.push(&Route::Lobby { id })
                        } else {
                            sent.set(false)
                        }
                    }
                    Err(_err) => sent.set(false),
                };
            });
        }
    };
    if !sent.deref() {
        html! {
            <form onsubmit={on_form_submit}>
                <input
                    type="number"
                    name="grid_width"
                    min={u8::MIN.to_string()}
                    max={u8::MAX.to_string()}
                    value={format!("{}", grid_width.deref())}
                    onchange={on_grid_width_change}
                />
                <input
                    type="number"
                    name="grid_height"
                    min={u8::MIN.to_string()}
                    max={u8::MAX.to_string()}
                    value={format!("{}", grid_height.deref())}
                    onchange={on_grid_height_change}
                />
                <button type="submit">
                    {"Create new lobby"}
                </button>
            </form>
        }
    } else {
        html! {}
    }
}
