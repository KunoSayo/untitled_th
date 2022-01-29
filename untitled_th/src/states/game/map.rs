use std::fs::File;

use specs::{Entity, World};

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
        self.blocks.get(x).and_then(|c| c.get(y))
    }
}

impl FixedMap {
    pub fn new(blocks: Vec<Vec<Entity>>) -> Self {
        Self {
            blocks
        }
    }
}

impl TryFrom<Vec<u8>> for FixedMap {
    type Error = &'static str;
    /// The format is temporarily
    /// Start bytes with uth
    ///
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {}
}