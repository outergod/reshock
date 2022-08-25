use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    slice::Iter,
};

use bevy_ecs::prelude::*;
use glam::{ivec2, IVec2};

pub type Path = Vec<IVec2>;

#[derive(Debug)]
pub struct RadialLines(pub HashMap<IVec2, HashSet<Path>>);

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
pub struct SpatialHash {
    pub cells: HashMap<IVec2, HashSet<Entity>>,
}

pub struct Room(pub HashMap<IVec2, char>);

impl From<String> for Room {
    fn from(s: String) -> Self {
        let room = s
            .lines()
            .rev()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, c)| ((x as i32, y as i32).into(), c))
            })
            .collect();

        Self(room)
    }
}

#[derive(Default)]
pub struct Log(Vec<String>);

impl Log {
    #[inline]
    pub fn add(&mut self, s: impl Display) {
        self.0.push(s.to_string());
    }

    #[inline]
    pub fn read(&self) -> Iter<String> {
        self.0.iter()
    }
}
