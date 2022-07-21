use bevy::prelude::*;

use crate::asset;

#[derive(Default)]
pub struct TileDimensions(pub Option<Size>);

pub struct ReshockFont {
    pub handle: Handle<Font>,
    pub size: f32,
    pub bounding_glyph: char,
}

pub struct Room(pub Handle<asset::Room>);
