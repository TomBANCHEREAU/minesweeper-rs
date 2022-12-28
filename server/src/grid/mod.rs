use std::collections::HashMap;

use self::region::{PlayableRegion, Region};

pub mod region;
pub mod tile;

struct Grid {
    regions: HashMap<(i32, i32), Region>,
}

impl Default for Grid {
    fn default() -> Self {
        let mut regions = HashMap::new();
        // let mut first_region = Region::Blank;
        // first_region.po
        // regions.insert((0, 0), Region::Blank);

        let out = Self { regions };
        out
    }
}

impl Grid {
    fn populate_numbers(&mut self, pos: (i32, i32)) {
        if let Region::Playable(_) = self.regions.entry(pos).or_default() {
            return;
        }
        // for dx in -1..=1 {
        //     for dy in -1..=1 {
        //         let bomb_count = 0f64;
        //         let region = self.regions.entry((pos.0 + dx, pos.1 + dy)).or_default();
        //         region.populate_bombs(bomb_count);
        //     }
        // }
        
        let playable_region = PlayableRegion::default();
        // for bomb_pos in self.regions {}
        // self.regions.
        // let region = self.regions.entry(pos).or_default();
        // let region = if let Some(region) = self.regions.get_mut(&pos) {
        //     region
        // } else {
        //     self.regions.insert(pos, Region::Blank);
        //     self.regions.get_mut(&pos).unwrap()
        // };
        // self.regions.get_many_mut(ks)
    }
}
