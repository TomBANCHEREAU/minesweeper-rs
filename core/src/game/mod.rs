use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::grid::Grid;
/**
 * How can we describe the game ?
 * We can play a move
 * We can listen to get an update if something change
 */
// pub mod r#trait {
//     pub trait Game<'a> {
//         type Move;
//         type Event;
//         // fn get_width(&self) -> usize;
//         // fn get_height(&self) -> usize;
//         fn play(&mut self, game_move: Self::Move);
//         fn listen(&mut self, callback: &'a dyn Fn(Self::Event) -> ());
//         // fn listen_area(&mut self); ?
//     }
// }
use crate::grid::{vec_grid::VecGrid, NEIGHBORS};
use crate::{
    pubsub::{Observer, Subject},
    tile::{Tile, TileContent, TileState},
};

// pub trait GameObserver: Observer<GameEvent> {}

#[cfg(feature = "server")]
#[derive(Default)]
pub struct Game {
    grid: VecGrid<Tile>,
    listeners: Vec<Box<dyn Observer<GameEvent>>>,
}
#[cfg(feature = "server")]
impl Game {
    pub fn new(grid: VecGrid<Tile>) -> Self {
        Self {
            grid,
            listeners: Default::default(),
        }
    }
    pub fn play(&mut self, play: GameInput) {
        let GameInput {
            action,
            // player: player,
        } = play;
        match action {
            GameAction::Discover { x, y } => {
                self.discover_tile(x, y);
            }
            GameAction::PlaceFlag { x, y } => {
                if let Some(tile) = self.grid.get_mut(x, y) {
                    match tile.state {
                        TileState::Untouched => {
                            tile.state = TileState::Flagged;
                            self.emit_event(GameEvent::TileStateUpdate {
                                x,
                                y,
                                state: TileState::Flagged,
                            })
                        }
                        TileState::Flagged | TileState::Discovered(_) => (),
                    }
                }
            }
            GameAction::RemoveFlag { x, y } => {
                if let Some(tile) = self.grid.get_mut(x, y) {
                    match tile.state {
                        TileState::Flagged => {
                            tile.state = TileState::Untouched;
                            self.emit_event(GameEvent::TileStateUpdate {
                                x,
                                y,
                                state: TileState::Untouched,
                            })
                        }
                        TileState::Untouched | TileState::Discovered(_) => (),
                    }
                }
            }
        }
    }
    fn emit_event(&mut self, event: GameEvent) {
        self.listeners
            .iter_mut()
            .for_each(|listener| listener.notify(event.clone()));
    }
    fn discover_tile(&mut self, x: i32, y: i32) {
        let Some(tile) = self.grid.get_mut(x, y) else { return };
        match tile.state {
            TileState::Untouched => match tile.content {
                TileContent::Empty => {
                    tile.state = TileState::Discovered(tile.content);
                    let state = tile.state;
                    self.emit_event(GameEvent::TileStateUpdate { x, y, state });
                    NEIGHBORS.iter().for_each(|(dx, dy)| {
                        self.discover_tile(x + i32::from(*dx), y + i32::from(*dy))
                    });
                }
                TileContent::Number(_) => {
                    tile.state = TileState::Discovered(tile.content);
                    let state = tile.state;
                    self.emit_event(GameEvent::TileStateUpdate { x, y, state });
                }
                TileContent::Bomb => self.emit_event(GameEvent::GameOver {}),
            },
            TileState::Flagged | TileState::Discovered(_) => (),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GameInput {
    // player: Player,
    pub action: GameAction,
}
/**
 * What action can we do ?
 * Discover a tile
 * Place a flag
 * Remove a flag
 */
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum GameAction {
    Discover { x: i32, y: i32 },
    PlaceFlag { x: i32, y: i32 },
    RemoveFlag { x: i32, y: i32 },
}

/**
 * What event we receive ?
 * A tile update
 * The game is over
 */
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum GameEvent {
    GameStart { grid: VecGrid<TileState> },
    TileStateUpdate { x: i32, y: i32, state: TileState },
    GameOver {},
}

/**
 * Trait iml
 */
#[cfg(feature = "server")]
impl Subject<GameEvent> for Game {
    fn subscribe(&mut self, mut observer: impl Observer<GameEvent> + 'static) {
        observer.notify(GameEvent::GameStart {
            grid: VecGrid::<TileState>::from(&self.grid),
        });
        self.listeners.push(Box::new(observer));
    }

    // fn unsubscribe(&mut self, observer: impl Observer<GameEvent> + 'static) {
    //     todo!()
    // }
}
