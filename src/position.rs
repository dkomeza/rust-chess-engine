use crate::types::*;

pub struct Position {
    pub _board: [i8; 64],
}

impl Position {
    pub fn new() -> Position {
        Position { _board: [0; 64] }
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
                sq = (sq as i8 + 2 * Direction::DOWN as i8) as usize;
            } else {
                let piece = Piece::get_piece(c);
                self._board[sq as usize] = piece;
                sq += Direction::RIGHT as usize;
            }
        }
    }

    pub fn board(&mut self) {
        println!("  +---+---+---+---+---+---+---+---+");
        for rank in 0..8 {
            print!("{} |", 8 - rank);
            for file in 0..8 {
                print!(
                    " {} |",
                    Piece::get_char(self._board[63 - (rank * 8 + file)])
                );
            }
            println!();
            println!("  +---+---+---+---+---+---+---+---+");
        }
        println!("    a   b   c   d   e   f   g   h");
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
