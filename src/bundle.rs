use bevy::prelude::*;

use crate::component::{self, Obstacle, Ordering, Position, Renderable};

#[derive(Bundle)]
pub struct Player {
    renderable: Renderable,
    position: Position,
    ordering: Ordering,
    obstacle: Obstacle,
    player: component::Player,
}

impl Player {
    pub fn new(position: Position) -> Self {
        Self {
            renderable: Renderable::Human,
            position,
            ordering: Ordering(u8::MIN),
            player: component::Player,
            obstacle: Obstacle::Always,
        }
    }
}

#[derive(Bundle)]
pub struct Floor {
    renderable: Renderable,
    position: Position,
    ordering: Ordering,
}

impl Floor {
    pub fn new(position: Position) -> Self {
        Self {
            renderable: Renderable::Floor,
            position,
            ordering: Ordering(u8::MAX),
        }
    }
}

#[derive(Bundle)]
pub struct Wall {
    renderable: Renderable,
    position: Position,
    ordering: Ordering,
    obstacle: Obstacle,
}

impl Wall {
    pub fn new(position: Position) -> Self {
        Self {
            renderable: Renderable::Wall,
            position,
            ordering: Ordering(4),
            obstacle: Obstacle::Always,
        }
    }
}
