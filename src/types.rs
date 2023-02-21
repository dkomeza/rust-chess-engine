#[derive(Copy)]
pub enum Move {
    NONE,
    NULL = 65,
}

impl Clone for Move {
    fn clone(&self) -> Move {
        *self
    }
}

pub enum Side {
    WHITE,
    BLACK,
}

pub enum Square {
    A1,
    B1,
    C1,
    D1,
    E1,
    F1,
    G1,
    H1,
    A2,
    B2,
    C2,
    D2,
    E2,
    F2,
    G2,
    H2,
    A3,
    B3,
    C3,
    D3,
    E3,
    F3,
    G3,
    H3,
    A4,
    B4,
    C4,
    D4,
    E4,
    F4,
    G4,
    H4,
    A5,
    B5,
    C5,
    D5,
    E5,
    F5,
    G5,
    H5,
    A6,
    B6,
    C6,
    D6,
    E6,
    F6,
    G6,
    H6,
    A7,
    B7,
    C7,
    D7,
    E7,
    F7,
    G7,
    H7,
    A8,
    B8,
    C8,
    D8,
    E8,
    F8,
    G8,
    H8,
    NONE,
}

pub enum Piece {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn = 8,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
    Empty = 16,
}

impl Piece {
    pub fn get_piece(piece: char) -> i8 {
        match piece {
            'p' => Piece::BlackPawn as i8,
            'n' => Piece::BlackKnight as i8,
            'b' => Piece::BlackBishop as i8,
            'r' => Piece::BlackRook as i8,
            'q' => Piece::BlackQueen as i8,
            'k' => Piece::BlackKing as i8,
            'P' => Piece::WhitePawn as i8,
            'N' => Piece::WhiteKnight as i8,
            'B' => Piece::WhiteBishop as i8,
            'R' => Piece::WhiteRook as i8,
            'Q' => Piece::WhiteQueen as i8,
            'K' => Piece::WhiteKing as i8,
            _ => panic!("Invalid FEN string"),
        }
    }
}

pub enum Direction {
    UP = 8,
    DOWN = -8,
    LEFT = -1,
    RIGHT = 1,
    UPRIGHT = 9,
    UPLEFT = 7,
    DOWNRIGHT = -7,
    DOWNLEFT = -9,
}
