pub struct MoveGen {
    _position: [[char; 8]; 8],
    _col_names: [char; 8],
    _found_moves: i128,
    pub _found_positions: Vec<[[char; 8]; 8]>,
}

impl MoveGen {
    pub fn new() -> MoveGen {
        MoveGen {
            _position: [
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            ],
            _col_names: ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'],
            _found_moves: 0,
            _found_positions: Vec::new(),
        }
    }

    pub fn set_position(&mut self, position: [[char; 8]; 8]) {
        self._position = position;
        self._found_moves = 0;
    }

    pub fn generate_moves(&mut self, color: char) {
        self._found_positions.clear();
        self._found_moves = 0;
        for i in 0..8 {
            for j in 0..8 {
                if self._position[i as usize][j as usize].is_uppercase() == (color == 'w') {
                    self.generate_piece_moves(i, j);
                } else if self._position[i as usize][j as usize].is_lowercase() == (color == 'b') {
                    self.generate_piece_moves(i, j);
                }
            }
        }
    }

    fn generate_piece_moves(&mut self, row: usize, col: usize) {
        let piece = self._position[row][col];
        if self.king_in_check(piece, self._position) {
            println!("King in check!");
        }
        match piece {
            // 'P' => self.generate_pawn_moves(col, row, piece),
            // 'R' => self.generate_rook_moves(col, row, piece),
            // 'N' => self.generate_knight_moves(col, row, piece),
            // 'B' => self.generate_bishop_moves(col, row, piece),
            // 'Q' => self.generate_queen_moves(col, row, piece),
            // 'K' => self.generate_king_moves(col, row, piece),
            // 'p' => self.generate_pawn_moves(col, row, piece),
            // 'r' => self.generate_rook_moves(col, row, piece),
            // 'n' => self.generate_knight_moves(col, row, piece),
            // 'b' => self.generate_bishop_moves(col, row, piece),
            // 'q' => self.generate_queen_moves(col, row, piece),
            // 'k' => self.generate_king_moves(col, row, piece),
            _ => (),
        }
    }

    fn king_in_check(&mut self, piece: char, board: [[char; 8]; 8]) -> bool {
        let king = if piece.is_uppercase() { 'K' } else { 'k' };
        let mut king_pos = (0, 0);

        for i in 0..8 {
            for j in 0..8 {
                if board[i][j] == king {
                    king_pos = (i, j);
                }
            }
        }

        if king.is_uppercase() {
            // Check for pawn checks
            if king_pos.0 > 0 && king_pos.1 > 0 {
                if board[king_pos.0 - 1][king_pos.1 - 1] == 'p' {
                    return true;
                }
            }
            if king_pos.0 > 0 && king_pos.1 < 7 {
                if board[king_pos.0 - 1][king_pos.1 + 1] == 'p' {
                    return true;
                }
            }

            // Check for knight checks
            for i in -2i8..=2 {
                for j in -2i8..=2 {
                    if (i == 2 || i == -2) && (j == 1 || j == -1) {
                        if self.is_on_board(king_pos.0 as i8 + i, king_pos.1 as i8 + j) {
                            if board[(king_pos.0 as i8 + i) as usize]
                                [(king_pos.1 as i8 + j) as usize]
                                == 'n'
                            {
                                return true;
                            }
                        }
                    } else if (i == 1 || i == -1) && (j == 2 || j == -2) {
                        if self.is_on_board(king_pos.0 as i8 + i, king_pos.1 as i8 + j) {
                            if board[(king_pos.0 as i8 + i) as usize]
                                [(king_pos.1 as i8 + j) as usize]
                                == 'n'
                            {
                                return true;
                            }
                        }
                    }
                }
            }

            // Check for bishop/queen checks
            for direction in 0..4 {
                let mut i = king_pos.0 as i8;
                let mut j = king_pos.1 as i8;
                loop {
                    match direction {
                        0 => {
                            i += 1;
                            j += 1;
                        }
                        1 => {
                            i += 1;
                            j -= 1;
                        }
                        2 => {
                            i -= 1;
                            j += 1;
                        }
                        3 => {
                            i -= 1;
                            j -= 1;
                        }
                        _ => (),
                    }
                    if self.is_on_board(i, j) {
                        if board[i as usize][j as usize] == 'b'
                            || board[i as usize][j as usize] == 'q'
                        {
                            return true;
                        } else if board[i as usize][j as usize] != ' ' {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }

            // Check for rook/queen checks
            for direction in 0..4 {
                let mut i = king_pos.0 as i8;
                let mut j = king_pos.1 as i8;
                loop {
                    match direction {
                        0 => i += 1,
                        1 => i -= 1,
                        2 => j += 1,
                        3 => j -= 1,
                        _ => (),
                    }
                    if self.is_on_board(i, j) {
                        if board[i as usize][j as usize] == 'r'
                            || board[i as usize][j as usize] == 'q'
                        {
                            return true;
                        } else if board[i as usize][j as usize] != ' ' {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        } else {
        }

        false
    }

    fn is_on_board(&self, row: i8, col: i8) -> bool {
        row >= 0 && row < 8 && col >= 0 && col < 8
    }

    fn create_position(&mut self, old: [i8; 2], new: [i8; 2]) {
        self._found_moves += 1;
        let mut new_position = self._position;
        new_position[7 - old[0] as usize][old[1] as usize] = ' ';
        new_position[7 - new[0] as usize][new[1] as usize] =
            self._position[7 - old[0] as usize][old[1] as usize];
        self._found_positions.push(new_position);
    }
}
