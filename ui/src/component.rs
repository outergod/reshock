use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};
use bevy_tweening::Lens;

#[derive(Component, Clone, Hash, Debug, PartialEq, Eq)]
pub struct ReshockEntity(pub u32);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Room;

#[derive(Component, Clone, Debug)]
pub struct Renderable {
    pub char: char,
    pub color: Color,
}

pub struct ColorLens {
    pub start: Color,
    pub end: Color,
}

impl Lens<Renderable> for ColorLens {
    fn lerp(&mut self, target: &mut Renderable, ratio: f32) {
        let start: Vec4 = self.start.into();
        let end: Vec4 = self.end.into();
        let value = start.lerp(end, ratio);
        target.color = value.into();
    }
}

impl Default for Renderable {
    fn default() -> Self {
        Self {
            char: ' ',
            color: Default::default(),
        }
    }
}

#[derive(Component)]
pub struct Obstacle(pub bool);

impl Default for Obstacle {
    fn default() -> Self {
        Self(true)
    }
}

#[derive(Component, Clone, Debug)]
pub enum Ordering {
    Floor,
    Door,
    Wall,
    Other,
}

impl TryInto<Ordering> for i32 {
    type Error = ();

    fn try_into(self) -> Result<Ordering, Self::Error> {
        match self {
            0 => Ok(Ordering::Floor),
            1 => Ok(Ordering::Door),
            2 => Ok(Ordering::Wall),
            3 => Ok(Ordering::Other),
            _ => Err(()),
        }
    }
}

impl Default for Ordering {
    fn default() -> Self {
        Self::Other
    }
}

#[derive(Component, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Position(pub IVec2);

impl Default for Position {
    fn default() -> Self {
        Self((0, 0).into())
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum SightKind {
    Blind,
    Omniscience,
    Eyes,
}

impl TryInto<SightKind> for i32 {
    type Error = ();

    fn try_into(self) -> Result<SightKind, Self::Error> {
        match self {
            0 => Ok(SightKind::Blind),
            1 => Ok(SightKind::Omniscience),
            2 => Ok(SightKind::Eyes),
            _ => Err(()),
        }
    }
}

impl Default for SightKind {
    fn default() -> Self {
        Self::Blind
    }
}

#[derive(Component, Clone, Default)]
pub struct Sight {
    pub kind: SightKind,
    pub seeing: HashSet<ReshockEntity>,
}

#[derive(Debug)]
pub struct MemoryComponents {
    pub renderable: Renderable,
    pub position: Position,
    pub ordering: Ordering,
}

#[derive(Default, Component)]
pub struct Memory {
    pub entities: HashMap<ReshockEntity, MemoryComponents>,
    pub color: Color,
}

#[derive(Component)]
pub struct Opaque(pub bool);

impl Default for Opaque {
    fn default() -> Self {
        Self(true)
    }
}

#[derive(Component)]
pub struct Door {
    pub open: bool,
    pub toggle: bool,
    pub open_color: Color,
    pub close_color: Color,
}

impl Default for Door {
    fn default() -> Self {
        Self {
            open: false,
            toggle: false,
            open_color: Default::default(),
            close_color: Default::default(),
        }
    }
}

#[derive(Component)]
pub enum AI {
    None,
    ServBot,
}

impl Default for AI {
    fn default() -> Self {
        Self::None
    }
}
