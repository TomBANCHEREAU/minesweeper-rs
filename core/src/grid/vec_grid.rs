use std::clone;

#[cfg(feature = "server")]
use rand::{rngs::ThreadRng, seq::SliceRandom};
use serde::{Deserialize, Serialize};

use crate::tile::{Tile, TileContent, TileState};

/*
 * Keep it simple for now
 * Simply use vec of vec instead of an array with a constant size
 *
 */
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VecGrid<T> {
    grid: Vec<Vec<T>>,
}

impl<T> VecGrid<T> {
    pub fn get(&self, x: i32, y: i32) -> Option<&T> {
        self.grid
            .get(usize::try_from(y).ok()?)
            .and_then(|vec| vec.get(usize::try_from(x).ok()?))
    }
    pub fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut T> {
        self.grid
            .get_mut(usize::try_from(y).ok()?)
            .and_then(|vec| vec.get_mut(usize::try_from(x).ok()?))
    }
}

impl Default for VecGrid<TileState> {
    fn default() -> Self {
        let mut grid: Vec<Vec<TileState>> = Default::default();
        for _y in 0..20 {
            let mut col = vec![];
            for _x in 0..20 {
                col.push(TileState::Untouched);
            }
            grid.push(col);
        }
        Self { grid }
    }
}
#[cfg(feature = "server")]
impl Default for VecGrid<Tile> {
    fn default() -> Self {
        let mut grid: Vec<Vec<Tile>> = Default::default();
        for _y in 0..20 {
            let mut col = vec![];
            for _x in 0..20 {
                col.push(Tile {
                    content: TileContent::Empty,
                    state: TileState::Untouched,
                });
            }
            grid.push(col);
        }
        let mut grid = Self { grid };
        for (y, x) in (0..20)
            .flat_map(|y| (0..20).map(move |x| (y, x)))
            .collect::<Vec<_>>()
            .partial_shuffle(&mut ThreadRng::default(), 80)
            .0
        {
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if let Some(Tile { content, state: _ }) = grid.get_mut(*x + dx, *y + dy) {
                        if dx == 0 && dy == 0 {
                            *content = TileContent::Bomb;
                        } else {
                            match content {
                                TileContent::Empty => *content = TileContent::Number(1),
                                TileContent::Number(n) => *n += 1,
                                TileContent::Bomb => (),
                            }
                        }
                    }
                }
            }
        }
        grid
    }
}

impl From<&VecGrid<Tile>> for VecGrid<TileState> {
    fn from(value: &VecGrid<Tile>) -> Self {
        Self {
            grid: value
                .grid
                .iter()
                .map(|col| col.iter().map(|tile| tile.state).collect())
                .collect(),
        }
    }
}
// }

// pub type ClientVecGrid = VecGrid<ClientTile>;

// impl<T> Grid<T> for VecGrid<T> {
//     fn get_width(&self) -> usize {
//         self.grid.get(0).map(|vec| vec.len()).unwrap_or(0)
//     }
//     fn get_height(&self) -> usize {
//         self.grid.len()
//     }
// fn get(&self, x: i32, y: i32) -> Option<&T> {
//     if x < 0 || y < 0 {
//         return None;
//     }
//     self.grid
//         .get(y as usize)
//         .and_then(|vec| vec.get(x as usize))
// }
// fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut T> {
//     if x < 0 || y < 0 {
//         return None;
//     }
//     self.grid
//         .get_mut(y as usize)
//         .and_then(|vec| vec.get_mut(x as usize))
// }
// }
