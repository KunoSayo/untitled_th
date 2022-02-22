use specs::{Entity};

/// The map in the game.
/// The left top is zero point
pub trait Map {
    /// Get the block in (x, y) and return None if not present.
    fn get_block(&self, x: i32, y: i32) -> Option<&Entity>;
}


pub struct FixedMap {
    blocks: Vec<Vec<Entity>>,
}

impl Map for FixedMap {
    fn get_block(&self, x: i32, y: i32) -> Option<&Entity> {
        self.blocks.get(x as usize).and_then(|c| c.get(y as usize))
    }
}

impl FixedMap {
    pub fn new(blocks: Vec<Vec<Entity>>) -> Self {
        Self {
            blocks
        }
    }
}
