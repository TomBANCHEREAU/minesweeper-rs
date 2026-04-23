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
    width: u8,
    grid: Vec<T>,
}
impl<T> VecGrid<T> {
    pub fn empty() -> Self {
        Self {
            grid: Vec::with_capacity(0),
            width: 0,
        }
    }
}

pub const NEIGHBORS: [(i8, i8); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl<T> Grid for VecGrid<T> {
    type Index = (u8, u8);
    type Item = T;

    fn get(&self, (x, y): Self::Index) -> Option<&T> {
        if x >= self.width {
            return None;
        };
        let index = usize::from(y) * usize::from(self.width) + usize::from(x);
        self.grid.get(usize::from(index))
    }
    fn get_mut(&mut self, (x, y): Self::Index) -> Option<&mut T> {
        if x >= self.width {
            return None;
        };
        let index = usize::from(y) * usize::from(self.width) + usize::from(x);
        self.grid.get_mut(usize::from(index))
    }

    fn iter(&self) -> impl Iterator<Item = (Self::Index, &Self::Item)> {
        self.grid.iter().enumerate().map(|(index, item)| {
            (
                (
                    (index % self.width as usize) as u8,
                    (index / self.width as usize) as u8,
                ),
                item,
            )
        })
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = (Self::Index, &mut Self::Item)> {
        self.grid.iter_mut().enumerate().map(|(index, item)| {
            (
                (
                    (index % self.width as usize) as u8,
                    (index / self.width as usize) as u8,
                ),
                item,
            )
        })
    }

    fn iter_around(&self, (x, y): Self::Index) -> impl Iterator<Item = (Self::Index, &Self::Item)> {
        return self
            .iter_around_indexes((x, y))
            .filter_map(move |(dx, dy)| {
                return self.get((dx, dy)).map(|item| ((dx, dy), item));
            });
    }

    fn iter_around_mut(
        &mut self,
        (x, y): Self::Index,
    ) -> impl Iterator<Item = (Self::Index, &mut Self::Item)> {
        let mut indexes = NEIGHBORS.map(|(dx, dy)| {
            let x = x.checked_add_signed(dx)?;
            let y = y.checked_add_signed(dy)?;
            if x >= self.width || y >= (self.grid.len() / self.width as usize) as u8 {
                return None;
            }
            return Some((x, y));
        });
        self.grid
            .get_disjoint_mut(indexes.map(|option| {
                let Some((x, y)) = option else {
                    return 0..0usize;
                };
                let index = x as usize + y as usize * self.width as usize;
                return index..(index + 1);
            }))
            .unwrap()
            .into_iter()
            .enumerate()
            .flat_map(move |(i, r)| {
                let index = indexes[i].take();
                r.iter_mut().map(move |item| (index.unwrap(), item))
            })
    }
    fn iter_around_indexes(
        &self,
        (x, y): Self::Index,
    ) -> impl Iterator<Item = Self::Index> + 'static {
        let width = self.width;
        let height = (self.grid.len() / width as usize) as u8;
        NEIGHBORS.iter().filter_map(move |(dx, dy)| {
            let x = x.checked_add_signed(*dx)?;
            let y = y.checked_add_signed(*dy)?;
            if x >= width || y >= height {
                return None;
            }
            return Some((x, y));
        })
    }
}

impl Default for VecGrid<TileState> {
    fn default() -> Self {
        let mut grid: Vec<TileState> = Default::default();
        let width = 20u8;
        for _y in 0..width {
            for _x in 0..width {
                grid.push(TileState::Untouched);
            }
        }
        Self { grid, width }
    }
}

#[cfg(feature = "server")]
impl VecGrid<Tile> {
    pub fn new(width: u8, height: u8) -> Self {
        let mut grid: Vec<Tile> = Vec::with_capacity(height as usize * width as usize);
        for _y in 0..height {
            for _x in 0..width {
                grid.push(Tile {
                    content: TileContent::Empty,
                    state: TileState::Untouched,
                });
            }
        }
        Self { grid, width }
    }
    pub fn width(&self) -> u8 {
        return self.width;
    }
    pub fn height(&self) -> u8 {
        return (self.grid.len() / self.width as usize) as u8;
    }
    pub fn populate(&mut self, input_x: u8, input_y: u8) {
        for (y, x) in (0..self.height())
            .flat_map(|y| (0..self.width()).map(move |x| (y, x)))
            .filter(|(y, x)| x.abs_diff(input_x) > 2 || y.abs_diff(input_y) > 2)
            .collect::<Vec<_>>()
            .partial_shuffle(
                &mut ThreadRng::default(),
                (self.width() as usize) * (self.height() as usize) / 5usize,
            )
            .0
        {
            let Some(tile) = self.get_mut((*x, *y)) else {
                continue;
            };
            tile.content = TileContent::Bomb;
            for ((_x, _y), tile) in self.iter_around_mut((*x, *y)) {
                match &mut tile.content {
                    TileContent::Empty => tile.content = TileContent::Number(1),
                    TileContent::Number(n) => *n += 1,
                    TileContent::Bomb => (),
                }
            }
        }
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
            grid: value.iter().map(|(_, tile)| tile.state).collect(),
            width: value.width,
        }
    }
}
