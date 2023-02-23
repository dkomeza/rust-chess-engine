#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;

pub mod endgame;
pub mod position;
pub mod search;
pub mod threads;
pub mod types;
pub mod uci;

lazy_static! {
    pub static ref POSITION: Mutex<position::Position> = Mutex::new(position::Position::new());
    pub static ref THREADS: threads::Threads = threads::Threads::new();
    pub static ref ENDGAME: endgame::Endgame = endgame::Endgame::new();
    pub static ref SEARCH: Mutex<search::Search> = Mutex::new(search::Search::new());
}

fn main() {
    let mut uci = uci::Uci::new();
    uci.uci_loop();
}
