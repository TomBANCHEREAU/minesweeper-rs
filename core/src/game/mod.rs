use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::spawn;

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
use crate::tile::{Tile, TileContent, TileState};

// pub trait GameObserver: Observer<GameEvent> {}

#[cfg(feature = "server")]
pub struct Game {
    grid: VecGrid<Tile>,
    sender: Sender<GameEvent>,
    receiver: Receiver<GameInput>,
    populated: bool,
    cursors: HashMap<String, CursorPosition>,
}
#[cfg(feature = "server")]
impl Game {
    pub fn new(
        grid: VecGrid<Tile>,
        sender: Sender<GameEvent>,
        receiver: Receiver<GameInput>,
    ) -> Self {
        Self {
            grid,
            sender,
            receiver,
            populated: false,
            cursors: HashMap::new(),
        }
    }
    pub fn start(mut self) {
        spawn(move || loop {
            let input = self.receiver.recv().unwrap();
            self.play(input)
        });
    }
    pub fn play(&mut self, play: GameInput) {
        let GameInput { action, username } = play;
        match action {
            GameAction::Discover { x, y } => {
                if !self.populated {
                    self.populated = true;
                    self.grid.populate(x, y);
                }
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
            GameAction::RedrawRequest => self.emit_event(GameEvent::GameStart {
                grid: VecGrid::<TileState>::from(&self.grid),
                cursors: self.cursors.clone(),
            }),
            GameAction::CursorMoved(cursor_position) => {
                self.cursors.insert(username.clone(), cursor_position);
                self.emit_event(GameEvent::CursorMoved(username, cursor_position));
            }
        }
    }
    fn emit_event(&mut self, event: GameEvent) {
        self.sender.send(event).unwrap();
        // self.listeners
        //     .iter_mut()
        //     .for_each(|listener| listener.notify(event.clone()));
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

#[derive(Debug, Clone)]
pub struct GameInput {
    pub username: String,
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
    RedrawRequest,
    CursorMoved(CursorPosition),
}

/**
 * What event we receive ?
 * A tile update
 * The game is over
 */
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum GameEvent {
    GameStart {
        grid: VecGrid<TileState>,
        cursors: HashMap<String, CursorPosition>,
    },
    TileStateUpdate {
        x: i32,
        y: i32,
        state: TileState,
    },
    GameOver {},
    CursorMoved(String, CursorPosition),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub struct CursorPosition {
    pub x: i32,
    pub y: i32,
}
