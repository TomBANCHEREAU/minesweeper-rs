use core::{
    game::GameAction,
    grid::{vec_grid::VecGrid, Grid},
    tile::{TileContent, TileState},
};

use yew::Callback;

// use web_sys::MouseEvent;
#[derive(Debug)]
pub struct MouseEvent {
    pub x: i32,
    pub y: i32,
    pub button: MouseButton,
}
#[derive(Debug)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Double,
}

pub fn handle_mouse_event(
    event: MouseEvent,
    grid: &VecGrid<TileState>,
    callback: Callback<GameAction>,
) {
    let MouseEvent { x, y, button } = event;
    match (grid.get(x, y), button) {
        (Some(TileState::Untouched), MouseButton::Left) => {
            callback.emit(GameAction::Discover { x, y })
        }
        (Some(TileState::Untouched), MouseButton::Right) => {
            callback.emit(GameAction::PlaceFlag { x, y })
        }
        (Some(TileState::Flagged), MouseButton::Right) => {
            callback.emit(GameAction::RemoveFlag { x, y })
        }
        (
            Some(TileState::Discovered(TileContent::Number(bomb_count))),
            MouseButton::Double | MouseButton::Middle,
        ) => {
            let neighbors = grid.iter_around(x, y);
            let (flag_count, discoverable_tiles) =
                neighbors.fold((0, vec![]), |mut acc, neighbor| {
                    match neighbor.1 {
                        TileState::Untouched => acc.1.push(neighbor.0),
                        TileState::Flagged => acc.0 += 1,
                        TileState::Discovered(_) => (),
                    }
                    return acc;
                });
            if flag_count == *bomb_count {
                for (x, y) in discoverable_tiles {
                    callback.emit(GameAction::Discover {
                        x: x.into(),
                        y: y.into(),
                    })
                }
            }
        }
        (None, _) => (),
        (_, _) => (),
    };
}
