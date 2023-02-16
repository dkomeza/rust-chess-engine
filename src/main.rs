pub mod uci;
pub mod endgame;
pub mod threads;

fn main() {
    let mut uci = uci::Uci::new();
    let threads = threads::Threads::new();

    uci.uci_loop();
}
