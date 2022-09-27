use std::fmt::Display;

use glam::{ivec2, IVec2};
use hit_event::HitDirection;

tonic::include_proto!("reshock");

pub const FILE_DESCRIPTOR_SET: &'static [u8] =
    tonic::include_file_descriptor_set!("reshock_descriptor");

impl Display for event::Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            event::Event::State(_) => "State",
            event::Event::Door(_) => "Door",
            event::Event::Spot(_) => "Spot",
            event::Event::Log(_) => "Log",
            event::Event::Hit(_) => "Hit",
            event::Event::Death(_) => "Death",
            event::Event::Shoot(_) => "Shoot",
        };

        write!(f, "{}", name)
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.event {
            Some(event) => write!(f, "{}", event),
            None => write!(f, "(empty)"),
        }
    }
}

impl From<IVec2> for Position {
    fn from(pos: IVec2) -> Self {
        Self { x: pos.x, y: pos.y }
    }
}

impl From<IVec2> for PositionComponent {
    fn from(pos: IVec2) -> Self {
        Self { x: pos.x, y: pos.y }
    }
}

impl From<IVec2> for HitDirection {
    fn from(vec: IVec2) -> Self {
        match (vec.x, vec.y) {
            (0, 1) => HitDirection::Top,
            (1, 1) => HitDirection::TopRight,
            (1, 0) => HitDirection::Right,
            (1, -1) => HitDirection::BottomRight,
            (0, -1) => HitDirection::Bottom,
            (-1, -1) => HitDirection::BottomLeft,
            (-1, 0) => HitDirection::Left,
            (-1, 1) => HitDirection::TopLeft,
            _ => HitDirection::None,
        }
    }
}

impl From<HitDirection> for IVec2 {
    fn from(direction: HitDirection) -> Self {
        match direction {
            HitDirection::Top => ivec2(0, 1),
            HitDirection::TopRight => ivec2(1, 1),
            HitDirection::Right => ivec2(1, 0),
            HitDirection::BottomRight => ivec2(1, -1),
            HitDirection::Bottom => ivec2(0, -1),
            HitDirection::BottomLeft => ivec2(-1, -1),
            HitDirection::Left => ivec2(-1, 0),
            HitDirection::TopLeft => ivec2(-1, 1),
            HitDirection::None => ivec2(0, 0),
        }
    }
}
