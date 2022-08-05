use bevy_ecs::prelude::*;

#[derive(Debug, Clone)]
pub enum Command {
    UpLeft,
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
}

pub struct ToggleDoor {
    pub entity: Entity,
    pub open: bool,
}
