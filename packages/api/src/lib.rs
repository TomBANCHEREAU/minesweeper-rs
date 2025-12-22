//! This crate contains all shared fullstack server functions.
use dioxus::{fullstack::FullstackContext, prelude::*};

mod minesweeper_context;
/// Echo the user input on the server.
#[post("/api/echo")]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    // let context: MinesweeperContext = FullstackContext::extract::<MinesweeperContext>()
    //     .await
    //     .unwrap();
    Ok(input)
}

pub mod contexts {
    pub use crate::minesweeper_context::MinesweeperContext;
}
