use std::{collections::HashSet, fmt::Display, slice::Iter};

use glam::{ivec2, IVec2};

pub type Path = Vec<IVec2>;

#[derive(Debug)]
pub struct RadialLines(pub HashSet<Path>);

pub struct Deltas(pub HashSet<IVec2>);

impl Deltas {
    pub fn neighbors() -> Self {
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

    pub fn cross() -> Self {
        let deltas = [ivec2(0, 1), ivec2(1, 0), ivec2(0, -1), ivec2(-1, 0)]
            .into_iter()
            .collect();

        Self(deltas)
    }
}

impl Default for Deltas {
    fn default() -> Self {
        Self::neighbors()
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
