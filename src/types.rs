pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
    NB,
}

#[derive(Copy, Clone)]
pub enum Piece {
    NONE,
    WP, WN, WB, WR, WQ, WK,
    BP = 9, BN, BB, BR, BQ, BK,
    NP = 16
}

impl Piece {
    pub fn index(&self) -> usize {
        *self as usize
    }
    pub fn from_char(c: char) -> Piece {
        match c {
            'P' => Piece::WP,
            'N' => Piece::WN,
            'B' => Piece::WB,
            'R' => Piece::WR,
            'Q' => Piece::WQ,
            'K' => Piece::WK,
            'p' => Piece::BP,
            'n' => Piece::BN,
            'b' => Piece::BB,
            'r' => Piece::BR,
            'q' => Piece::BQ,
            'k' => Piece::BK,
            _ => Piece::NONE,
        }
    }
    pub fn make_piece(color: Color, piece_type: PieceType) -> Piece {
        match color {
            Color::WHITE => Piece::from_char(piece_type as u8 as char),
            Color::BLACK => Piece::from_char((piece_type as u8 as char).to_lowercase().next().unwrap()),
            _ => Piece::NONE,
        }
    }
}

pub enum PieceType {
    NONE, P, N, B, R, Q, K,
    NB = 8
}

pub enum Direction {
    UP = 8,
    RIGHT = 1,
    DOWN = -8,
    LEFT = -1,

    UPRIGHT = 9,
    DOWNRIGHT = -7,
    DOWNLEFT = -9,
    UPLEFT = 7,
}

pub const PIECE_CHARS: &str = " PNBRQK  pnbrqk";

pub enum Color {
    WHITE,
    BLACK,
    NB
}