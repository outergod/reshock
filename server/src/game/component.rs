use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::*;
use glam::IVec2;

#[derive(Component, Default)]
pub struct Player;

#[derive(Component, Default, Clone, Copy)]
pub struct Wall;

impl From<&Wall> for api::WallComponent {
    fn from(_: &Wall) -> Self {
        Self {}
    }
}

#[derive(Component, Default)]
pub struct God;

#[derive(Component, Default, Clone)]
pub struct Position(pub IVec2);

impl From<&Position> for api::PositionComponent {
    fn from(position: &Position) -> Self {
        Self {
            x: position.0.x,
            y: position.0.y,
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct Door {
    pub open: bool,
}

impl Default for Door {
    fn default() -> Self {
        Self { open: false }
    }
}

impl From<&Door> for api::DoorComponent {
    fn from(door: &Door) -> Self {
        Self { open: door.open }
    }
}

#[derive(Component, Clone)]
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

impl From<&Renderable> for api::RenderableComponent {
    fn from(renderable: &Renderable) -> Self {
        Self {
            renderable: *renderable as i32,
        }
    }
}

#[derive(Component, Clone)]
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

impl From<&Ordering> for api::OrderingComponent {
    fn from(ordering: &Ordering) -> Self {
        Self {
            ordering: *ordering as i32,
        }
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

#[derive(Default)]
pub struct MemoryComponents {
    pub position: Position,
    pub renderable: Renderable,
    pub ordering: Ordering,
    pub door: Option<Door>,
    pub wall: Option<Wall>,
}

impl From<&MemoryComponents> for api::Components {
    fn from(memory: &MemoryComponents) -> Self {
        Self {
            position: Some((&memory.position).into()),
            renderable: Some((&memory.renderable).into()),
            ordering: Some((&memory.ordering).into()),
            door: memory.door.map(|it| (&it).into()),
            wall: memory.wall.map(|it| (&it).into()),
            memory: Some(api::MemoryComponent {}),
        }
    }
}

#[derive(Component, Default)]
pub struct Memory(pub HashMap<Entity, MemoryComponents>);

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
