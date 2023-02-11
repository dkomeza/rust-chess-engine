pub struct Uci {
    pub _name: &'static str,
    pub _author: &'static str,
    pub _version: &'static str,
}

impl Uci {
    pub fn parse_command(&mut self, command: String) {
        let mut command = command.split_whitespace();
        match command.next() {
            Some("uci") => self.uci(),
            Some("isready") => self.isready(),
            Some("ucinewgame") => self.ucinewgame(),
            Some("position") => self.position(command),
            Some("go") => self.go(command),
            Some("stop") => self.stop(),
            Some("ponderhit") => self.ponderhit(),
            Some("quit") => self.quit(),
            _ => println!("Unknown command"),
        }
    }
    fn uci(&mut self) {
        println!("id name {}", self._name);
        println!("id author {}", self._author);
        println!("id version {}", self._version);
        println!("uciok");
    }

    fn isready(&mut self) {
        println!("readyok");
    }

    fn ucinewgame(&mut self) {
        println!("ucinewgame");
    }

    fn position(&mut self, command: std::str::SplitWhitespace) {
        println!("position");
    }

    fn go(&mut self, command: std::str::SplitWhitespace) {
        println!("go");
    }

    fn stop(&mut self) {
        println!("stop");
    }

    fn ponderhit(&mut self) {
        println!("ponderhit");
    }

    fn quit(&mut self) {
        println!("quit");
    }
}
