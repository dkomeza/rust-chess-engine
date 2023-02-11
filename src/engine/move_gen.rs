use super::board;

pub struct MoveGen {
    _position: board::Board,
}

impl MoveGen {
    pub fn new() -> MoveGen {
        MoveGen {
            _position: board::Board::new(),
        }
    }

    pub fn position(&mut self, position: board::Board) {
        self._position = position;
    }
}
