use bevy::prelude::*;

use crate::component;
use crate::resource::ReshockFont;

#[derive(Bundle)]
pub struct Tile {
    position: component::Position,
    tile: component::Tile,

    #[bundle]
    text: Text2dBundle,
}

impl Tile {
    pub fn new(position: component::Position, font: &ReshockFont) -> Self {
        Self {
            position,
            tile: component::Tile,
            text: Text2dBundle {
                text: Text::from_section(
                    " ".to_string(),
                    TextStyle {
                        font: font.handle.clone_weak(),
                        font_size: font.size,
                        color: Color::WHITE,
                    },
                ),
                ..Default::default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct Effect {
    pub effect: component::Effect,
    pub position: component::Position,
    pub renderable: component::Renderable,
}
