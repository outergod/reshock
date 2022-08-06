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
pub enum RenderableKind {
    None,
    Human,
    ServBot,
    Floor,
    Wall,
    Door,
}

impl Default for RenderableKind {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Component, Clone, Debug)]
pub struct Renderable {
    pub kind: RenderableKind,
    pub char: char,
    pub color: Color,
}

impl TryInto<Renderable> for api::RenderableComponent {
    type Error = ();

    fn try_into(self) -> Result<Renderable, Self::Error> {
        use api::renderable_component::Renderable as ApiRenderable;
        match ApiRenderable::from_i32(self.renderable) {
            Some(ApiRenderable::None) => Ok(Renderable::default()),
            Some(ApiRenderable::Wall) => Ok(Renderable {
                kind: RenderableKind::Wall,
                ..Default::default()
            }),
            Some(ApiRenderable::Door) => Ok(Renderable {
                kind: RenderableKind::Door,
                ..Default::default()
            }),
            Some(ApiRenderable::Human) => Ok(Renderable {
                kind: RenderableKind::Human,
                char: '@',
                color: Color::WHITE,
            }),
            Some(ApiRenderable::ServBot) => Ok(Renderable {
                kind: RenderableKind::ServBot,
                char: 'b',
                color: Color::ORANGE_RED,
            }),
            Some(ApiRenderable::Floor) => Ok(Renderable {
                kind: RenderableKind::Floor,
                char: '·',
                color: Color::ALICE_BLUE,
            }),
            None => Err(()),
        }
    }
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
            kind: Default::default(),
            char: ' ',
            color: Default::default(),
        }
    }
}

#[derive(Component, Clone, Debug)]
pub enum Ordering {
    Floor,
    Door,
    Wall,
    Other,
}

impl TryFrom<i32> for Ordering {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
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

#[derive(Component, Clone, Default)]
pub struct Sight {
    pub seeing: HashSet<ReshockEntity>,
}

pub struct MemoryComponents {
    pub player: Option<Player>,
    pub wall: Option<Wall>,
    pub room: Option<Room>,
    pub door: Option<Door>,
    pub renderable: Renderable,
    pub position: Position,
    pub ordering: Ordering,
}

impl TryFrom<api::Entity> for MemoryComponents {
    type Error = ();

    fn try_from(value: api::Entity) -> Result<Self, Self::Error> {
        match (value.renderable, value.position, value.ordering) {
            (Some(renderable), Some(position), Some(ordering)) => Ok(Self {
                player: value.player.map(|_| Player {}),
                wall: value.wall.map(|_| Wall {}),
                room: value.room.map(|_| Room {}),
                door: value.door.map(|door| Door {
                    open: door.open,
                    ..Default::default()
                }),
                renderable: renderable.try_into()?,
                position: Position((position.x, position.y).into()),
                ordering: ordering.ordering.try_into()?,
            }),
            _ => Err(()),
        }
    }
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
