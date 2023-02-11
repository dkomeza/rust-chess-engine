pub struct Engine {
    pub _name: &'static str,
    pub _author: &'static str,
    pub _version: &'static str,
}

impl Engine {
    pub fn new(name: &'static str, author: &'static str, version: &'static str) -> Engine {
        Engine {
            _name: name,
            _author: author,
            _version: version,
        }
    }
}

// Path: src/uci.rs