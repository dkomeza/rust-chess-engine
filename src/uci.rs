pub struct Uci {
    _name: &'static str,
    _author: &'static str,
    _version: &'static str,
}

impl Uci {
    pub fn new() -> Uci {
        Uci {
            _name: "Rusty",
            _author: "dkomeza",
            _version: "0.1.0",
        }
    }

    pub fn uci_loop(&mut self) {
        let mut line = String::new();
        let stdin = std::io::stdin();

        loop {
            line.clear();
            stdin.read_line(&mut line).unwrap();
            let mut args = line.split_whitespace();
            let command = args.next().unwrap_or("");

            match command {
                "uci" => self.uci(),
                "isready" => self.isready(),
                // "ucinewgame" => self.ucinewgame(&mut stdout),
                "position" => self.position(args),
                // "go" => self.go(&mut stdout, args),
                // "stop" => self.stop(&mut stdout),
                "quit" => break,
                _ => (),
            }
        }
    }

    fn uci(&mut self) {
        println!(
            "Name: {}, Author: {}, Version: {}",
            self._name, self._author, self._version
        );
    }

    fn isready(&mut self) {
        println!("readyok");
    }

    fn position(&mut self, mut args: std::str::SplitWhitespace) {
        let start_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let mut fen = String::new();

        match args.next().unwrap_or("") {
            "startpos" => fen.push_str(start_fen),
            "fen" => {
                while let Some(arg) = args.next() {
                    if arg == "moves" {
                        break;
                    }
                    fen.push_str(arg);
                    fen.push(' ');
                }
            }
            _ => (),
        }

        println!("FEN: {}", fen);
    }
}
