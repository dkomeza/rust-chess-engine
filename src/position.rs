use crate::types::*;

pub struct Position {
    pub board: [Piece; Square::NB as usize],
    pub side: Color,
}

impl Position {
    pub fn new() -> Position {
        Position {
            board: [Piece::NONE; Square::NB as usize],
            side: Color::WHITE,
        }
    }

    pub fn set(&mut self, fen: &str) {
        let (mut col, mut row, mut token): (char, char, char);
        let mut square = Square::A8 as i8;

        let mut fen = fen.split_whitespace();

        // place the pieces
        for c in fen.next().unwrap().chars() {
            if c == ' ' {
                break;
            }
            if c.is_digit(10) {
                square += c.to_digit(10).unwrap() as i8;
            } else if c == '/' {
                square += 2 * Direction::DOWN as i8;
            } else if PIECE_CHARS.find(c) != None {
                let piece = Piece::from_char(c);
                self.set_piece(piece, square);
                square += Direction::RIGHT as i8;
            }
        }

        // set the side to move
        self.side = if fen.next().unwrap() == "w" {
            Color::WHITE
        } else {
            Color::BLACK
        };

        // set the castling rights
        for c in fen.next().unwrap().chars() {
            let mut square = Square::A1 as i8;
            let color = if c.is_lowercase() {
                Color::BLACK
            } else {
                Color::WHITE
            };
            let rook = Piece::make_piece(color, PieceType::R);

            let char = c.to_uppercase().next().unwrap();

            if char == 'K' {
                self.set_piece(rook, Square::H1 as i8);
            } else if char == 'Q' {
                self.set_piece(rook, Square::A1 as i8);
            }
        }
    }

    fn set_piece(&mut self, piece: Piece, square: i8) {
        self.board[square as usize] = piece;
    }

    fn set_castling_rights(&mut self, c: Color, rfrom: Square) {}
}

// Path: src/position.rs
