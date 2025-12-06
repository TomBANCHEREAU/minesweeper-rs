use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

pub mod chunk;
pub mod position;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        let i = i16::MAX;
    }
}
// Un petit morceau tout mignon

// impl From<GlobalPosition> for (ChunkPosition, ChunkSubPosition) {
//     fn from(value: GlobalPosition) -> Self {
//         // let chunk_x = value.x.div_euclid(rhs);
//         // (ChunkPosition { x, y }, ChunkSubPosition { x, y })
//         todo!()
//     }
// }

type TEST5 = HashMap<u8, ()>;
