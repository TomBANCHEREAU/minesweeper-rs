// pub mod chunk;
pub mod vec_grid;

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

pub trait Grid {
    type Tile;
    type Index: GridIndex;
    fn get(
        &self,
        x: impl TryInto<Self::Index>,
        y: impl TryInto<Self::Index>,
    ) -> Option<&Self::Tile>;
    fn get_mut(
        &mut self,
        x: impl TryInto<Self::Index>,
        y: impl TryInto<Self::Index>,
    ) -> Option<&mut Self::Tile>;
    fn iter_around<'a>(
        &'a self,
        x: impl TryInto<Self::Index>,
        y: impl TryInto<Self::Index>,
    ) -> NeighborsIter<'a, Self>
    where
        Self: Sized,
    {
        NeighborsIter {
            grid: self,
            x: x.try_into().ok(),
            y: y.try_into().ok(),
            index: 0,
        }
    }
    // fn iter_position_around(
    //     &self,
    //     x: impl TryInto<Self::Index>,
    //     y: impl TryInto<Self::Index>,
    // ) -> IntoIter<&(i8, i8)> {
    //     let x: Option<Self::Index> = x.try_into().ok();
    //     let y: Option<Self::Index> = y.try_into().ok();
    //     let out = NEIGHBORS
    //         .iter()
    //         .filter(|(dx, dy)| {
    //             let Some(x) = x.and_then(|x|x.try_add(*dx)) else { return false };
    //             let Some(y) = y.and_then(|y|y.try_add(*dy)) else { return false };
    //             return true;
    //         })
    //         .collect::<Vec<_>>();
    //     out.into_iter()
    // }
}

pub trait GridIndex: Copy {
    fn try_add(&self, value: i8) -> Option<Self>
    where
        Self: Sized;
}

impl GridIndex for u8 {
    fn try_add(&self, value: i8) -> Option<Self>
    where
        Self: Sized,
    {
        self.checked_add_signed(value)
    }
}

pub struct NeighborsIter<'a, G: Grid> {
    grid: &'a G,
    x: Option<G::Index>,
    y: Option<G::Index>,
    index: usize,
}

impl<'a, G: Grid> Iterator for NeighborsIter<'a, G> {
    type Item = ((G::Index, G::Index), &'a G::Tile);
    fn next(&mut self) -> Option<Self::Item> {
        while self.index != 8 {
            let (dx, dy) = NEIGHBORS[self.index];
            self.index += 1;
            let Some(x) = self.x.as_ref()?.try_add(dx) else {continue;};
            let Some(y) = self.y.as_ref()?.try_add(dy) else {continue;};
            if let Some(tile) = self.grid.get(x, y) {
                return Some(((x, y), tile));
            }
        }
        None
    }
}
