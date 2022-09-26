use std::collections::VecDeque;
use std::fmt::Display;

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
    pub symbol_handle: Handle<Font>,
    pub size: f32,
    pub bounding_glyph: char,
}

impl ReshockFont {
    pub fn handle_for(&self, c: char) -> Handle<Font> {
        match c {
            '⌖' => self.symbol_handle.clone_weak(),
            _ => self.handle.clone_weak(),
        }
    }
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

impl Display for ReshockEvents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let separator = " -> ".to_string();
        for token in self
            .queue
            .iter()
            .map(|event| event.to_string())
            .intersperse(separator)
        {
            write!(f, "{}", token)?;
        }

        Ok(())
    }
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

#[derive(Default)]
pub struct Log(pub Vec<String>);

impl Log {
    #[inline]
    pub fn read(&self) -> String {
        self.0.join("\n")
    }
}
