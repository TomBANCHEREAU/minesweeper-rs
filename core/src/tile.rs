use serde::{Deserialize, Serialize};

/*
 * Keep it simple for now
 * - TileContent save the real content of the tile
 * - TileState save the client side state
 * - Tile save the client side state and the real content
 */
// #[cfg(feature = "server")]
#[derive(Debug, Clone, Copy, Default)]
pub struct Tile {
    pub content: TileContent,
    pub state: TileState,
}

// #[cfg(feature = "server")]
impl Tile {
    pub fn as_client(&self) -> TileState {
        self.state
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum TileContent {
    #[default]
    Empty,
    Number(u8),
    Bomb,
}
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum TileState {
    #[default]
    Untouched,
    Flagged,
    Discovered(TileContent),
}
