use crate::pages::{lobby_id::PageLobbyId, lobby_new::PageLobbyNew};
use yew::{function_component, html, Html};
use yew_router::{BrowserRouter, Routable, Switch};

fn switch(routes: Route) -> Html {
    match routes {
        Route::Lobby { id } => html! { <PageLobbyId id={id}/> },
        Route::CreateLobby => html! { <PageLobbyNew/> },
    }
}

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/lobby/:id")]
    Lobby { id: String },
    #[not_found]
    #[at("/lobby/new")]
    CreateLobby,
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <main>
                <Switch<Route> render={switch} />
            </main>
        </BrowserRouter>
    }
}
