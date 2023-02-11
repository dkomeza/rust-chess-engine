use crate::engine::Engine;

pub struct Uci {
    pub engine: Engine,
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
        println!("id name {}", self.engine._name);
        println!("id author {}", self.engine._author);
        println!("id version {}", self.engine._version);
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
