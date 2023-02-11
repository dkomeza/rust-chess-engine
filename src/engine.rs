mod board;
mod move_gen;

pub struct Engine {
    pub _name: &'static str,
    pub _author: &'static str,
    pub _version: &'static str,

    _board: board::Board,
    _move_gen: move_gen::MoveGen,
}

impl Engine {
    pub fn new(name: &'static str, author: &'static str, version: &'static str) -> Engine {
        Engine {
            _name: name,
            _author: author,
            _version: version,
            _board: board::Board::new(),
            _move_gen: move_gen::MoveGen::new(),
        }
    }

    pub fn ucinewgame(&mut self) {
        println!("ucinewgame");
    }

    pub fn isready(&mut self) {
        println!("readyok");
    }

    pub fn position(&mut self, command: std::str::SplitWhitespace) {
        self._board.position(command);
    }

    pub fn go(&mut self, command: std::str::SplitWhitespace) {
        self._move_gen.set_position(self._board._board);
        self.parse_go_command(command);
        self._move_gen.generate_moves();
    }

    pub fn stop(&mut self) {
        println!("stop");
    }

    pub fn print_board(&mut self) {
        self._board.print_board();
    }

    fn parse_go_command(&mut self, command: std::str::SplitWhitespace) {
        let mut command = command;
        match command.next() {
            Some("searchmoves") => println!("searchmoves"),
            Some("ponder") => println!("ponder"),
            Some("wtime") => println!("wtime"),
            Some("btime") => println!("btime"),
            Some("winc") => println!("winc"),
            Some("binc") => println!("binc"),
            Some("movestogo") => println!("movestogo"),
            Some("depth") => println!("{}", command.next().unwrap()),
            Some("nodes") => println!("nodes"),
            Some("mate") => println!("mate"),
            Some("movetime") => println!("movetime"),
            Some("infinite") => println!("infinite"),
            _ => println!("Unknown command"),
        }
    }
}

// Path: src/uci.rs
