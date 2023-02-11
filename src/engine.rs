mod board;

pub struct Engine {
    pub _name: &'static str,
    pub _author: &'static str,
    pub _version: &'static str,

    _board: board::Board,
}

impl Engine {
    pub fn new(name: &'static str, author: &'static str, version: &'static str) -> Engine {
        Engine {
            _name: name,
            _author: author,
            _version: version,
            _board: board::Board::new(),
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
}

// Path: src/uci.rs
