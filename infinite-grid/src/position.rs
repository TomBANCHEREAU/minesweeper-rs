#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position<T> {
    pub x: T,
    pub y: T,
}

pub(crate) struct ChunkSubPosition(Position<u8>);
pub(crate) struct ChunkPosition(Position<u8>);
pub struct GlobalPosition(Position<i16>);
