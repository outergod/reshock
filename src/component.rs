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

// impl Renderable {
//     pub fn char(&self) -> char {
//         match self {
//             Renderable::Human => '@',
//             Renderable::Floor => '·',
//             Renderable::Wall => 'X',
//         }
//     }

//     pub fn color(&self) -> Color {
//         match self {
//             Renderable::Human => Color::WHITE,
//             Renderable::Floor => Color::GRAY,
//             Renderable::Wall => Color::ALICE_BLUE,
//         }
//     }
// }

#[derive(Component)]
pub struct Ordering(pub u8);

#[derive(Component, Hash, PartialEq, Eq)]
pub struct Position(pub IVec2);

impl Default for Position {
    fn default() -> Self {
        Self((0, 0).into())
    }
}
