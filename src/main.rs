pub mod uci;

fn main() {
    let mut uci = uci::Uci {
        _name: "RustChess",
        _author: "RustChess",
        _version: "0.1",
    };
    loop {
        let mut command = String::new();
        command.clear();
        std::io::stdin().read_line(&mut command).unwrap();
        uci.parse_command(command);
    }
}
