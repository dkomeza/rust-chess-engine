#[derive(Copy)]
pub enum Move {
    NONE,
    NULL = 65,
}

impl Clone for Move {
    fn clone(&self) -> Move {
        *self
    }
}

pub enum Side {
    WHITE,
    BLACK,
}