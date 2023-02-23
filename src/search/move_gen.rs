use crate::types::*;

use super::StateInfo;

pub struct MoveGen {
    _position: [i8; 64],
    _castling: i8,
    _side_to_move: i8,
}

impl MoveGen {
    pub fn new() -> MoveGen {
        MoveGen {
            _position: [0; 64],
            _castling: 0,
            _side_to_move: 0,
        }
    }

    pub fn start(&mut self, state_info: StateInfo) {
        self._position = state_info.position.clone();
        self._castling = state_info.castling.clone();
        self._side_to_move = state_info.side_to_move.clone();

        if self._side_to_move == Color::WHITE as i8 {
            for sq in 0..64 {
                if self._position[sq] < Color::BLACK as i8 && self._position[sq] > 0 {}
            }
        } else {
            for sq in 0..64 {
                if self._position[sq] > Color::BLACK as i8 && self._position[sq] < 15 {
                    
                }
            }
        }
    }
}
