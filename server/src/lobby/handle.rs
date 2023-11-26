use std::sync::mpsc::{Receiver, Sender};

use minesweeper_core::messages::GenericServerMessage;

use crate::middleware::auth::User;

use super::lobby::{LobbyMessage, LobbyMessageContent};

/**
 * Lobby Handle
 * impl Drop: send a message to the lobby to self unregister
 * run by async thread
 * created by facade
 */
pub struct LobbyHandle {
    pub user: User,
    pub lobby_message_sender: Sender<LobbyMessage>,
    pub lobby_event_receiver: Option<Receiver<GenericServerMessage>>,
}

impl LobbyHandle {
    pub fn lobby_message(&self, content: LobbyMessageContent) -> LobbyMessage {
        LobbyMessage {
            author: self.user.username.clone(),
            content,
        }
    }
}
