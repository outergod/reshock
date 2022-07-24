use bevy::prelude::*;

use crate::{
    component::{self, Obstacle, Opaque, Ordering, Position, Renderable, Sight, Visible},
    resource::ReshockFont,
};

#[derive(Bundle)]
pub struct Player {
    renderable: Renderable,
    position: Position,
    ordering: Ordering,
    obstacle: Obstacle,
    sight: Sight,
    visible: Visible,
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
            sight: Sight::Eyes,
            visible: Visible(true),
        }
    }
}

#[derive(Bundle)]
pub struct Floor {
    renderable: Renderable,
    position: Position,
    ordering: Ordering,
    visible: Visible,
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
            visible: Default::default(),
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
    visible: Visible,
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
            visible: Default::default(),
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
