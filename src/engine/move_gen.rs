pub struct MoveGen {
    _position: [[char; 8]; 8],
    _col_names: [char; 8],
    _found_moves: i128,
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
        }
    }

    pub fn set_position(&mut self, position: [[char; 8]; 8]) {
        self._position = position;
        self._found_moves = 0;
    }

    pub fn generate_moves(&mut self) {
        for i in 0i8..8 {
            for j in 0i8..8 {
                match self._position[i as usize][j as usize]
                    .to_lowercase()
                    .next()
                    .unwrap()
                {
                    'p' => {
                        self.generate_pawn_moves(7 - i, j, self._position[i as usize][j as usize])
                    }
                    // 'R' => self.generate_rook_moves(i, j),
                    // 'N' => self.generate_knight_moves(i, j),
                    // 'B' => self.generate_bishop_moves(i, j),
                    // 'Q' => self.generate_queen_moves(i, j),
                    'k' => {
                        self.generate_king_moves(7 - i, j, self._position[i as usize][j as usize])
                    }
                    _ => (),
                }
            }
        }
    }

    fn generate_king_moves(&mut self, row: i8, col: i8, color: char) {
        for i in -1i8..=1 {
            for j in -1i8..=1 {
                if self.is_on_board(row + i, col + j)
                    && self.is_empty(row + i, col + j)
                    && (row + i != row || col + j != col)
                {
                    self._found_moves += 1;
                    println!(
                        "{}{}{}, found moves: {}",
                        color,
                        self._col_names[(col + j) as usize],
                        row + i + 1,
                        self._found_moves
                    );
                }
            }
        }
    }

    fn generate_pawn_moves(&mut self, row: i8, col: i8, color: char) {
        if color == 'P' {
            if row == 1 && self.is_empty(row + 1, col) && self.is_empty(row + 2, col) {
                self._found_moves += 1;
                println!(
                    "{}{}{}, found moves: {}",
                    color,
                    self._col_names[col as usize],
                    row + 3,
                    self._found_moves
                );
            }
            if self.is_empty(row + 1, col) && row != 6 {
                self._found_moves += 1;
                println!(
                    "{}{}{}, found moves: {}",
                    color,
                    self._col_names[col as usize],
                    row + 2,
                    self._found_moves
                );
            }
        } else {
            if row == 6 && self.is_empty(row - 1, col) && self.is_empty(row - 2, col) {
                self._found_moves += 1;
                println!(
                    "{}{}{}, found moves: {}",
                    color, self._col_names[col as usize], row, self._found_moves
                );
            }
            if self.is_empty(row - 1, col) && row != 1 {
                self._found_moves += 1;
                println!(
                    "{}{}{}, found moves: {}",
                    color, self._col_names[col as usize], row, self._found_moves
                );
            }
        }
    }

    fn is_on_board(&self, row: i8, col: i8) -> bool {
        row >= 0 && row < 8 && col >= 0 && col < 8
    }

    fn is_empty(&self, row: i8, col: i8) -> bool {
        self._position[7 - row as usize][col as usize] == ' '
    }
}
