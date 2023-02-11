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
        self._move_gen.generate_moves('w');
        let mut iter = self._move_gen._found_positions.iter();
        let mut current = 0;
        while let Some(position) = iter.next() {
            current += 1;
            println!("current position: {}", current);
            self._board.print_position(position);
            println!("---------------------------------------------------------");
        }
    }

    pub fn stop(&mut self) {
        println!("stop");
    }

    pub fn print_board(&mut self) {
        self._board.print_board();
    }

    fn parse_go_command(&mut self, command: std::str::SplitWhitespace) -> &str {
        let mut command = command;
        match command.next() {
            // Some("depth") => return command.next().unwrap(),
            // Some("infinite") => return "0",
            _ => return "",
        }
    }
}

// Path: src/uci.rs
