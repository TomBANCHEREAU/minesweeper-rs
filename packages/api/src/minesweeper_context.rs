use std::{convert::Infallible, future::Future};

use dioxus::fullstack::extract::FromRequestParts;

#[derive(Debug, Clone)]
pub struct MinesweeperContext {
    pub a: String,
}

impl Default for MinesweeperContext {
    fn default() -> Self {
        Self {
            a: "Wow".to_string(),
        }
    }
}

impl FromRequestParts<MinesweeperContext> for MinesweeperContext {
    type Rejection = Infallible;

    fn from_request_parts(
        _parts: &mut dioxus::fullstack::http::request::Parts,
        state: &MinesweeperContext,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        let state = state.clone();
        async { Ok(state) }
    }
}
