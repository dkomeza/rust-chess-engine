use crate::types::*;

pub struct Position {
    _board: [i8; 64],
    _side_to_move: i8,
    _castle_rights: i8,
}

impl Position {
    pub fn new() -> Position {
        Position {
            _board: [0; 64],
            _side_to_move: 0,
            _castle_rights: 0,
        }
    }

    pub fn set(&mut self, fen: &str) {
        self._board = [0; 64];
        let mut sq = Square::A8 as usize;
        let mut fen = fen.split_whitespace();
        let board = fen.next().unwrap();

        // set the board
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

        // set the side to move
        self._side_to_move = if fen.next().unwrap_or("") == "w" {
            Color::WHITE as i8
        } else {
            Color::BLACK as i8
        };

        // set the castling rights
        let castling = fen.next().unwrap_or("");
        if castling != "-" {
            for c in castling.chars() {
                match c {
                    'K' => self._castle_rights |= Castling::WhiteKingside as i8,
                    'Q' => self._castle_rights |= Castling::WhiteQueenside as i8,
                    'k' => self._castle_rights |= Castling::BlackKingside as i8,
                    'q' => self._castle_rights |= Castling::BlackQueenside as i8,
                    _ => (),
                }
            }
        }

        // set the en passant square
        let en_passant = fen.next().unwrap_or("");
        if en_passant != "-" {
            let mut sq = Square::A1 as usize;
            for c in en_passant.chars() {
                if c.is_digit(10) {
                    sq += ((c.to_digit(10).unwrap_or(0) - 1) as u8 * Direction::UP as u8) as usize;
                } else {
                    sq += FILES.binary_search(&c).unwrap_or(0) * Direction::RIGHT as usize;
                }
            }
            self._board[sq] = Piece::EnPassant as i8;
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
