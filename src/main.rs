pub mod uci;
pub mod engine;
pub mod position;

fn main() {
    let mut uci = uci::Uci::new();
    uci.uci_loop();
}
