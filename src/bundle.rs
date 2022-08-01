use bevy::prelude::*;

use crate::{
    component::{
        self, Memory, Obstacle, Opaque, Ordering, Position, Renderable, Room, Sight, SightKind,
    },
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
            obstacle: Default::default(),
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
    room: Room,
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
            room: Room,
            renderable: Default::default(),
            position,
            ordering: Ordering(4),
            obstacle: Default::default(),
            opaque: Default::default(),
        }
    }
}

#[derive(Bundle)]
pub struct Door {
    door: component::Door,
    room: Room,
    renderable: Renderable,
    position: Position,
    ordering: Ordering,
    obstacle: Obstacle,
    opaque: Opaque,
}

impl Door {
    pub fn new(position: Position, open: bool) -> Self {
        let color = if open { Color::DARK_GRAY } else { Color::WHITE };

        Self {
            door: component::Door {
                open,
                toggle: false,
            },
            room: Room,
            renderable: Renderable { char: ' ', color },
            position,
            ordering: Ordering(4),
            obstacle: Default::default(),
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
                text: Text::from_section(
                    " ".to_string(),
                    TextStyle {
                        font: font.handle.clone_weak(),
                        font_size: font.size,
                        color: Color::WHITE,
                    },
                ),
                ..Default::default()
            },
        }
    }
}
