use std::collections::{HashMap, HashSet};

use glam::IVec2;

pub type Path = Vec<IVec2>;

#[derive(Debug)]
pub struct RadialLines(pub HashMap<IVec2, HashSet<Path>>);
