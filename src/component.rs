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

#[derive(Component)]
pub enum Sight {
    Omniscience,
    Eyes,
    Sensors,
}
