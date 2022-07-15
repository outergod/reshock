use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Floor;

#[derive(Component)]
pub enum Renderable {
    Human,
    Floor,
}

impl Renderable {
    pub fn char(&self) -> char {
        match self {
            Renderable::Human => '@',
            Renderable::Floor => '·',
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Renderable::Human => Color::WHITE,
            Renderable::Floor => Color::GRAY,
        }
    }
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
