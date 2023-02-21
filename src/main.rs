#[macro_use]
extern crate lazy_static;

pub mod endgame;
pub mod position;
pub mod threads;
pub mod uci;
pub mod search;
pub mod types;

lazy_static! {
    pub static ref POSITION: position::Position = position::Position::new();
    pub static ref THREADS: threads::Threads = threads::Threads::new();
    pub static ref ENDGAME: endgame::Endgame = endgame::Endgame::new();
    pub static ref SEARCH: search::Search = search::Search::new();
}

fn main() {
    let mut uci = uci::Uci::new();
    uci.uci_loop();
}
