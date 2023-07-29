// pub mod chunk;
pub mod builder;
pub mod impl_vec_grid;

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
    // fn iter_mut_around<'a>(
    //     &'a mut self,
    //     x: impl TryInto<Self::Index>,
    //     y: impl TryInto<Self::Index>,
    // ) -> NeighborsIterMut<'a, Self>
    // where
    //     Self: Sized,
    // {
    //     NeighborsIterMut {
    //         grid: self,
    //         x: x.try_into().ok(),
    //         y: y.try_into().ok(),
    //         index: 0,
    //     }
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

// pub struct NeighborsIterMut<'a, G: Grid> {
//     grid: &'a mut G,
//     x: Option<G::Index>,
//     y: Option<G::Index>,
//     index: usize,
// }

// impl<'a, G: Grid> Iterator for NeighborsIterMut<'a, G> {
//     type Item = ((G::Index, G::Index), &'a mut G::Tile);
//     fn next<'b: 'a>(&'b mut self) -> Option<Self::Item> {
//         while self.index != 8 {
//             let (dx, dy) = NEIGHBORS[self.index];
//             self.index += 1;
//             let Some(x) = self.x.as_ref()?.try_add(dx) else {continue;};
//             let Some(y) = self.y.as_ref()?.try_add(dy) else {continue;};
//             if let Some(tile) = self.grid.get_mut(x, y) {
//                 return Some(((x, y), tile));
//             }
//         }
//         None
//     }
// }
