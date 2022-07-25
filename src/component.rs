use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Wall;

#[derive(Component, Clone, Debug)]
pub struct Renderable {
    pub char: char,
    pub color: Color,
}

impl Default for Renderable {
    fn default() -> Self {
        Self {
            char: ' ',
            color: Default::default(),
        }
    }
}

#[derive(Component)]
pub enum Obstacle {
    Always,
    // Door,
}

#[derive(Component, Clone, Debug)]
pub struct Ordering(pub u8);

#[derive(Component, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Position(pub IVec2);

impl Default for Position {
    fn default() -> Self {
        Self((0, 0).into())
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum SightKind {
    Blind,
    Omniscience,
    Eyes,
    Sensors,
}

#[derive(Component, Clone)]
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
pub struct Memory(pub HashMap<Entity, MemoryComponents>);

#[derive(Component)]
pub struct Opaque(pub bool);

impl Default for Opaque {
    fn default() -> Self {
        Self(true)
    }
}
