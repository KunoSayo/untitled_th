use specs::{Component, FlaggedStorage, VecStorage};

use uth_map::bound::*;

#[derive(Debug, Copy, Clone)]
pub struct Block {
    pass_bounding: Bounding,
    bullet_bounding: Bounding,
}

/// The entity is fixed and cannot be move in any way (Such as background..?)
pub struct Fixed;

/// The entity can be push in normal situation
pub struct Movable;

/// The item can place in
pub struct Placeable;

impl Component for Block {
    type Storage = VecStorage<Block>;
}

macro_rules! impl_flag {
    ($($e: ty), *) => {
        $(impl Component for $e { type Storage = FlaggedStorage<$e>; })*
    };
}

impl_flag!(Movable, Placeable, Fixed);