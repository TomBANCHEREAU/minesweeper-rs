// use super::tile::Tile;
// use rand::prelude::*;
// type GridIndex = usize;

// pub const REGION_SIZE: GridIndex = 100;

// pub enum Region {
//     Playable(PlayableRegion),
//     Populated(PopulatedRegion),
//     Blank,
// }

// impl Region {
//     pub(crate) fn populate_bombs(&mut self, ratio: f64) {
//         match self {
//             Region::Blank => {
//                 let mut random = ThreadRng::default();
//                 let mut populated_region = PopulatedRegion::default();
//                 for _ in 0usize..((REGION_SIZE * REGION_SIZE) as f64 * ratio) as usize {
//                     populated_region.bombs.push(loop {
//                         let x = random.gen_range(0..REGION_SIZE);
//                         let y = random.gen_range(0..REGION_SIZE);
//                         if !populated_region.bombs.contains(&(x, y)) {
//                             break (x, y);
//                         }
//                     })
//                 }
//                 *self = Self::Populated(populated_region);
//             }
//             _ => (),
//         }
//     }
// }

// pub struct PopulatedRegion {
//     bombs: Vec<(GridIndex, GridIndex)>,
// }

// pub struct PlayableRegion {
//     // size: GridIndex,
//     tiles: [[Tile; REGION_SIZE]; REGION_SIZE],
// }

// impl Default for Region {
//     fn default() -> Self {
//         Region::Blank
//     }
// }

// impl Default for PlayableRegion {
//     fn default() -> Self {
//         Self {
//             tiles: [[Default::default(); REGION_SIZE]; REGION_SIZE],
//         }
//     }
// }

// impl Default for PopulatedRegion {
//     fn default() -> Self {
//         Self { bombs: Vec::new() }
//     }
// }
