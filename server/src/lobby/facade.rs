use std::{
    sync::mpsc::{channel, Sender},
    thread::spawn,
};

use crate::middleware::auth::User;

use super::{
    handle::LobbyHandle,
    lobby::{lobby, LobbyMessage, LobbyMessageContent},
};

/**
 * Lobby Facade
 * impl Default: create new lobby and start thread
 */
pub struct LobbyFacade {
    // server_message_receiver: Receiver<()>,
    lobby_message_sender: Sender<LobbyMessage>,
}

impl LobbyFacade {
    pub fn new(create_lobby_body: model::CreateLobbyBody) -> Self {
        let (lobby_message_sender, lobby_message_receiver) = channel();
        spawn(move || lobby(lobby_message_receiver, create_lobby_body));
        Self {
            // server_message_receiver,
            lobby_message_sender,
        }
    }
}
impl LobbyFacade {
    pub fn create_handle(&self, user: User) -> LobbyHandle {
        let (sender, receiver) = channel();
        self.lobby_message_sender
            .send(LobbyMessage {
                author: user.username.clone(),
                content: LobbyMessageContent::Listen { listenner: sender },
            })
            .unwrap();
        LobbyHandle {
            user,
            lobby_message_sender: self.lobby_message_sender.clone(),
            lobby_event_receiver: Some(receiver),
        }
    }
}
