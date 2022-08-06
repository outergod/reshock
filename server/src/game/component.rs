use std::collections::HashSet;

use bevy_ecs::prelude::*;
use glam::IVec2;

#[derive(Component, Default)]
pub struct Player;

#[derive(Component, Default)]
pub struct Position(pub IVec2);

#[derive(Component)]
pub struct Door {
    pub open: bool,
}

impl Default for Door {
    fn default() -> Self {
        Self { open: false }
    }
}

#[derive(Component)]
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

#[derive(Component, Default)]
pub struct Sight {
    pub kind: SightKind,
    pub seeing: HashSet<Entity>,
}

#[derive(Component, Default)]
pub struct Memory(pub World);

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

#[derive(Default, Component)]
pub struct Opaque;

#[derive(Default, Component)]
pub struct Solid;

#[derive(Default, Component)]
pub struct Room;
