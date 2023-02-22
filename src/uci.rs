use crate::position;
use crate::types::*;
use crate::SEARCH;

pub struct Uci {
    _name: &'static str,
    _author: &'static str,
    _version: &'static str,
    _start_fen: &'static str,
    _position: position::Position,
}

impl Uci {
    pub fn new() -> Uci {
        Uci {
            _name: "Rusty",
            _author: "dkomeza",
            _version: "0.1.0",
            _start_fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            _position: position::Position::new(),
        }
    }

    pub fn uci_loop(&mut self) {
        self._position.set(self._start_fen);
        // create variables for storing the input
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
                "go" => self.go(args),
                // "stop" => self.stop(&mut stdout),
                "quit" => break,
                "board" => self.board(),
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

        if fen.is_empty() {
            fen.push_str(start_fen);
        }

        self._position.set(&fen);

        if args.next().unwrap_or("") == "moves" {
            while let Some(arg) = args.next() {
                self._position.make_move(arg);
            }
            // self._position.board();
            // println!("Castle rights: {}", self._position._castle_rights);
            // println!("Side to move: {}", self._position._side_to_move);
        }
    }

    fn go(&mut self, mut args: std::str::SplitWhitespace) {
        let mut limits = SEARCH._limits.clone();
        let mut ponder_mode = false;

        limits._starttime = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i32;

        while let Some(arg) = args.next() {
            match arg {
                "searchmoves" => {
                    while let Some(arg) = args.next() {
                        // limits._moves.push(Move::from_uci(arg));
                    }
                }
                "wtime" => {
                    limits._time[Side::WHITE as usize] = args.next().unwrap_or("0").parse().unwrap()
                }
                "btime" => {
                    limits._time[Side::BLACK as usize] = args.next().unwrap_or("0").parse().unwrap()
                }
                "winc" => {
                    limits._inc[Side::WHITE as usize] = args.next().unwrap_or("0").parse().unwrap()
                }
                "binc" => {
                    limits._inc[Side::BLACK as usize] = args.next().unwrap_or("0").parse().unwrap()
                }
                "movestogo" => limits._movestogo = args.next().unwrap_or("0").parse().unwrap(),
                "depth" => limits._depth = args.next().unwrap_or("0").parse().unwrap(),
                "nodes" => limits._nodes = args.next().unwrap_or("0").parse().unwrap(),
                "mate" => limits._mate = args.next().unwrap_or("0").parse().unwrap(),
                "movetime" => limits._movetime = args.next().unwrap_or("0").parse().unwrap(),
                "infinite" => limits._infinite = true,
                "ponder" => ponder_mode = true,
                _ => (),
            }
        }

        // THREADS.start(pos, states, limits, ponder_mode)
    }

    fn board(&mut self) {
        self._position.board();
    }
}
