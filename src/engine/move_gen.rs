pub struct MoveGen {
    _position: [[char; 8]; 8],
    _col_names: [char; 8],
    _en_passant: String,
    pub _found_positions: Vec<[[char; 8]; 8]>,
    pub _found_moves: Vec<String>,
    pub _own_en_passant: String,
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
            _own_en_passant: String::from("-"),
        }
    }

    pub fn set_position(&mut self, position: [[char; 8]; 8]) {
        self._position = position;
    }

    pub fn set_en_passant(&mut self, en_passant: String) {
        self._en_passant = en_passant;
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
            'R' => self.generate_rook_moves(row, col, piece),
            'N' => self.generate_knight_moves(row, col, piece),
            'B' => self.generate_bishop_moves(row, col, piece),
            'Q' => self.generate_queen_moves(row, col, piece),
            'K' => self.generate_king_moves(row, col, piece),
            'p' => self.generate_pawn_moves(row, col, piece),
            'r' => self.generate_rook_moves(row, col, piece),
            'n' => self.generate_knight_moves(row, col, piece),
            'b' => self.generate_bishop_moves(row, col, piece),
            'q' => self.generate_queen_moves(row, col, piece),
            'k' => self.generate_king_moves(row, col, piece),
            _ => (),
        }
    }

    fn generate_pawn_moves(&mut self, row: usize, col: usize, piece: char) {
        if piece == 'P' {
            if row == 6
                && self._position[row - 1][col] == ' '
                && self.is_legal_move([row as i8, col as i8], [row as i8 - 2, col as i8], piece)
            {
                self.create_position(
                    [row as i8, col as i8],
                    [row as i8 - 2, col as i8],
                    piece,
                    [-1, -1],
                );
                self._own_en_passant = format!("{}{}", self._col_names[col], 3);
                self._own_en_passant = format!("{}{}", (col as u8 + 97) as char, row as u8 - 1);
            }

            if self.is_legal_move([row as i8, col as i8], [row as i8 - 1, col as i8], piece) {
                self.create_position(
                    [row as i8, col as i8],
                    [row as i8 - 1, col as i8],
                    piece,
                    [-1, -1],
                )
            }

            if col > 0
                && !(self._position[row - 1][col - 1] == ' ')
                && self.is_legal_move(
                    [row as i8, col as i8],
                    [row as i8 - 1, col as i8 - 1],
                    piece,
                )
            {
                self.create_position(
                    [row as i8, col as i8],
                    [row as i8 - 1, col as i8 - 1],
                    piece,
                    [-1, -1],
                )
            }

            if col < 7
                && !(self._position[row - 1][col + 1] == ' ')
                && self.is_legal_move(
                    [row as i8, col as i8],
                    [row as i8 - 1, col as i8 + 1],
                    piece,
                )
            {
                self.create_position(
                    [row as i8, col as i8],
                    [row as i8 - 1, col as i8 + 1],
                    piece,
                    [-1, -1],
                )
            }

            // en passant
            if self._en_passant != "-" && row == 3 {
                if col < 7 && self._en_passant == format!("{}{}", self._col_names[(col + 1)], 6) {
                    self.create_position(
                        [row as i8, col as i8],
                        [row as i8 - 1, col as i8 + 1],
                        piece,
                        [3, (col + 1) as i8],
                    )
                }
                if col > 0 && self._en_passant == format!("{}{}", self._col_names[(col - 1)], 6) {
                    self.create_position(
                        [row as i8, col as i8],
                        [row as i8 - 1, col as i8 - 1],
                        piece,
                        [3, (col - 1) as i8],
                    )
                }
            }
        }
    }

    fn generate_rook_moves(&mut self, row: usize, col: usize, piece: char) {
        for direction in 0i8..=3 {
            let mut i = row as i8;
            let mut j = col as i8;
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
                if i < 0 || i > 7 || j < 0 || j > 7 {
                    break;
                }
                let field = self._position[i as usize][j as usize];
                if !self.is_legal_move([row as i8, col as i8], [i, j], piece) && field != ' ' {
                    break;
                }
                if self.is_legal_move([row as i8, col as i8], [i, j], piece) {
                    if field != ' ' {
                        found = true;
                    }
                    self.create_position([row as i8, col as i8], [i, j], piece, [-1, -1])
                }
            }
        }
    }

    fn generate_knight_moves(&mut self, row: usize, col: usize, piece: char) {
        for i in -2i8..=2 {
            for j in -2i8..=2 {
                if (i == 2 || i == -2) && (j == 1 || j == -1) {
                    if self.is_legal_move(
                        [row as i8, col as i8],
                        [row as i8 + i, col as i8 + j],
                        piece,
                    ) {
                        self.create_position(
                            [row as i8, col as i8],
                            [row as i8 + i, col as i8 + j],
                            piece,
                            [-1, -1],
                        )
                    }
                } else if (i == 1 || i == -1) && (j == 2 || j == -2) {
                    if self.is_legal_move(
                        [row as i8, col as i8],
                        [row as i8 + i, col as i8 + j],
                        piece,
                    ) {
                        self.create_position(
                            [row as i8, col as i8],
                            [row as i8 + i, col as i8 + j],
                            piece,
                            [-1, -1],
                        )
                    }
                }
            }
        }
    }

    fn generate_bishop_moves(&mut self, row: usize, col: usize, piece: char) {
        for direction in 0i8..=3 {
            let mut i = row as i8;
            let mut j = col as i8;
            let mut found = false;
            loop {
                if found {
                    break;
                }
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
                if i < 0 || i > 7 || j < 0 || j > 7 {
                    break;
                }
                let field = self._position[i as usize][j as usize];
                if !self.is_legal_move([row as i8, col as i8], [i, j], piece) && field != ' ' {
                    break;
                }
                if self.is_legal_move([row as i8, col as i8], [i, j], piece) {
                    if field != ' ' {
                        found = true;
                    }
                    self.create_position([row as i8, col as i8], [i, j], piece, [-1, -1])
                }
            }
        }
    }

    fn generate_queen_moves(&mut self, row: usize, col: usize, piece: char) {
        for direction in 0i8..=3 {
            let mut i = row as i8;
            let mut j = col as i8;
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
                if i < 0 || i > 7 || j < 0 || j > 7 {
                    break;
                }
                let field = self._position[i as usize][j as usize];
                if !self.is_legal_move([row as i8, col as i8], [i, j], piece) && field != ' ' {
                    break;
                }
                if self.is_legal_move([row as i8, col as i8], [i, j], piece) {
                    if field != ' ' {
                        found = true;
                    }
                    self.create_position([row as i8, col as i8], [i, j], piece, [-1, -1])
                }
            }
        }
        for direction in 0i8..=3 {
            let mut i = row as i8;
            let mut j = col as i8;
            let mut found = false;
            loop {
                if found {
                    break;
                }
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
                if i < 0 || i > 7 || j < 0 || j > 7 {
                    break;
                }
                let field = self._position[i as usize][j as usize];
                if !self.is_legal_move([row as i8, col as i8], [i, j], piece) && field != ' ' {
                    break;
                }
                if self.is_legal_move([row as i8, col as i8], [i, j], piece) {
                    if field != ' ' {
                        found = true;
                    }
                    self.create_position([row as i8, col as i8], [i, j], piece, [-1, -1])
                }
            }
        }
    }

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
                        [-1, -1],
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

    fn create_position(&mut self, old: [i8; 2], new: [i8; 2], piece: char, en_passant: [i8; 2]) {
        if en_passant[0] != -1 {
            self._position[en_passant[0] as usize][en_passant[1] as usize] = ' ';
        }
        let mut new_position = self._position;
        new_position[old[0] as usize][old[1] as usize] = ' ';
        new_position[new[0] as usize][new[1] as usize] = piece;
        self._found_positions.push(new_position);
    }
}
