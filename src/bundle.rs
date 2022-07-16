use bevy::prelude::*;

use crate::component::{self, Ordering, Position, Renderable};

#[derive(Bundle)]
pub struct Player {
    renderable: Renderable,
    position: Position,
    ordering: Ordering,
    player: component::Player,
}

impl Player {
    pub fn new(position: Position) -> Self {
        Self {
            renderable: Renderable::Human,
            position,
            ordering: Ordering(u8::MIN),
            player: component::Player,
        }
    }
}

#[derive(Bundle)]
pub struct Floor {
    renderable: Renderable,
    position: Position,
    ordering: Ordering,
    floor: component::Floor,
}

impl Floor {
    pub fn new(position: Position) -> Self {
        Self {
            renderable: Renderable::Floor,
            position,
            ordering: Ordering(u8::MAX),
            floor: component::Floor,
        }
    }
}

#[derive(Bundle)]
pub struct Wall {
    renderable: Renderable,
    position: Position,
    ordering: Ordering,
    wall: component::Wall,
}

impl Wall {
    pub fn new(position: Position) -> Self {
        Self {
            renderable: Renderable::Wall,
            position,
            ordering: Ordering(4),
            wall: component::Wall,
        }
    }
}
