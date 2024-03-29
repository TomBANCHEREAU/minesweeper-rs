use std::{ops::Deref, rc::Rc};

use gloo_net::http::Request;
use model::Lobby;
use yew::{
    function_component, html, use_context, use_effect, use_effect_with, use_state, Html, Properties,
};

use crate::{components::game_canvas::GameCanvas, contexts::user::User};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}
enum PageState {
    Loading,
    Ready { lobby: Rc<Lobby> },
}
#[function_component(PageLobbyId)]
pub fn page_lobby_id(props: &Props) -> Html {
    let id = props.id.clone();
    let page_state = use_state(|| PageState::Loading);
    {
        let page_state = page_state.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let lobby = Request::get(format!("/api/lobby/{}", id).as_str())
                    .send()
                    .await
                    .unwrap()
                    .json::<Lobby>()
                    .await
                    .unwrap();
                page_state.set(PageState::Ready {
                    lobby: Rc::new(lobby),
                })
            })
        });
    }
    let user = use_context::<User>().unwrap();
    match page_state.deref() {
        PageState::Loading => html!("Loading..."),
        PageState::Ready { lobby } => html! {
           <GameCanvas lobby={lobby} user={user}/>
        },
    }
}
