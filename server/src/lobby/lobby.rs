use std::sync::mpsc::{Receiver, Sender};

use minesweeper_core::{
    game::Game,
    grid::vec_grid::VecGrid,
    messages::{GenericClientMessage, GenericServerMessage},
};

pub struct LobbyMessage {
    pub author: String,
    pub content: LobbyMessageContent,
}
pub enum LobbyMessageContent {
    Listen {
        listenner: Sender<GenericServerMessage>,
    },
    GenericClientMessage(GenericClientMessage),
}

pub fn lobby(receiver: Receiver<LobbyMessage>, create_lobby_body: model::CreateLobbyBody) {
    let mut listenners: Vec<Sender<GenericServerMessage>> = vec![];
    let mut game = Game::new(VecGrid::new(
        create_lobby_body.grid_width,
        create_lobby_body.grid_height,
    ));
    for message in receiver.iter() {
        match message.content {
            LobbyMessageContent::Listen { listenner } => {
                listenner
                    .send(GenericServerMessage::GameEvent(game.get_start_event()))
                    .unwrap();
                listenners.push(listenner)
            }
            LobbyMessageContent::GenericClientMessage(GenericClientMessage::GameAction(action)) => {
                game.play(minesweeper_core::game::GameInput {
                    username: message.author,
                    action,
                });
                for event in game.buffered_events() {
                    for listenner in &listenners {
                        listenner
                            .send(GenericServerMessage::GameEvent(event.clone()))
                            .unwrap();
                    }
                }
            }
        }
    }
}
