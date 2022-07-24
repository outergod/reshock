use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Wall;

#[derive(Component, Clone)]
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

#[derive(Component)]
pub struct Ordering(pub u8);

#[derive(Component, Hash, PartialEq, Eq)]
pub struct Position(pub IVec2);

impl Default for Position {
    fn default() -> Self {
        Self((0, 0).into())
    }
}

#[derive(Component, Clone)]
pub enum Sight {
    Blind,
    Omniscience,
    Eyes,
    Sensors,
}

#[derive(Component)]
pub struct Visible(pub bool);

impl Default for Visible {
    fn default() -> Self {
        Self(false)
    }
}

#[derive(Component)]
pub struct Opaque(pub bool);

impl Default for Opaque {
    fn default() -> Self {
        Self(true)
    }
}
