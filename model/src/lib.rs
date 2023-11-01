use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Copy, Debug)]
pub struct CreateLobbyBody {
    pub grid_width: u8,
    pub grid_height: u8,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Lobby {
    pub id: String,
}

// pub struct GameConfig {}
