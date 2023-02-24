// pub enum ClientTile {
//     Untouched,
//     Flagged,
//     Discovered(TileContent),
// }

struct ClientTile {
    flagged: bool,
    covered: bool,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum TileState {
    #[default]
    Untouched,
    Flagged,
    Discovered,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum TileContent {
    #[default]
    Empty,
    Number(u8),
    Bomb,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ServerTile {
    pub(crate) content: TileContent,
    state: TileState,
}

//

// impl From<&ServerTile> for ClientTile {
//     fn from(value: &ServerTile) -> ClientTile {
//         ClientTile {
//             content: match value.state {
//                 TileState::Discovered => Some(value.content),
//                 _ => None,
//             },
//             state: value.state,
//         }
//     }
// }
