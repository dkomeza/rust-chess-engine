pub struct MoveGen {
    _position: [[char; 8]; 8],
    _col_names: [char; 8],
    _en_passant: String,
    _castling: String,
    _found_moves: Vec<String>,
    pub _found_positions: Vec<FoundPosition>,
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
            _found_positions: Vec::new(),
            _found_moves: Vec::new(),
            _en_passant: String::from("-"),
            _castling: String::from("-"),
        }
    }

    pub fn set_position(&mut self, position: [[char; 8]; 8]) {
        self._position = position;
    }

    pub fn generate_moves(&mut self, color: char) {
        self._found_positions.clear();
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
        match piece {
            'P' => self.generate_pawn_moves(row, col, piece),
            // 'R' => self.generate_rook_moves(row, col, piece),
            // 'N' => self.generate_knight_moves(row, col, piece),
            // 'B' => self.generate_bishop_moves(row, col, piece),
            // 'Q' => self.generate_queen_moves(row, col, piece),
            'K' => self.generate_king_moves(row, col, piece),
            'p' => self.generate_pawn_moves(row, col, piece),
            // 'r' => self.generate_rook_moves(row, col, piece),
            // 'n' => self.generate_knight_moves(row, col, piece),
            // 'b' => self.generate_bishop_moves(row, col, piece),
            // 'q' => self.generate_queen_moves(row, col, piece),
            'k' => self.generate_king_moves(row, col, piece),
            _ => (),
        }
    }

    fn generate_pawn_moves(&mut self, row: usize, col: usize, piece: char) {
        let direction = if piece.is_uppercase() { -1 } else { 1 };

        // Double push
        if piece == 'P' && row == 6 {
            if self._position[row - 1][col] == ' '
                && self.is_legal_move([row as i8, col as i8], [row as i8 - 2, col as i8], piece)
            {
                self.create_position([row as i8, col as i8], [row as i8 - 2, col as i8], piece);
            }
        } else if piece == 'p' && row == 1 {
            if self._position[row + 1][col] == ' '
                && self.is_legal_move([row as i8, col as i8], [row as i8 + 2, col as i8], piece)
            {
                self.create_position([row as i8, col as i8], [row as i8 + 2, col as i8], piece);
            }
        }

        // Single push
        if self.is_legal_move(
            [row as i8, col as i8],
            [row as i8 + direction, col as i8],
            piece,
        ) {
            self.create_position(
                [row as i8, col as i8],
                [row as i8 + direction, col as i8],
                piece,
            );
        }

        // Capture left
        if col > 0 {
            // Normal capture
            if self.is_legal_move(
                [row as i8, col as i8],
                [row as i8 + direction, col as i8 - 1],
                piece,
            ) && self.is_oponent(piece, [row as i8 + direction, col as i8 - 1])
            {
                self.create_position(
                    [row as i8, col as i8],
                    [row as i8 + direction, col as i8 - 1],
                    piece,
                );
            }

            // En passant capture
            if piece == 'P' && self._position[(row as i8 + direction) as usize][col - 1] == 'x' {
                self.create_position(
                    [row as i8, col as i8],
                    [row as i8 + direction, col as i8 - 1],
                    piece,
                );
            } else if piece == 'p'
                && self._position[(row as i8 + direction) as usize][col - 1] == 'X'
            {
                self.create_position(
                    [row as i8, col as i8],
                    [row as i8 + direction, col as i8 - 1],
                    piece,
                );
            }
        }

        // Capture right
        if col < 7 {
            if self.is_legal_move(
                [row as i8, col as i8],
                [row as i8 + direction, col as i8 + 1],
                piece,
            ) && self.is_oponent(piece, [row as i8 + direction, col as i8 + 1])
            {
                self.create_position(
                    [row as i8, col as i8],
                    [row as i8 + direction, col as i8 + 1],
                    piece,
                );
            }

            // En passant capture
            if piece == 'P' && self._position[(row as i8 + direction) as usize][col + 1] == 'x' {
                self.create_position(
                    [row as i8, col as i8],
                    [row as i8 + direction, col as i8 + 1],
                    piece,
                );
            } else if piece == 'p'
                && self._position[(row as i8 + direction) as usize][col + 1] == 'X'
            {
                self.create_position(
                    [row as i8, col as i8],
                    [row as i8 + direction, col as i8 + 1],
                    piece,
                );
            }
        }
    }

    // fn generate_rook_moves(&mut self, row: usize, col: usize, piece: char) {
    //     for direction in 0i8..=3 {
    //         let mut i = row as i8;
    //         let mut j = col as i8;
    //         let mut found = false;
    //         loop {
    //             if found {
    //                 break;
    //             }
    //             match direction {
    //                 0 => i += 1,
    //                 1 => i -= 1,
    //                 2 => j += 1,
    //                 3 => j -= 1,
    //                 _ => (),
    //             }
    //             if i < 0 || i > 7 || j < 0 || j > 7 {
    //                 break;
    //             }
    //             let field = self._position[i as usize][j as usize];
    //             if !self.is_legal_move([row as i8, col as i8], [i, j], piece) && field != ' ' {
    //                 break;
    //             }
    //             if self.is_legal_move([row as i8, col as i8], [i, j], piece) {
    //                 if field != ' ' {
    //                     found = true;
    //                 }
    //                 self.create_position([row as i8, col as i8], [i, j], piece, [-1, -1])
    //             }
    //         }
    //     }
    // }

    // fn generate_knight_moves(&mut self, row: usize, col: usize, piece: char) {
    //     for i in -2i8..=2 {
    //         for j in -2i8..=2 {
    //             if (i == 2 || i == -2) && (j == 1 || j == -1) {
    //                 if self.is_legal_move(
    //                     [row as i8, col as i8],
    //                     [row as i8 + i, col as i8 + j],
    //                     piece,
    //                 ) {
    //                     self.create_position(
    //                         [row as i8, col as i8],
    //                         [row as i8 + i, col as i8 + j],
    //                         piece,
    //                         [-1, -1],
    //                     )
    //                 }
    //             } else if (i == 1 || i == -1) && (j == 2 || j == -2) {
    //                 if self.is_legal_move(
    //                     [row as i8, col as i8],
    //                     [row as i8 + i, col as i8 + j],
    //                     piece,
    //                 ) {
    //                     self.create_position(
    //                         [row as i8, col as i8],
    //                         [row as i8 + i, col as i8 + j],
    //                         piece,
    //                         [-1, -1],
    //                     )
    //                 }
    //             }
    //         }
    //     }
    // }

    // fn generate_bishop_moves(&mut self, row: usize, col: usize, piece: char) {
    //     for direction in 0i8..=3 {
    //         let mut i = row as i8;
    //         let mut j = col as i8;
    //         let mut found = false;
    //         loop {
    //             if found {
    //                 break;
    //             }
    //             match direction {
    //                 0 => {
    //                     i += 1;
    //                     j += 1;
    //                 }
    //                 1 => {
    //                     i += 1;
    //                     j -= 1;
    //                 }
    //                 2 => {
    //                     i -= 1;
    //                     j += 1;
    //                 }
    //                 3 => {
    //                     i -= 1;
    //                     j -= 1;
    //                 }
    //                 _ => (),
    //             }
    //             if i < 0 || i > 7 || j < 0 || j > 7 {
    //                 break;
    //             }
    //             let field = self._position[i as usize][j as usize];
    //             if !self.is_legal_move([row as i8, col as i8], [i, j], piece) && field != ' ' {
    //                 break;
    //             }
    //             if self.is_legal_move([row as i8, col as i8], [i, j], piece) {
    //                 if field != ' ' {
    //                     found = true;
    //                 }
    //                 self.create_position([row as i8, col as i8], [i, j], piece, [-1, -1])
    //             }
    //         }
    //     }
    // }

    // fn generate_queen_moves(&mut self, row: usize, col: usize, piece: char) {
    //     for direction in 0i8..=3 {
    //         let mut i = row as i8;
    //         let mut j = col as i8;
    //         let mut found = false;
    //         loop {
    //             if found {
    //                 break;
    //             }
    //             match direction {
    //                 0 => i += 1,
    //                 1 => i -= 1,
    //                 2 => j += 1,
    //                 3 => j -= 1,
    //                 _ => (),
    //             }
    //             if i < 0 || i > 7 || j < 0 || j > 7 {
    //                 break;
    //             }
    //             let field = self._position[i as usize][j as usize];
    //             if !self.is_legal_move([row as i8, col as i8], [i, j], piece) && field != ' ' {
    //                 break;
    //             }
    //             if self.is_legal_move([row as i8, col as i8], [i, j], piece) {
    //                 if field != ' ' {
    //                     found = true;
    //                 }
    //                 self.create_position([row as i8, col as i8], [i, j], piece, [-1, -1])
    //             }
    //         }
    //     }
    //     for direction in 0i8..=3 {
    //         let mut i = row as i8;
    //         let mut j = col as i8;
    //         let mut found = false;
    //         loop {
    //             if found {
    //                 break;
    //             }
    //             match direction {
    //                 0 => {
    //                     i += 1;
    //                     j += 1;
    //                 }
    //                 1 => {
    //                     i += 1;
    //                     j -= 1;
    //                 }
    //                 2 => {
    //                     i -= 1;
    //                     j += 1;
    //                 }
    //                 3 => {
    //                     i -= 1;
    //                     j -= 1;
    //                 }
    //                 _ => (),
    //             }
    //             if i < 0 || i > 7 || j < 0 || j > 7 {
    //                 break;
    //             }
    //             let field = self._position[i as usize][j as usize];
    //             if !self.is_legal_move([row as i8, col as i8], [i, j], piece) && field != ' ' {
    //                 break;
    //             }
    //             if self.is_legal_move([row as i8, col as i8], [i, j], piece) {
    //                 if field != ' ' {
    //                     found = true;
    //                 }
    //                 self.create_position([row as i8, col as i8], [i, j], piece, [-1, -1])
    //             }
    //         }
    //     }
    // }

    fn generate_king_moves(&mut self, row: usize, col: usize, piece: char) {
        for i in -1i8..=1 {
            for j in -1i8..=1 {
                if self.is_legal_move(
                    [row as i8, col as i8],
                    [row as i8 + i, col as i8 + j],
                    piece,
                ) {
                    self.create_position(
                        [row as i8, col as i8],
                        [row as i8 + i, col as i8 + j],
                        piece,
                    )
                }
            }
        }
    }

    fn is_legal_move(&mut self, old: [i8; 2], new: [i8; 2], piece: char) -> bool {
        if !self.is_on_board(new[0], new[1]) {
            return false;
        }
        if self._position[new[0] as usize][new[1] as usize] != ' ' && !self.is_oponent(piece, new) {
            return false;
        }
        let mut position = self._position.clone();
        position[new[0] as usize][new[1] as usize] = piece;
        position[old[0] as usize][old[1] as usize] = ' ';
        if self.king_in_check(piece, position) {
            return false;
        }
        true
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

        // check for pawn checks
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
        } else {
            // Check for pawn checks
            if king_pos.0 < 7 && king_pos.1 > 0 {
                if board[king_pos.0 + 1][king_pos.1 - 1] == 'P' {
                    return true;
                }
            }
            if king_pos.0 < 7 && king_pos.1 < 7 {
                if board[king_pos.0 + 1][king_pos.1 + 1] == 'P' {
                    return true;
                }
            }
        }

        // Check for knight checks
        for i in -2i8..=2 {
            for j in -2i8..=2 {
                if (i == 2 || i == -2) && (j == 1 || j == -1) {
                    if self.is_on_board(king_pos.0 as i8 + i, king_pos.1 as i8 + j) {
                        let target =
                            board[(king_pos.0 as i8 + i) as usize][(king_pos.1 as i8 + j) as usize];

                        if king.is_uppercase() && target == 'n' {
                            return true;
                        } else if target == 'N' {
                            return true;
                        }
                    }
                } else if (i == 1 || i == -1) && (j == 2 || j == -2) {
                    if self.is_on_board(king_pos.0 as i8 + i, king_pos.1 as i8 + j) {
                        let target =
                            board[(king_pos.0 as i8 + i) as usize][(king_pos.1 as i8 + j) as usize];

                        if king.is_uppercase() && target == 'n' {
                            return true;
                        } else if target == 'N' {
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
                    let target = board[i as usize][j as usize];
                    if king.is_uppercase() && (target == 'b' || target == 'q') {
                        return true;
                    } else if king.is_lowercase() && (target == 'B' || target == 'Q') {
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
                    let target = board[i as usize][j as usize];
                    if king.is_uppercase() && (target == 'r' || target == 'q') {
                        return true;
                    } else if king.is_lowercase() && (target == 'R' || target == 'Q') {
                        return true;
                    } else if board[i as usize][j as usize] != ' ' {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        // Check for king checks
        for i in -1i8..=1 {
            for j in -1i8..=1 {
                let row = king_pos.0 as i8 + i;
                let col = king_pos.1 as i8 + j;
                if self.is_on_board(row, col) {
                    let target =
                        board[(king_pos.0 as i8 + i) as usize][(king_pos.1 as i8 + j) as usize];

                    if king.is_uppercase() && target == 'k' {
                        return true;
                    } else if king.is_lowercase() && target == 'K' {
                        return true;
                    }
                }
            }
        }
        // no checks
        false
    }

    fn is_on_board(&mut self, row: i8, col: i8) -> bool {
        row >= 0 && row < 8 && col >= 0 && col < 8
    }

    fn is_oponent(&mut self, piece: char, new: [i8; 2]) -> bool {
        if piece.is_uppercase() {
            return self._position[new[0] as usize][new[1] as usize].is_lowercase();
        } else {
            return self._position[new[0] as usize][new[1] as usize].is_uppercase();
        }
    }

    fn create_position(&mut self, old: [i8; 2], new: [i8; 2], piece: char) {
        let mut new_position = self._position;
        self.create_en_passant(old, new, piece, &mut new_position);
        self.capture_en_passant(old, new, piece, &mut new_position);
        new_position[old[0] as usize][old[1] as usize] = ' ';
        if self.can_promote(new, piece) {
            let pieces = self.promote(piece);
            for piece in pieces {
                new_position[new[0] as usize][new[1] as usize] = piece;
                self._found_positions.push(FoundPosition {
                    _position: new_position,
                    _en_passant: [-1, -1],
                    _castling: [false, false, false, false],
                });
            }
        } else {
            new_position[new[0] as usize][new[1] as usize] = piece;
            self._found_positions.push(FoundPosition {
                _position: new_position,
                _en_passant: [-1, -1],
                _castling: [false, false, false, false],
            });
        }
    }

    fn create_en_passant(
        &mut self,
        old: [i8; 2],
        new: [i8; 2],
        piece: char,
        new_position: &mut [[char; 8]; 8],
    ) {
        if piece == 'P' {
            if old[0] == 6 && new[0] == 4 {
                new_position[5][old[1] as usize] = 'X';
            }
        } else if piece == 'p' {
            if old[0] == 1 && new[0] == 3 {
                new_position[2][old[1] as usize] = 'x';
            }
        }
    }

    fn capture_en_passant(
        &mut self,
        old: [i8; 2],
        new: [i8; 2],
        piece: char,
        new_position: &mut [[char; 8]; 8],
    ) {
        if piece == 'P' {
            if new[1] - old[1] != 0 && new_position[new[0] as usize][new[1] as usize] == ' ' {
                new_position[old[0] as usize][new[1] as usize] = ' ';
            }
        } else if piece == 'p' {
            if new[1] - old[1] != 0 && new_position[new[0] as usize][new[1] as usize] == ' ' {
                new_position[old[0] as usize][new[1] as usize] = ' ';
            }
        }
    }

    fn can_promote(&mut self, new: [i8; 2], piece: char) -> bool {
        if piece == 'P' && new[0] == 0 {
            return true;
        } else if piece == 'p' && new[0] == 7 {
            return true;
        }
        false
    }

    fn promote(&mut self, piece: char) -> [char; 4] {
        if piece == 'P' {
            return ['Q', 'R', 'B', 'N'];
        } else if piece == 'p' {
            return ['q', 'r', 'b', 'n'];
        }
        [' ', ' ', ' ', ' ']
    }
}

pub struct FoundPosition {
    pub _position: [[char; 8]; 8],
    _en_passant: [i8; 2],
    _castling: [bool; 4],
}
