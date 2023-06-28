use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::game::{GameAction, GameEvent, GameInput};

pub trait ClientMessage: Serialize + DeserializeOwned + Into<GenericClientMessage> {
    type ServerResponse: Serialize + DeserializeOwned;
}

pub trait ServerMessage {}

#[derive(Debug, Serialize, Deserialize)]
pub enum GenericClientMessage {
    GameAction(GameAction),
}
#[derive(Debug, Serialize, Deserialize)]
pub enum GenericServerMessage {
    GameEvent(GameEvent),
}
