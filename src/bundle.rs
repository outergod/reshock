use bevy::prelude::*;

use crate::{
    component::{self, Memory, Obstacle, Opaque, Ordering, Position, Renderable, Sight, SightKind},
    resource::ReshockFont,
};

#[derive(Bundle)]
pub struct Player {
    renderable: Renderable,
    position: Position,
    ordering: Ordering,
    obstacle: Obstacle,
    sight: Sight,
    memory: Memory,
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
            sight: Sight {
                kind: SightKind::Eyes,
                seeing: Default::default(),
            },
            memory: Default::default(),
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
            ordering: Ordering(u8::MAX - 1),
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
    opaque: Opaque,
}

impl Wall {
    pub fn new(position: Position) -> Self {
        Self {
            wall: component::Wall,
            renderable: Default::default(),
            position,
            ordering: Ordering(4),
            obstacle: Obstacle::Always,
            opaque: Default::default(),
        }
    }
}

#[derive(Bundle)]
pub struct Tile {
    position: Position,

    #[bundle]
    text: Text2dBundle,
}

impl Tile {
    pub fn new(position: Position, font: &Res<ReshockFont>) -> Self {
        Self {
            position,
            text: Text2dBundle {
                text: Text::with_section(
                    " ".to_string(),
                    TextStyle {
                        font: font.handle.clone_weak(),
                        font_size: font.size,
                        color: Color::WHITE,
                    },
                    TextAlignment::default(),
                ),
                ..Default::default()
            },
        }
    }
}
