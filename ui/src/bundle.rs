use bevy::prelude::*;

use crate::component;
use crate::resource::ReshockFont;

#[derive(Bundle)]
pub struct Tile {
    position: component::Position,

    #[bundle]
    text: Text2dBundle,
}

impl Tile {
    pub fn new(position: component::Position, font: &ReshockFont) -> Self {
        Self {
            position,
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
