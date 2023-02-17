pub struct Threads {
    _thread: String,
}

impl Threads {
    pub fn new() -> Threads {
        Threads {
            _thread: "Thread".to_string(),
        }
    }

    pub fn init(&mut self) {
        println!("Thread: {}", self._thread);
    }

    pub fn start(&mut self) {

    }
}

// Path: src/uci.rs
