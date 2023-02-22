use crate::types::*;

pub struct Position {
    _board: [i8; 64],
    pub _side_to_move: i8,
    pub _castle_rights: i8,
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
            self._board[sq] = Piece::ENPASSANT as i8;
            if self._side_to_move == Color::WHITE as i8 {
                self._board[sq] |= Color::BLACK as i8;
            }
        }
    }

    pub fn make_move(&mut self, m: &str) {
        let chars: Vec<char> = m.chars().collect();
        self._clear_enpassant();
        if m == "O-O" {
            self._castle_right();
        } else if m == "O-O-O" {
            self._castle_left();
        } else if chars.contains(&'x') {
            self._make_capture(&chars);
        } else if chars.len() == 2 {
            self._make_pawn_move(&chars);
        } else if chars[0].is_uppercase() {
            self._make_piece_move(&chars);
        }
        // switch sides after a move
        self.board();
        self._side_to_move = Color::BLACK as i8 - self._side_to_move;
    }

    fn _make_capture(&mut self, m: &Vec<char>) {
        if m[0].is_lowercase() {
            self._make_pawn_capture(m);
        } else {
            self._make_piece_capture(m);
        }
    }

    fn _castle_right(&mut self) {
        let (king, rook) = if self._side_to_move == Color::WHITE as i8 {
            (Square::E1 as usize, Square::H1 as usize)
        } else {
            (Square::E8 as usize, Square::H8 as usize)
        };
        self._board[king] = Piece::EMPTY as i8;
        self._board[rook] = Piece::EMPTY as i8;
        self._board[(king + Direction::RIGHT as usize)] = Piece::KING as i8 | self._side_to_move;
        self._board[(rook - Direction::RIGHT as usize)] = Piece::ROOK as i8 | self._side_to_move;
        if self._side_to_move == Color::WHITE as i8 {
            self._castle_rights ^= Castling::WhiteKingside as i8;
            self._castle_rights ^= Castling::WhiteQueenside as i8;
        } else {
            self._castle_rights ^= Castling::BlackKingside as i8;
            self._castle_rights ^= Castling::BlackQueenside as i8;
        };
    }

    fn _castle_left(&mut self) {
        let (king, rook) = if self._side_to_move == Color::WHITE as i8 {
            (Square::E1 as i8, Square::A1 as i8)
        } else {
            (Square::E8 as i8, Square::A8 as i8)
        };
        self._board[king as usize] = Piece::EMPTY as i8;
        self._board[rook as usize] = Piece::EMPTY as i8;
        self._board[(king + 2 * Direction::LEFT as i8) as usize] =
            Piece::KING as i8 | self._side_to_move;
        self._board[(king + Direction::LEFT as i8) as usize] =
            Piece::ROOK as i8 | self._side_to_move;
        if self._side_to_move == Color::WHITE as i8 {
            self._castle_rights ^= Castling::WhiteKingside as i8;
            self._castle_rights ^= Castling::WhiteQueenside as i8;
        } else {
            self._castle_rights ^= Castling::BlackKingside as i8;
            self._castle_rights ^= Castling::BlackQueenside as i8;
        };
    }

    fn _make_pawn_move(&mut self, m: &Vec<char>) {
        let target_sq = Square::get_square(&m[0], &m[1]);
        let mut pawn_sq = target_sq as i8;
        let mut ep_sq = -1i8;
        if self._side_to_move == Color::WHITE as i8 {
            pawn_sq += Direction::DOWN as i8;
            if self._board[pawn_sq as usize] == Piece::EMPTY as i8 {
                ep_sq = pawn_sq;
                pawn_sq += Direction::DOWN as i8;
            }
        } else {
            pawn_sq += Direction::UP as i8;
            if self._board[pawn_sq as usize] == Piece::EMPTY as i8 {
                ep_sq = pawn_sq;
                pawn_sq += Direction::UP as i8;
            }
        }
        self._board[target_sq] = Piece::PAWN as i8 | self._side_to_move;
        self._board[pawn_sq as usize] = Piece::EMPTY as i8;
        if ep_sq != -1 {
            self._board[ep_sq as usize] = Piece::ENPASSANT as i8 | self._side_to_move;
        }
    }

    fn _make_piece_move(&mut self, m: &Vec<char>) {
        let target_sq = Square::get_square(&m[1], &m[2]);
        let piece = Piece::get_piece(m[0]);
        let mut sq = Square::A1 as usize;
        for i in 0..64 {
            if self._board[i] == piece | self._side_to_move {
                if self._is_pseudo_legal(piece, i, target_sq) {
                    sq = i;
                    break;
                }
            }
        }
        self._board[target_sq] = piece | self._side_to_move;
        self._board[sq] = Piece::EMPTY as i8;
    }

    fn _make_pawn_capture(&mut self, m: &Vec<char>) {
        let target_sq = Square::get_square(&m[2], &m[3]);
        let mut pawn_sq = Square::get_square(&m[0], &m[3]);
        if self._side_to_move == Color::WHITE as i8 {
            pawn_sq = (pawn_sq as i8 + Direction::DOWN as i8) as usize;
        } else {
            pawn_sq = (pawn_sq as i8 + Direction::UP as i8) as usize;
        }

        self._board[target_sq] = Piece::PAWN as i8 | self._side_to_move;
        self._board[pawn_sq] = Piece::EMPTY as i8;
    }

    fn _make_piece_capture(&mut self, m: &Vec<char>) {
        let target_sq = Square::get_square(&m[&m.len() - 2], &m[&m.len() - 1]);
        let piece = Piece::get_piece(m[0]);
        if m[1] == 'x' {
            let mut sq = Square::A1 as usize;
            for i in 0..64 {
                if self._board[i] == piece | self._side_to_move {
                    if self._is_pseudo_legal(piece, i, target_sq) {
                        sq = i;
                        break;
                    }
                }
            }
            self._board[target_sq] = piece | self._side_to_move;
            self._board[sq] = Piece::EMPTY as i8;
        } else if m.len() == 5 {
            if m[1].is_digit(10) {
                let row = m[1].to_digit(10).unwrap() as usize;
                for i in 0..64 {
                    if self._board[i] == piece | self._side_to_move {
                        if Square::get_rank(i) == row {
                            if self._is_pseudo_legal(piece, i, target_sq) {
                                self._board[target_sq] = piece | self._side_to_move;
                                self._board[i] = Piece::EMPTY as i8;
                                break;
                            }
                        }
                    }
                }
            } else {
                let col = &m[1];
                for i in 0..64 {
                    if self._board[i] == piece | self._side_to_move {
                        if Square::get_file(i) == *col {
                            if self._is_pseudo_legal(piece, i, target_sq) {
                                self._board[target_sq] = piece | self._side_to_move;
                                self._board[i] = Piece::EMPTY as i8;
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn board(&mut self) {
        println!("  +---+---+---+---+---+---+---+---+");
        for rank in (0..8).rev() {
            print!("{} |", rank + 1);
            for file in 0..8 {
                print!(" {} |", Piece::get_char(self._board[rank * 8 + file]));
            }
            println!();
            println!("  +---+---+---+---+---+---+---+---+");
        }
        println!("    a   b   c   d   e   f   g   h");
    }

    fn _is_pseudo_legal(&mut self, piece: i8, source: usize, target: usize) -> bool {
        // check if the piece is the knight
        if piece & 7 == Piece::KNIGHT as i8 {
            let diff = (source as i8 - target as i8).abs();
            return diff == 6 || diff == 10 || diff == 15 || diff == 17;
        }

        // check if the piece is the king
        if piece & 7 == Piece::KING as i8 {
            let diff = (source as i8 - target as i8).abs();
            return diff == 1 || diff == 7 || diff == 8 || diff == 9;
        }

        // check if the piece is the rook
        if piece & 7 == Piece::ROOK as i8 {
            let diff = (source as i8 - target as i8).abs();
            return diff % 8 == 0 || diff < 8;
        }

        // check if the piece is the bishop
        if piece & 7 == Piece::BISHOP as i8 {
            let diff = (source as i8 - target as i8).abs();
            return diff % 7 == 0 || diff % 9 == 0;
        }

        // check if the piece is the queen
        if piece & 7 == Piece::QUEEN as i8 {
            let diff = (source as i8 - target as i8).abs();
            return diff % 7 == 0 || diff % 9 == 0 || diff % 8 == 0 || diff < 8;
        }
        false
    }

    fn _clear_enpassant(&mut self) {
        for i in 0..self._board.len() {
            if (self._board[i] & 7) == Piece::ENPASSANT as i8 {
                self._board[i] = Piece::EMPTY as i8;
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
