use crate::engine::Engine;
use crate::position::Position;

pub struct Uci {
    pub engine: Engine,
    pub position: Position,
    _start_fen: String,
}

impl Uci {
    pub fn new() -> Uci {
        Uci {
            engine: Engine::new(),
            position: Position::new(),
            _start_fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        }
    }
    pub fn uci_loop(&mut self) {
        // set the starting position
        self.position.set(&self._start_fen);

        // loop until the program is terminated
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            let mut args = input.split_whitespace();
            match args.next() {
                Some("uci") => println!(
                    "Name: {}, Author: {}, Version: {}",
                    self.engine._name, self.engine._author, self.engine._version
                ), // Done
                Some("isready") => println!("readyok"), // Done
                Some("ucinewgame") => println!("ucinewgame"),
                Some("position") => self.position(&mut args),
                Some("go") => println!("go"),
                Some("quit") => break,
                _ => println!("Unknown command: {}", input),
            }
        }
    }

    fn position(&mut self, args: &mut std::str::SplitWhitespace) {
        let arg = args.next();
        let mut fen = String::new();
        match arg {
            Some("startpos") => {
                fen = self._start_fen.clone();
                args.next();
            }
            Some("fen") => {
                while let Some(s) = args.next() {
                    if s == "moves" {
                        break;
                    }
                    fen.push_str(s);
                    fen.push_str(" ");
                }
            }
            _ => (),
        }

        self.position.set(&fen);

        while let Some(s) = args.next() {
            println!("move: {}", s);
        }
    }
}

// Path: src/uci.rs
