pub struct Board {
    pub _board: [[char; 8]; 8],
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
        }
    }

    pub fn position(&mut self, command: std::str::SplitWhitespace) {
        let mut command = command;
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
    }

    fn fen(&mut self, command: std::str::SplitWhitespace) {
        let mut command = command;
        let mut fen = String::new();
        while let Some(word) = command.next() {
            fen.push_str(word);
            fen.push(' ');
        }
        let mut fen = fen.split_whitespace();
        let mut board = String::new();
        while let Some(word) = fen.next() {
            board.push_str(word);
        }
        let mut board = board.split('/');
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

    pub fn print_board(&self) {
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
}
