use crate::data_types::Position;

pub struct Creature {
    position: Position,
}

impl Creature {
    pub fn new(position: Position) -> Creature {
        Creature { position }
    }
}
