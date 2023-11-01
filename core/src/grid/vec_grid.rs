#[cfg(feature = "server")]
use rand::{rngs::ThreadRng, seq::SliceRandom};
use serde::{Deserialize, Serialize};

use crate::tile::{Tile, TileContent, TileState};

use super::Grid;

/*
 * Keep it simple for now
 * Simply use vec of vec instead of an array with a constant size
 *
 */
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VecGrid<T> {
    pub grid: Vec<Vec<T>>,
}
impl<T> VecGrid<T> {
    pub fn empty() -> Self {
        Self {
            grid: Vec::with_capacity(0),
        }
    }
}

impl<T> Grid for VecGrid<T> {
    fn get(&self, x: impl TryInto<u8>, y: impl TryInto<u8>) -> Option<&T> {
        let x: u8 = x.try_into().ok()?;
        let y: u8 = y.try_into().ok()?;
        self.grid
            .get(usize::try_from(y).ok()?)
            .and_then(|vec| vec.get(usize::try_from(x).ok()?))
    }
    fn get_mut(&mut self, x: impl TryInto<u8>, y: impl TryInto<u8>) -> Option<&mut T> {
        let x: u8 = x.try_into().ok()?;
        let y: u8 = y.try_into().ok()?;
        self.grid
            .get_mut(usize::try_from(y).ok()?)
            .and_then(|vec| vec.get_mut(usize::try_from(x).ok()?))
    }

    type Index = u8;
    type Tile = T;
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
impl VecGrid<Tile> {
    pub fn new(width: u8, height: u8) -> Self {
        let mut grid: Vec<Vec<Tile>> = Default::default();
        for _y in 0..height {
            let mut col = vec![];
            for _x in 0..width {
                col.push(Tile {
                    content: TileContent::Empty,
                    state: TileState::Untouched,
                });
            }
            grid.push(col);
        }
        let mut grid = Self { grid };
        for (y, x) in (0..height)
            .flat_map(|y| (0..width).map(move |x| (y, x)))
            .collect::<Vec<_>>()
            .partial_shuffle(
                &mut ThreadRng::default(),
                usize::from(u16::from(width) * u16::from(height) / 5u16),
            )
            .0
        {
            for dy in -1..=1i16 {
                for dx in -1..=1i16 {
                    let Ok(x) = u8::try_from(dx + i16::from(*x)) else {continue;};
                    let Ok(y) = u8::try_from(dy + i16::from(*y)) else {continue;};
                    if let Some(Tile { content, state: _ }) = grid.get_mut(x, y) {
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
#[cfg(feature = "server")]
impl Default for VecGrid<Tile> {
    fn default() -> Self {
        Self::new(20, 20)
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
