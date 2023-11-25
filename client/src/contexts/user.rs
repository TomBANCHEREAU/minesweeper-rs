use std::ops::Deref;

use jwt::{Claims, Header, Token, VerifyWithKey};
use serde::Deserialize;
use web_sys::HtmlDocument;
use yew::prelude::*;

fn document() -> HtmlDocument {
    use wasm_bindgen::JsCast;

    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .dyn_into::<HtmlDocument>()
        .unwrap()
}

fn cookie_string() -> String {
    document().cookie().unwrap()
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct User {
    pub username: String,
}
#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html, // the field name `children` is important!
}
#[function_component]
pub fn UserContext(props: &Props) -> Html {
    let token_cookie = wasm_cookies::cookies::get(&cookie_string(), "token")
        .unwrap()
        .unwrap();
    let token: Token<Header, User, _> = Token::parse_unverified(token_cookie.as_str()).unwrap();
    let user: User = token.claims().clone();

    let ctx = use_state(move || user);

    html! {
        <ContextProvider<User> context={(*ctx).clone()}>
            {props.children.clone()}
        </ContextProvider<User>>
    }
}
