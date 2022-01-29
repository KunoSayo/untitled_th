use specs::{Component, VecStorage, World};

use crate::{GameState, StateData};

mod map;
mod block;

pub struct Health {
    hp: i32,
}

pub struct GamePos {
    x: f32,
    y: f32,
}

impl Component for Health { type Storage = VecStorage<Health>; }

impl Component for GamePos { type Storage = VecStorage<GamePos>; }


pub struct Gaming {
    world: World,
}

impl GameState for Gaming {
    fn start(&mut self, _: &mut StateData) {}
}