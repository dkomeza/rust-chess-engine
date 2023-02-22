use crate::types::*;

pub struct Search {
    pub _limits: Limits,
}

impl Search {
    pub fn new() -> Search {
        Search {
            _limits: Limits::new(),
        }
    }

    
}

pub struct Limits {
    pub _time: [i32; 2],
    pub _inc: [i32; 2],
    pub _npmsec: i32,
    pub _movetime: i32,
    pub _starttime: i32,
    pub _depth: i32,
    pub _movestogo: i32,
    pub _mate: i32,
    pub _infinite: bool,
    pub _perft: i32,
    pub _nodes: i64,
    pub _moves: Vec<Move>,
}

impl Limits {
    pub fn new() -> Limits {
        Limits {
            _time: [0, 0],
            _inc: [0, 0],
            _npmsec: 0,
            _movetime: 0,
            _starttime: 0,
            _depth: 0,
            _movestogo: 0,
            _mate: 0,
            _infinite: false,
            _perft: 0,
            _nodes: 0,
            _moves: Vec::new(),
        }
    }
    pub fn clone(&self) -> Limits {
        Limits {
            _time: self._time,
            _inc: self._inc,
            _npmsec: self._npmsec,
            _movetime: self._movetime,
            _starttime: self._starttime,
            _depth: self._depth,
            _movestogo: self._movestogo,
            _mate: self._mate,
            _infinite: self._infinite,
            _perft: self._perft,
            _nodes: self._nodes,
            _moves: self._moves.clone(),
        }
    }
}
