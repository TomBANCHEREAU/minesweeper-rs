use crate::position::ChunkSubPosition;

pub const CHUNK_SIZE: usize = 64;
pub struct Chunk<T> {
    items: [[T; CHUNK_SIZE]; CHUNK_SIZE],
}

impl<T> Chunk<T> {
    pub fn get(&self, index: &ChunkSubPosition) -> Option<&T> {
        self.items
            .get(usize::from(index.x))
            .and_then(|i| i.get(usize::from(index.y)))
    }
    pub fn get_mut(&mut self, index: &ChunkSubPosition) -> Option<&mut T> {
        self.items
            .get_mut(usize::from(index.x))
            .and_then(|i| i.get_mut(usize::from(index.y)))
    }
}
