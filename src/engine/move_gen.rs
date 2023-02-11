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
        for i in 0i8..8 {
            for j in 0i8..8 {
                if self._position[i as usize][j as usize].is_uppercase() == (color == 'w') {
                    self.generate_piece_moves(i, j);
                } else if self._position[i as usize][j as usize].is_lowercase() == (color == 'b') {
                    self.generate_piece_moves(i, j);
                }
            }
        }
    }

    fn generate_piece_moves(&mut self, i: i8, j: i8) {
        match self._position[i as usize][j as usize]
            .to_lowercase()
            .next()
            .unwrap()
        {
            'p' => self.generate_pawn_moves(7 - i, j, self._position[i as usize][j as usize]),
            'r' => self.generate_rook_moves(7 - i, j, self._position[i as usize][j as usize]),
            // 'N' => self.generate_knight_moves(i, j),
            // 'B' => self.generate_bishop_moves(i, j),
            // 'Q' => self.generate_queen_moves(i, j),
            'k' => self.generate_king_moves(7 - i, j, self._position[i as usize][j as usize]),
            _ => (),
        }
    }

    fn generate_king_moves(&mut self, row: i8, col: i8, piece: char) {
        for i in -1i8..=1 {
            for j in -1i8..=1 {
                if self.is_on_board(row + i, col + j) && (row + i != row || col + j != col) {
                    if (!self.is_empty(row + i, col + j)
                        && self.is_oponent(row + i, col + j, piece))
                        || self.is_empty(row + i, col + j)
                    {
                        self.create_position([row, col], [row + i, col + j]);
                    }
                }
            }
        }
    }

    fn generate_pawn_moves(&mut self, row: i8, col: i8, color: char) {
        if color == 'P' {
            if row == 1 && self.is_empty(row + 1, col) && self.is_empty(row + 2, col) {
                self.create_position([row, col], [row + 2, col]);
            }
            if self.is_empty(row + 1, col) && row != 6 {
                self.create_position([row, col], [row + 1, col]);
            }
        } else {
            if row == 6 && self.is_empty(row - 1, col) && self.is_empty(row - 2, col) {
                self.create_position([row, col], [row - 2, col]);
            }
            if self.is_empty(row - 1, col) && row != 1 {
                self.create_position([row, col], [row - 1, col]);
            }
        }
    }

    fn generate_rook_moves(&mut self, row: i8, col: i8, color: char) {
        for direction in 0i8..=3 {
            let mut i = row;
            let mut j = col;
            let mut found = false;
            loop {
                if found {
                    break;
                }
                match direction {
                    0 => i += 1,
                    1 => i -= 1,
                    2 => j += 1,
                    3 => j -= 1,
                    _ => (),
                }
                if !self.is_on_board(i, j) {
                    break;
                }
                if !self.is_empty(i, j) && !self.is_oponent(i, j, color) {
                    break;
                }
                if !self.is_empty(i, j) && self.is_oponent(i, j, color) {
                    found = true;
                }
                self.create_position([row, col], [i, j]);
            }
        }
    }

    fn is_on_board(&mut self, row: i8, col: i8) -> bool {
        row >= 0 && row < 8 && col >= 0 && col < 8
    }

    fn is_empty(&mut self, row: i8, col: i8) -> bool {
        self._position[7 - row as usize][col as usize] == ' '
    }

    fn is_oponent(&mut self, row: i8, col: i8, color: char) -> bool {
        let piece = self._position[7 - row as usize][col as usize];
        if color.is_uppercase() {
            piece.is_lowercase()
        } else {
            piece.is_uppercase()
        }
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
