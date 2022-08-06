use bevy_ecs::prelude::*;

use super::component;

#[derive(Bundle)]
pub struct Player {
    pub renderable: component::Renderable,
    pub position: component::Position,
    pub ordering: component::Ordering,
    pub solid: component::Solid,
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
            solid: Default::default(),
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
    pub solid: component::Solid,
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
    pub room: component::Room,
    pub renderable: component::Renderable,
    pub position: component::Position,
    pub ordering: component::Ordering,
    pub solid: component::Solid,
    pub opaque: component::Opaque,
}

impl Default for Wall {
    fn default() -> Self {
        Self {
            room: component::Room,
            renderable: component::Renderable::Wall,
            position: Default::default(),
            ordering: component::Ordering::Wall,
            solid: Default::default(),
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
}

impl Default for Door {
    fn default() -> Self {
        Self {
            door: Default::default(),
            room: component::Room,
            renderable: component::Renderable::Door,
            position: Default::default(),
            ordering: component::Ordering::Door,
        }
    }
}
