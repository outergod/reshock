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
            renderable: Renderable {
                char: '@',
                color: Color::WHITE,
            },
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
            renderable: Renderable {
                char: '·',
                color: Color::ALICE_BLUE,
            },
            position,
            ordering: Ordering(u8::MAX),
        }
    }
}

#[derive(Bundle)]
pub struct Wall {
    wall: component::Wall,
    renderable: Renderable,
    position: Position,
    ordering: Ordering,
    obstacle: Obstacle,
}

impl Wall {
    pub fn new(position: Position) -> Self {
        Self {
            wall: component::Wall,
            renderable: Default::default(),
            position,
            ordering: Ordering(4),
            obstacle: Obstacle::Always,
        }
    }
}
