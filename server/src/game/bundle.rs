use bevy_ecs::prelude::*;

use super::component;

#[derive(Bundle)]
pub struct Player {
    pub renderable: component::Renderable,
    pub position: component::Position,
    pub ordering: component::Ordering,
    pub obstacle: component::Obstacle,
    pub sight: component::Sight,
    pub memory: component::Memory,
    pub player: component::Player,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            renderable: component::Renderable::Human,
            position: Default::default(),
            ordering: Default::default(),
            obstacle: Default::default(),
            sight: component::Sight {
                kind: component::SightKind::Eyes,
                seeing: Default::default(),
            },
            memory: Default::default(),
            player: component::Player,
        }
    }
}

#[derive(Bundle, Default)]
pub struct NPC {
    pub renderable: component::Renderable,
    pub position: component::Position,
    pub ordering: component::Ordering,
    pub obstacle: component::Obstacle,
    pub sight: component::Sight,
    pub memory: component::Memory,
    pub ai: component::AI,
}

#[derive(Bundle)]
pub struct Floor {
    pub renderable: component::Renderable,
    pub position: component::Position,
    pub ordering: component::Ordering,
}

impl Default for Floor {
    fn default() -> Self {
        Self {
            renderable: component::Renderable::Floor,
            position: Default::default(),
            ordering: component::Ordering::Floor,
        }
    }
}

#[derive(Bundle)]
pub struct Wall {
    pub wall: component::Wall,
    pub room: component::Room,
    pub renderable: component::Renderable,
    pub position: component::Position,
    pub ordering: component::Ordering,
    pub obstacle: component::Obstacle,
    pub opaque: component::Opaque,
}

impl Default for Wall {
    fn default() -> Self {
        Self {
            wall: component::Wall,
            room: component::Room,
            renderable: component::Renderable::Wall,
            position: Default::default(),
            ordering: component::Ordering::Wall,
            obstacle: Default::default(),
            opaque: Default::default(),
        }
    }
}

#[derive(Bundle)]
pub struct Door {
    pub door: component::Door,
    pub room: component::Room,
    pub renderable: component::Renderable,
    pub position: component::Position,
    pub ordering: component::Ordering,
    pub obstacle: component::Obstacle,
    pub opaque: component::Opaque,
}

impl Default for Door {
    fn default() -> Self {
        Self {
            door: Default::default(),
            room: component::Room,
            renderable: component::Renderable::Door,
            position: Default::default(),
            ordering: component::Ordering::Door,
            obstacle: Default::default(),
            opaque: Default::default(),
        }
    }
}
