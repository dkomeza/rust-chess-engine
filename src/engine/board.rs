pub struct Board {
    pub _board: [[char; 8]; 8],
    pub _turn: char,
    pub _castling: String,
    pub _en_passant: String,
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
            _turn: 'w',
            _castling: String::from("KQkq"),
            _en_passant: String::from("-"),
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
            self._castling = castling.unwrap().to_string();
        }
        let en_passant = command.next();
        if en_passant.is_some() {
            self._en_passant = en_passant.unwrap().to_string();
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
