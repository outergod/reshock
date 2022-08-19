use std::collections::VecDeque;

use bevy::{
    math::ivec2,
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

#[derive(Debug, PartialEq)]
pub enum TransitionState {
    Active,
    Inactive,
}

impl Default for TransitionState {
    fn default() -> Self {
        Self::Inactive
    }
}

#[derive(Debug, Default)]
pub struct ReshockEvents {
    pub queue: VecDeque<api::Event>,
    pub state: TransitionState,
}

pub struct Deltas(pub HashSet<IVec2>);

impl Default for Deltas {
    fn default() -> Self {
        let deltas: HashSet<_> = (-1..=1)
            .flat_map(|x| {
                (-1..=1).filter_map(move |y| {
                    if x == 0 && y == 0 {
                        None
                    } else {
                        Some(ivec2(x, y))
                    }
                })
            })
            .collect();

        Self(deltas)
    }
}
