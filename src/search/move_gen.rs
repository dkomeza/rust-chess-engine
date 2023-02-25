use crate::types::*;
use std::collections::HashMap;

use super::StateInfo;

pub struct MoveGen {
    _position: [i8; 64],
    _castling: i8,
    _side_to_move: i8,
    _found_moves: i8,
}

impl MoveGen {
    pub fn new() -> MoveGen {
        MoveGen {
            _position: [0; 64],
            _castling: 0,
            _side_to_move: 0,
            _found_moves: 0,
        }
    }

    pub fn start(&mut self, state_info: StateInfo) {
        self._position = state_info.position.clone();
        self._castling = state_info.castling.clone();
        self._side_to_move = state_info.side_to_move.clone();

        self.create_squares();
        // if self._side_to_move == Color::WHITE as i8 {
        //     for sq in 0..64 {
        //         let piece = self._position[sq];
        //         if piece < Color::BLACK as i8 && piece > 0 {
        //             self.generate_piece_moves(sq, piece)
        //         }
        //     }
        // } else {
        //     for sq in 0..64 {
        //         let piece = self._position[sq];
        //         if piece > Color::BLACK as i8 && piece < 15 {
        //             self.generate_piece_moves(sq, piece)
        //         }
        //     }
        // }
    }

    fn create_squares(&mut self) {
        let king = self
            ._position
            .iter()
            .position(|&x| x == Piece::KING as i8 | self._side_to_move)
            .unwrap_or(0);
        let opponent_color = if self._side_to_move == Color::WHITE as i8 {
            Color::BLACK as i8
        } else {
            Color::WHITE as i8
        };
        self.create_red_squares(king, opponent_color)
    }

    fn create_red_squares(&mut self, king: usize, opponent_color: i8) {
        // create red squares
        let mut red_squares = HashMap::new();

        if self._side_to_move == Color::WHITE as i8 {
            // pawn checks
            if self._position[king + 7] == (Piece::PAWN as i8 | Color::BLACK as i8) {
                red_squares.insert(king + 7, true);
            }
            if self._position[king + 9] == (Piece::PAWN as i8 | Color::BLACK as i8) {
                red_squares.insert(king + 9, true);
            }
        } else {
            // pawn checks
            if self._position[king - 7] == (Piece::PAWN as i8 | Color::WHITE as i8) {
                red_squares.insert(king - 7, true);
            }
            if self._position[king - 9] == (Piece::PAWN as i8 | Color::WHITE as i8) {
                red_squares.insert(king - 9, true);
            }
        }

        // knight checks
        let kinght_checks = [
            king as i8 + 17,
            king as i8 + 15,
            king as i8 + 10,
            king as i8 + 6,
            king as i8 - 17,
            king as i8 - 15,
            king as i8 - 10,
            king as i8 - 6,
        ];
        for sq in kinght_checks.iter() {
            if *sq > 0 && *sq < 64 {
                if self._position[*sq as usize] == (Piece::KNIGHT as i8 | opponent_color) {
                    red_squares.insert(*sq as usize, true);
                }
            }
        }

        // bishop checks
        let bishop_directions: [i8; 4] = [9, 7, -9, -7];
        for direction in 0..4 {
            let mut sq = king;
            let mut squares: Vec<i8> = Vec::new();
            loop {
                let new_sq = sq as i8 + bishop_directions[direction];
                let rank = sq % 8;
                if new_sq < 0 || new_sq > 63 {
                    break;
                }
                if rank == 0 && direction == 0 {
                    break;
                }
                if rank == 0 && direction == 1 {
                    break;
                }
                if rank == 7 && direction == 2 {
                    break;
                }
                if rank == 7 && direction == 3 {
                    break;
                }

                sq = new_sq as usize;
                squares.push(sq as i8);
                if self._position[sq] == (Piece::BISHOP as i8 | opponent_color) {
                    red_squares.insert(sq, true);
                    for square in squares {
                        red_squares.insert(square as usize, true);
                    }
                    break;
                }
                if self._position[sq] != Piece::EMPTY as i8 {
                    break;
                }
            }
        }

        // rook checks
        let rook_directions: [i8; 4] = [8, 1, -8, -1];
        for direction in 0..4 {
            let mut sq = king;
            let mut squares: Vec<i8> = Vec::new();
            loop {
                let new_sq = sq as i8 + rook_directions[direction];
                let rank = sq % 8;
                if new_sq < 0 || new_sq > 63 {
                    break;
                }
                if direction == 1 && rank == 7 {
                    break;
                }
                if direction == 3 && rank == 0 {
                    break;
                }
                sq = new_sq as usize;
                squares.push(sq as i8);
                if self._position[sq] == (Piece::ROOK as i8 | opponent_color) {
                    println!("rook check, sq: {}", sq);
                    red_squares.insert(sq, true);
                    for square in &squares {
                        red_squares.insert(square.to_owned() as usize, true);
                    }
                    break;
                }
                if self._position[sq] != Piece::EMPTY as i8 {
                    break;
                }
            }
            squares.clear();
        }

        // queen checks
        let queen_directions: [i8; 8] = [8, 1, -8, -1, 9, 7, -9, -7];
        for direction in 0..8 {
            let mut sq = king;
            let mut squares: Vec<i8> = Vec::new();
            loop {
                let new_sq = sq as i8 + queen_directions[direction];
                let rank = sq % 8;
                if new_sq < 0 || new_sq > 63 {
                    break;
                }
                if direction == 1 && rank == 7 {
                    break;
                }
                if direction == 3 && rank == 0 {
                    break;
                }
                if direction == 4 && rank == 7 {
                    break;
                }
                if direction == 5 && rank == 7 {
                    break;
                }
                if direction == 6 && rank == 0 {
                    break;
                }
                if direction == 7 && rank == 0 {
                    break;
                }

                sq = new_sq as usize;
                squares.push(sq as i8);
                if self._position[sq] == (Piece::QUEEN as i8 | opponent_color) {
                    red_squares.insert(sq, true);
                    for square in &squares {
                        red_squares.insert(square.to_owned() as usize, true);
                    }
                    break;
                }
                if self._position[sq] != Piece::EMPTY as i8 {
                    break;
                }
            }
            squares.clear();
        }

        // king checks
        let king_checks = [
            king as i8 + 8,
            king as i8 + 1,
            king as i8 - 8,
            king as i8 - 1,
            king as i8 + 9,
            king as i8 + 7,
            king as i8 - 9,
            king as i8 - 7,
        ];
        for sq in king_checks.iter() {
            if *sq > 0 && *sq < 64 {
                if self._position[*sq as usize] == (Piece::KING as i8 | opponent_color) {
                    red_squares.insert(*sq as usize, true);
                }
            }
        }
    }

    fn create_yellow_squares(&mut self) {

    }

    fn generate_piece_moves(&mut self, sq: usize, piece: i8) {
        match piece & 7 {
            1 => self.generate_pawn_moves(sq, piece),
            _ => (),
        }
    }

    fn generate_pawn_moves(&mut self, sq: usize, piece: i8) {
        let direction = if piece > Color::BLACK as i8 {
            Direction::DOWN as i8
        } else {
            Direction::UP as i8
        };
        let rank = sq / 8;
        if rank == 1 {}
    }
}
