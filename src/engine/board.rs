pub struct Board {
    pub _board: [[char; 8]; 8],
    _columns: [char; 8],
    pub _turn: char,
    pub _castling: [bool; 4],
    pub _en_passant: [i8; 2],
    pub _move_history: Vec<String>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            _board: [
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            ],
            _columns: ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'],
            _turn: 'w',
            _castling: [false, false, false, false],
            _en_passant: [-1, -1],
            _move_history: Vec::new(),
        }
    }

    pub fn position(&mut self, command: std::str::SplitWhitespace) {
        let mut command = command.to_owned();
        match command.next() {
            Some("startpos") => self.startpos(),
            Some("fen") => self.fen(command),
            _ => println!("Unknown command"),
        }
    }

    fn startpos(&mut self) {
        self._board = [
            ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'],
            ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
            ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'],
        ];
        self._turn = 'w';
        self._castling = [true, true, true, true];
        self._en_passant = [-1, -1];
        // while let Some(token) = command.next() {
        //     match token {
        //         "moves" => self.moves(command),
        //         _ => println!("Unknown command"),
        //     }
        // }
    }

    fn fen(&mut self, command: std::str::SplitWhitespace) {
        let mut command = command.to_owned();
        let fen_string = command.next();
        let fen = if fen_string.is_some() {
            fen_string.unwrap().to_string()
        } else {
            String::from("")
        };
        if fen.len() == 0 {
            println!("Unknown command");
            return;
        }

        let turn = command.next();
        if turn.is_some() {
            self._turn = turn.unwrap().chars().next().unwrap();
        }

        let castling = command.next();
        if castling.is_some() {
            for c in castling.unwrap().chars() {
                match c {
                    'K' => self._castling[0] = true,
                    'Q' => self._castling[1] = true,
                    'k' => self._castling[2] = true,
                    'q' => self._castling[3] = true,
                    _ => (),
                }
            }
        }

        let mut board = fen.split('/');
        let mut row = 0;
        while let Some(word) = board.next() {
            let mut col = 0;
            for c in word.chars() {
                if c.is_digit(10) {
                    for _ in 0..c.to_digit(10).unwrap() {
                        self._board[row][col] = ' ';
                        col += 1;
                    }
                } else {
                    self._board[row][col] = c;
                    col += 1;
                }
            }
            row += 1;
        }

        // calculate en passant
        let en_passant = command.next();
        if en_passant.is_some() {
            let col = self
                ._columns
                .iter()
                .position(|&c| c == en_passant.unwrap().chars().next().unwrap())
                .unwrap();
            let row = 8 - en_passant
                .unwrap()
                .chars()
                .nth(1)
                .unwrap()
                .to_digit(10)
                .unwrap();
            if self._turn == 'w' {
                self._board[row as usize][col] = 'x';
            } else {
                self._board[row as usize][col] = 'X';
            }
        }
    }

    pub fn print_board(&mut self) {
        println!("  +---+---+---+---+---+---+---+---+");
        for row in 0..8 {
            print!("{} |", 8 - row);
            for col in 0..8 {
                print!(" {} |", self._board[row][col]);
            }
            println!();
            println!("  +---+---+---+---+---+---+---+---+");
        }
        println!("    a   b   c   d   e   f   g   h");
    }

    pub fn print_position(&mut self, position: &[[char; 8]; 8]) {
        for row in 0..8 {
            print!("{} |", 8 - row);
            for col in 0..8 {
                print!(" {} |", position[row][col]);
            }
            println!();
            println!("  +---+---+---+---+---+---+---+---+");
        }
    }
}
