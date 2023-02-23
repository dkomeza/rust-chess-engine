use crate::{types::*, POSITION};

use self::move_gen::MoveGen;
pub mod move_gen;

pub struct Search {
    pub _limits: Limits,
    _move_gen: MoveGen,
}

impl Search {
    pub fn new() -> Search {
        Search {
            _limits: Limits::new(),
            _move_gen: MoveGen::new(),
        }
    }
    pub fn start(&mut self, limits: &Limits) {
        self._limits = limits.clone();

        if self._limits._depth > 0 {
            self._start_depth_search(self._limits._depth);
        }
    }

    fn _start_depth_search(&mut self, depth: i8) {
        let position = POSITION.lock().unwrap();
        let state_info = StateInfo {
            position: &position._board,
            castling: &position._castle_rights,
            side_to_move: &position._side_to_move,
        };
        self._move_gen.start(state_info);
        let mut current_depth = 0;
        while current_depth < depth {
            current_depth += 1;
        }
    }
}

pub struct Limits {
    pub _time: [i32; 2],
    pub _inc: [i32; 2],
    pub _npmsec: i32,
    pub _movetime: i32,
    pub _starttime: i32,
    pub _depth: i8,
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

pub struct StateInfo<'a> {
    pub position: &'a [i8; 64],
    pub castling: &'a i8,
    pub side_to_move: &'a i8,
}
