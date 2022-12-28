#[derive(Debug, Clone, Copy)]
pub struct Tile {
    content: TileContent,
    state: TileState,
}

#[derive(Debug, Clone, Copy)]
pub enum TileContent {
    Empty,
    Number(u8),
    Bomb,
}

#[derive(Debug, Clone, Copy)]
pub enum TileState {
    Flagged,
    Discovered,
    Untouched,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            content: TileContent::Empty,
            state: TileState::Untouched,
        }
    }
}
