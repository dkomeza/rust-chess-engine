use crate::types::*;

pub struct Position {
    _board: [i8; 64],
}

impl Position {
    pub fn new() -> Position {
        Position {
            _board: [0; 64],
        }
    }

    pub fn set(&mut self, fen: &str) {
        let mut sq = Square::A8 as usize;
        let board = fen.split_whitespace().next().unwrap();
        
        for c in board.chars() {
            if c == ' ' {
                break;
            }
            if c.is_digit(10) {
                sq += c.to_digit(10).unwrap() as usize;
            } else if c == '/' {
                sq += 2 * Direction::DOWN as usize;
            } else {
                let piece = Piece::get_piece(c);
                self._board[sq as usize] = piece;
                sq += Direction::RIGHT as usize;
            }
        }
    }
}

pub struct StateInfo {
    pub pawn_key: u64,
    pub material_key: u64,
    pub non_pawn_material: [i32; 2],
    pub _castling_rights: u8,
    pub _rule50: i8,
    pub _plies_from_null: i8,
    pub ep_square: i8,
    pub _key: u64,
    pub _checkers: u64,
}
