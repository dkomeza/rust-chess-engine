pub mod engine;
pub mod uci;

fn main() {
    let engine = engine::Engine::new("RustChess", "RustChess", "0.1");
    let mut uci = uci::Uci { engine };
    loop {
        let mut command = String::new();
        std::io::stdin().read_line(&mut command).unwrap();
        uci.parse_command(command);
    }
}
