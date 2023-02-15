pub struct Engine {
    pub _name: &'static str,
    pub _author: &'static str,
    pub _version: &'static str,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            _name: "RustyChess",
            _author: "dkomeza",
            _version: "0.1",
        }
    }
}

// Path: src/engine.rs
