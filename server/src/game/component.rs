use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use bevy_ecs::prelude::*;
use glam::IVec2;

#[derive(Component, Default, Clone, Debug)]
pub struct Player;

#[allow(dead_code)]
pub enum Article {
    None,
    A,
    An,
}

impl Default for Article {
    fn default() -> Self {
        Self::None
    }
}

impl Display for Article {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let article = match self {
            Article::None => "",
            Article::A => "a ",
            Article::An => "an ",
        };

        write!(f, "{}", article)
    }
}

#[derive(Component, Default)]
pub struct Description {
    pub name: String,
    pub article: Article,
}

impl Display for Description {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.article, self.name)
    }
}

impl Description {
    pub fn to_capitalized_string(&self) -> String {
        let s = self.to_string();
        let mut c = s.chars();
        match c.next() {
            Some(f) => f.to_uppercase().to_string() + c.as_str(),
            None => String::new(),
        }
    }
}

#[derive(Component, Default, Clone, Copy, Debug)]
pub struct Wall;

impl From<&Wall> for api::WallComponent {
    fn from(_: &Wall) -> Self {
        Self {}
    }
}

#[derive(Component)]
pub struct Floor;

#[derive(Component, Default)]
pub struct God;

#[derive(Component, Default, Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Position(pub IVec2);

impl From<&Position> for api::PositionComponent {
    fn from(position: &Position) -> Self {
        Self {
            x: position.0.x,
            y: position.0.y,
        }
    }
}

impl From<&Position> for api::Position {
    fn from(position: &Position) -> Self {
        Self {
            x: position.0.x,
            y: position.0.y,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum DoorKind {
    Door,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Door {
    pub kind: DoorKind,
    pub open: bool,
}

impl Default for Door {
    fn default() -> Self {
        Self {
            kind: DoorKind::Door,
            open: false,
        }
    }
}

impl From<&Door> for api::DoorComponent {
    fn from(door: &Door) -> Self {
        Self { open: door.open }
    }
}

#[derive(Component, Clone, Debug)]
pub enum Renderable {
    None,
    Human,
    ServBot,
    Floor,
    Wall,
    Door,
    Melee,
    ProjectileGun,
    EnergyGun,
    Magazine,
    Corpse,
}

impl Default for Renderable {
    fn default() -> Self {
        Self::None
    }
}

impl From<&Renderable> for api::RenderableComponent {
    fn from(renderable: &Renderable) -> Self {
        Self {
            renderable: *renderable as i32,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SightKind {
    Blind,
    Omniscience,
    Eyes,
}

impl Default for SightKind {
    fn default() -> Self {
        Self::Blind
    }
}

#[derive(Component, Default, Debug, Clone)]
pub struct Sight {
    pub kind: SightKind,
    pub seeing: HashSet<Entity>,
    pub mask: HashSet<IVec2>,
}

#[derive(Debug, Clone)]
pub struct MemoryComponents {
    pub position: Position,
    pub renderable: Renderable,
    pub door: Option<Door>,
    pub wall: Option<Wall>,
    pub player: Option<Player>,
}

impl From<&MemoryComponents> for api::Components {
    fn from(memory: &MemoryComponents) -> Self {
        Self {
            position: Some((&memory.position).into()),
            renderable: Some((&memory.renderable).into()),
            door: memory.door.map(|it| (&it).into()),
            wall: memory.wall.map(|it| (&it).into()),
            memory: Some(api::MemoryComponent {}),
        }
    }
}

#[derive(Component, Default, Debug, Clone)]
pub struct Memory(pub HashMap<Entity, MemoryComponents>);

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

#[derive(Default, Component)]
pub struct Opaque;

#[derive(Default, Component)]
pub struct Solid;

#[derive(Default, Component)]
pub struct Room;

#[derive(Default, Component)]
pub struct Item {
    pub owner: Option<Entity>,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Copy)]
pub enum AttackKind {
    Kinetic,
    Beam,
    Needle,
    Tranquilizer,
    Magnetic,
    Gas,
}

#[derive(Component, Clone, Debug, Copy)]
pub struct Damage {
    pub attack: AttackKind,
    pub amount: u16,
    pub penetration: u8,
    pub offense: u8,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Copy)]
pub enum MeleeWeaponKind {
    LeadPipe,
    LaserRapier,
    Appendages,
}

#[derive(Component, Clone, Debug, Copy)]
pub struct MeleeWeapon {
    pub kind: MeleeWeaponKind,
    pub damage: Damage,
}

#[allow(dead_code)]
#[derive(Component)]
pub enum ProjectileKind {
    RubberSlug,
    Needle,
    Bullet,
    Slug,
    Slag,
    EmpBurst,
    Grenade,
}

#[derive(Component)]
pub struct Projectile {
    pub kind: ProjectileKind,
    pub damage: Damage,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum OperationKind {
    SemiAutomatic,
    Automatic(u16),
}

impl OperationKind {
    pub fn amount(&self) -> u16 {
        match self {
            OperationKind::SemiAutomatic => 1,
            OperationKind::Automatic(n) => *n,
        }
    }
}

#[allow(dead_code)]
#[derive(Component)]
pub enum RangedWeapon {
    Projectile(ProjectileGun),
    Energy(EnergyGun),
}

#[allow(dead_code)]
impl RangedWeapon {
    pub fn projectile(&self) -> Option<ProjectileGun> {
        match self {
            RangedWeapon::Projectile(it) => Some(*it),
            RangedWeapon::Energy(_) => None,
        }
    }

    pub fn energy(&self) -> Option<EnergyGun> {
        match self {
            RangedWeapon::Projectile(_) => None,
            RangedWeapon::Energy(it) => Some(*it),
        }
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct ProjectileGun {
    pub kind: ProjectileGunKind,
    pub operation: OperationKind,
}

#[allow(dead_code)]
#[derive(Component)]
pub struct Magazine {
    pub gun: ProjectileGunKind,
    pub projectile: Projectile,
    pub amount: u16,
    pub attached: Option<Entity>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum ProjectileGunKind {
    RiotGun,
    DartPistol,
    Minipistol,
    Flechette,
    Magnum,
    Skorpion,
    AssaultRifle,
    RailGun,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum EnergyGunKind {
    StunGun,
    Sparq,
    Blaster,
    IonPulse,
    Plasma,
}

#[allow(dead_code)]
pub enum BeamKind {
    Stun,
    SparqBeam,
    Laser,
    IonPulse,
    Plasma,
}

#[derive(Component)]
pub struct Beam {
    pub kind: BeamKind,
    pub damage: Damage,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct EnergyGun {
    pub kind: EnergyGunKind,
    pub operation: OperationKind,
    pub damage: Damage,
    pub efficiency: f32,
    pub max: u8,
}

#[derive(Component)]
pub struct PersonalBattery {
    pub max: u16,
    pub charge: u16,
}

#[derive(Component, Default)]
pub struct MeleeSlot;

#[derive(Component, Default)]
pub struct GunSlot;

#[derive(Component, Default)]
pub struct Equipped;

#[allow(dead_code)]
#[derive(Component)]
pub enum VulnerableKind {
    None,
    Avian,
    GorillaTiger,
    Humanoid,
    Invisible,
    Plant,
    Virus,
    ZeroGrav,
    Robot,
    RoboticCyborg,
    HumanoidCyborg,
}

impl Default for VulnerableKind {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Component, Default)]
pub struct Vulnerable {
    pub kind: VulnerableKind,
    pub hp: u16,
    pub max: u16,
    pub defense: u8,
    pub armor: u8,
}

#[derive(Component, Debug, Clone, Copy)]
pub enum Alive {
    Human,
    ServBot,
}

#[derive(Component)]
pub struct RoomSpawner;
