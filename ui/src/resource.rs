use std::collections::VecDeque;

use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};

#[derive(Default)]
pub struct TileDimensions(pub Option<Size>);

#[derive(Default, Clone)]
pub struct ReshockFont {
    pub handle: Handle<Font>,
    pub size: f32,
    pub bounding_glyph: char,
}

pub type Path = Vec<IVec2>;

#[derive(Debug)]
pub struct RadialLines(pub HashMap<IVec2, HashSet<Path>>);

#[derive(Debug, Default)]
pub struct ReshockEvents(pub VecDeque<api::Event>);
