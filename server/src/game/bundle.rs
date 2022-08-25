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
    pub description: component::Description,
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
                ..Default::default()
            },
            memory: Default::default(),
            player: component::Player,
            description: component::Description {
                name: "the Hacker".to_string(),
                article: component::Article::None,
            },
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
    pub description: component::Description,
}

#[derive(Bundle)]
pub struct Floor {
    pub renderable: component::Renderable,
    pub position: component::Position,
    pub ordering: component::Ordering,
    pub description: component::Description,
}

impl Default for Floor {
    fn default() -> Self {
        Self {
            renderable: component::Renderable::Floor,
            position: Default::default(),
            ordering: component::Ordering::Floor,
            description: component::Description {
                name: "flooring".to_string(),
                article: component::Article::None,
            },
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
    pub wall: component::Wall,
    pub description: component::Description,
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
            wall: Default::default(),
            description: component::Description {
                name: "wall".to_string(),
                article: component::Article::A,
            },
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
    pub description: component::Description,
}

impl Default for Door {
    fn default() -> Self {
        Self {
            door: Default::default(),
            room: component::Room,
            renderable: component::Renderable::Door,
            position: Default::default(),
            ordering: component::Ordering::Door,
            description: component::Description {
                name: "door".to_string(),
                article: component::Article::A,
            },
        }
    }
}
