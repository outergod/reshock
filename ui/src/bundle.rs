use bevy::prelude::*;

use crate::{
    component::{
        self, Memory, Obstacle, Opaque, Ordering, Position, Renderable, Room, Sight, SightKind,
    },
    resource::ReshockFont,
};

#[derive(Bundle)]
pub struct Player {
    pub renderable: Renderable,
    pub position: Position,
    pub ordering: Ordering,
    pub obstacle: Obstacle,
    pub sight: Sight,
    pub memory: Memory,
    pub player: component::Player,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            renderable: Renderable {
                char: '@',
                color: Color::WHITE,
            },
            position: Default::default(),
            ordering: Default::default(),
            obstacle: Default::default(),
            sight: Sight {
                kind: SightKind::Eyes,
                seeing: Default::default(),
            },
            memory: Memory {
                color: Color::DARK_GRAY,
                ..Default::default()
            },
            player: component::Player,
        }
    }
}

#[derive(Bundle)]
pub struct NPC {
    pub renderable: Renderable,
    pub position: Position,
    pub ordering: Ordering,
    pub obstacle: Obstacle,
    pub sight: Sight,
    pub memory: Memory,
    pub ai: component::AI,
}

impl Default for NPC {
    fn default() -> Self {
        Self {
            renderable: Default::default(),
            position: Default::default(),
            ordering: Default::default(),
            obstacle: Default::default(),
            sight: Default::default(),
            memory: Default::default(),
            ai: Default::default(),
        }
    }
}

#[derive(Bundle)]
pub struct Floor {
    pub renderable: Renderable,
    pub position: Position,
    pub ordering: Ordering,
}

impl Default for Floor {
    fn default() -> Self {
        Self {
            renderable: Renderable {
                char: '·',
                color: Color::ALICE_BLUE,
            },
            position: Default::default(),
            ordering: Ordering::Floor,
        }
    }
}

#[derive(Bundle)]
pub struct Wall {
    pub wall: component::Wall,
    pub room: Room,
    pub renderable: Renderable,
    pub position: Position,
    pub ordering: Ordering,
    pub obstacle: Obstacle,
    pub opaque: Opaque,
}

impl Default for Wall {
    fn default() -> Self {
        Self {
            wall: component::Wall,
            room: Room,
            renderable: Default::default(),
            position: Default::default(),
            ordering: Ordering::Wall,
            obstacle: Default::default(),
            opaque: Default::default(),
        }
    }
}

#[derive(Bundle)]
pub struct Door {
    pub door: component::Door,
    pub room: Room,
    pub renderable: Renderable,
    pub position: Position,
    pub ordering: Ordering,
    pub obstacle: Obstacle,
    pub opaque: Opaque,
}

impl Default for Door {
    fn default() -> Self {
        Self {
            door: Default::default(),
            room: Room,
            renderable: Default::default(),
            position: Default::default(),
            ordering: Ordering::Door,
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
