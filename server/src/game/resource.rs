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

#[derive(Default, Clone, Debug)]
pub struct Cell {
    pub visible: HashSet<Entity>,
    pub door: Option<Entity>,
    pub wall: Option<Entity>,
    pub solid: Option<Entity>,
    pub opaque: HashSet<Entity>,
    pub vulnerable: Option<Entity>,
}

#[derive(Default, Clone)]
pub struct SpatialHash {
    pub cells: HashMap<IVec2, Cell>,
}

impl SpatialHash {
    pub fn entities_at(&self, pos: &IVec2) -> HashSet<Entity> {
        self.cells.get(pos).cloned().unwrap_or_default().visible
    }

    pub fn is_opaque(&self, pos: &IVec2) -> bool {
        self.cells
            .get(pos)
            .is_some_and(|cell| !cell.opaque.is_empty())
    }

    pub fn door_at(&self, pos: &IVec2) -> Option<Entity> {
        self.cells.get(pos).cloned().unwrap_or_default().door
    }

    pub fn wall_at(&self, pos: &IVec2) -> Option<Entity> {
        self.cells.get(pos).cloned().unwrap_or_default().wall
    }

    pub fn vulnerable_at(&self, pos: &IVec2) -> Option<Entity> {
        self.cells.get(pos).cloned().unwrap_or_default().vulnerable
    }
}

impl Display for SpatialHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        let (min_x, min_y, max_x, max_y) =
            self.cells
                .keys()
                .fold((0, 0, 0, 0), |(min_x, min_y, max_x, max_y), item| {
                    (
                        min_x.min(item.x),
                        min_y.min(item.y),
                        max_x.max(item.x),
                        max_y.max(item.y),
                    )
                });

        let width = (max_x - min_x).abs() + 1;
        let height = (max_y - min_y).abs() + 1;

        for y in (0..height as i32).rev() {
            for x in 0..width as i32 {
                let pos = ivec2(x + min_x, y + min_y);
                if self.door_at(&pos).is_some() {
                    s.push('o');
                } else if self.wall_at(&pos).is_some() {
                    s.push('#');
                } else if self.cells.contains_key(&pos) {
                    s.push('·');
                } else {
                    s.push(' ');
                }
            }
            s.push('\n');
        }

        write!(f, "{}", s)
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
