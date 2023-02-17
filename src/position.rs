pub struct Position {

}

impl Position {
    pub fn new() -> Position {
        Position {}
    }

    pub fn set(&mut self) {
        
    }
}

pub struct StateInfo {
    pub pawn_key: u64,
    pub material_key: u64,
    pub non_pawn_material: [i32; 2],
    pub _castling_rights: u8,
    pub _rule50: i8,
    pub _plies_from_null: i8,
    pub ep_square: i8,
    pub _key: u64,
    pub _checkers: u64,
}