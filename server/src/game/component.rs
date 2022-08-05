use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::*;
use glam::IVec2;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Room;

#[derive(Component)]
pub struct Door {
    pub open: bool,
}

impl Default for Door {
    fn default() -> Self {
        Self { open: false }
    }
}

#[derive(Component, Clone, Debug)]
pub enum Renderable {
    None,
    Human,
    ServBot,
    Floor,
    Wall,
    Door,
}

impl Default for Renderable {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Component)]
pub struct Obstacle(pub bool);

impl Default for Obstacle {
    fn default() -> Self {
        Self(true)
    }
}

#[derive(Component, Clone, Debug)]
pub enum Ordering {
    Floor = 0,
    Door = 1,
    Wall = 2,
    Other = 3,
}

impl Default for Ordering {
    fn default() -> Self {
        Self::Other
    }
}

#[derive(Component, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Position(pub IVec2);

impl Default for Position {
    fn default() -> Self {
        Self((0, 0).into())
    }
}

#[derive(Clone)]
pub enum SightKind {
    Blind,
    Omniscience,
    Eyes,
}

impl Default for SightKind {
    fn default() -> Self {
        Self::Blind
    }
}

#[derive(Component, Clone, Default)]
pub struct Sight {
    pub kind: SightKind,
    pub seeing: HashSet<Entity>,
}

#[derive(Debug)]
pub struct MemoryComponents {
    pub renderable: Renderable,
    pub position: Position,
    pub ordering: Ordering,
}

#[derive(Default, Component)]
pub struct Memory {
    pub entities: HashMap<Entity, MemoryComponents>,
}

#[derive(Component)]
pub struct Opaque(pub bool);

impl Default for Opaque {
    fn default() -> Self {
        Self(true)
    }
}

#[derive(Component)]
pub enum AI {
    None,
    ServBot,
}

impl Default for AI {
    fn default() -> Self {
        Self::None
    }
}
