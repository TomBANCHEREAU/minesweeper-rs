use crate::tile::{ClientTile, ServerTile, TileContent};
#[cfg(feature = "server")]
use rand::prelude::*;

#[cfg(feature = "server")]
#[derive(Debug)]
pub struct ServerGrid {
    tiles: Vec<Vec<ServerTile>>,
}

#[derive(Debug)]
pub struct ClientGrid {
    tiles: Vec<Vec<ClientTile>>,
}

const NEIGHBORS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
//

#[cfg(feature = "server")]
impl ServerGrid {
    pub fn generate(
        (width, height): (usize, usize),
        (start_point_x, start_point_y): (usize, usize),
        bomb_ratio: f64,
    ) -> ServerGrid {
        let mut random = ThreadRng::default();
        let mut tiles: Vec<Vec<ServerTile>> = Vec::with_capacity(height);
        for _ in 0..width {
            tiles.push(Vec::with_capacity(width));
        }

        let mut bomb_count = ((width * height) as f64 * bomb_ratio) as usize;

        while bomb_count > 0 {
            let x = random.gen_range(0..width);
            let y = random.gen_range(0..height);
            if !matches!(tiles[y][x].content, TileContent::Bomb)
                && (start_point_x + 2 < x
                    || start_point_y + 2 < y
                    || start_point_x > x + 2
                    || start_point_y > y + 2)
            {
                tiles[y][x].content = TileContent::Bomb;
                for (delta_x, delta_y) in NEIGHBORS {
                    let neighbor_x = delta_x + x as i32;
                    let neighbor_y = delta_y + y as i32;
                    if neighbor_x >= 0
                        && neighbor_y >= 0
                        && neighbor_x < width as i32
                        && neighbor_y < height as i32
                    {
                        match &mut tiles[neighbor_y as usize][neighbor_x as usize].content {
                            TileContent::Empty => {
                                tiles[neighbor_y as usize][neighbor_x as usize].content =
                                    TileContent::Number(1)
                            }
                            TileContent::Number(bomb_count) => *bomb_count += 1,
                            TileContent::Bomb => (),
                        }
                    }
                }
                bomb_count -= 1;
            }
        }
        Self { tiles }
    }
}

//

// impl<const S: usize> Default for ServerGrid<S> {
//     fn default() -> Self {
//         Self {
//             tiles: [[ServerTile::default(); S]; S],
//         }
//     }
// }

#[cfg(feature = "server")]
impl From<&ServerGrid> for ClientGrid {
    fn from(value: &ServerGrid) -> ClientGrid {
        ClientGrid {
            tiles: value
                .tiles
                .iter()
                .map(|slice| slice.iter().map(|tile| tile.into()).collect())
                .collect(),
        }
    }
}
