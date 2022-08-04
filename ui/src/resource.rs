use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};

#[derive(Default)]
pub struct TileDimensions(pub Option<Size>);

#[derive(Default)]
pub struct ReshockFont {
    pub handle: Handle<Font>,
    pub size: f32,
    pub bounding_glyph: char,
}

// impl Default for ReshockFont {
//     fn default() -> Self {
//         Self {
//             handle: Default::default(),
//             size: 30.0,
//             bounding_glyph: '@',
//         }
//     }
// }

pub type Path = Vec<IVec2>;

#[derive(Debug)]
pub struct RadialLines(pub HashMap<IVec2, HashSet<Path>>);
