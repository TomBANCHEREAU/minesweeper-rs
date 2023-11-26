use self::facade::LobbyFacade;
use std::{collections::HashMap, sync::Mutex};
pub mod actor;
pub mod facade;
pub mod handle;
pub mod lobby;

pub type Lobbies = Mutex<HashMap<String, LobbyFacade>>;
