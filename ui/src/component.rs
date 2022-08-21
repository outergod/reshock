use bevy::prelude::*;
use bevy_tweening::Lens;

#[derive(Component, Clone, Hash, Debug, PartialEq, Eq)]
pub struct ReshockEntity(pub u32);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Focus;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Memory;

#[derive(Component, Clone, Debug)]
pub struct Renderable {
    pub char: char,
    pub color: Color,
}

pub struct ColorLens {
    pub start: Color,
    pub end: Color,
}

impl Lens<Renderable> for ColorLens {
    fn lerp(&mut self, target: &mut Renderable, ratio: f32) {
        let start: Vec4 = self.start.into();
        let end: Vec4 = self.end.into();
        let value = start.lerp(end, ratio);
        target.color = value.into();
    }
}

impl Default for Renderable {
    fn default() -> Self {
        Self {
            char: ' ',
            color: Default::default(),
        }
    }
}

#[derive(Component, Clone, Debug)]
pub enum Ordering {
    Floor,
    Door,
    Wall,
    Other,
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

#[derive(Component)]
pub struct Door {
    pub open: bool,
    pub open_color: Color,
    pub close_color: Color,
}

impl Default for Door {
    fn default() -> Self {
        Self {
            open: false,
            open_color: Default::default(),
            close_color: Default::default(),
        }
    }
}
