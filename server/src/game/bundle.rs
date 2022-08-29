use bevy_ecs::prelude::*;

use super::component;

#[derive(Bundle)]
pub struct Player {
    pub renderable: component::Renderable,
    pub position: component::Position,
    pub solid: component::Solid,
    pub sight: component::Sight,
    pub memory: component::Memory,
    pub player: component::Player,
    pub description: component::Description,
    pub melee: component::MeleeSlot,
    pub gun: component::GunSlot,
    pub battery: component::PersonalBattery,
    pub vulnerable: component::Vulnerable,
    pub alive: component::Alive,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            renderable: component::Renderable::Human,
            position: Default::default(),
            solid: Default::default(),
            sight: component::Sight {
                kind: component::SightKind::Eyes,
                ..Default::default()
            },
            memory: Default::default(),
            player: component::Player,
            description: component::Description {
                name: "the Hacker".to_string(),
                article: component::Article::None,
            },
            melee: Default::default(),
            gun: Default::default(),
            battery: component::PersonalBattery {
                max: 200,
                charge: 50,
            },
            vulnerable: component::Vulnerable {
                kind: component::VulnerableKind::Humanoid,
                hp: 212,
                max: 255,
                defense: 4,
                armor: 0,
            },
            alive: component::Alive::Human,
        }
    }
}

#[derive(Bundle, Default)]
pub struct NPC {
    pub renderable: component::Renderable,
    pub position: component::Position,
    pub solid: component::Solid,
    pub sight: component::Sight,
    pub memory: component::Memory,
    pub ai: component::AI,
    pub description: component::Description,
    pub vulnerable: component::Vulnerable,
}

#[derive(Bundle)]
pub struct Floor {
    pub renderable: component::Renderable,
    pub position: component::Position,
    pub description: component::Description,
}

impl Default for Floor {
    fn default() -> Self {
        Self {
            renderable: component::Renderable::Floor,
            position: Default::default(),
            description: component::Description {
                name: "flooring".to_string(),
                article: component::Article::None,
            },
        }
    }
}

#[derive(Bundle)]
pub struct Wall {
    pub room: component::Room,
    pub renderable: component::Renderable,
    pub position: component::Position,
    pub solid: component::Solid,
    pub opaque: component::Opaque,
    pub wall: component::Wall,
    pub description: component::Description,
}

impl Default for Wall {
    fn default() -> Self {
        Self {
            room: component::Room,
            renderable: component::Renderable::Wall,
            position: Default::default(),
            solid: Default::default(),
            opaque: Default::default(),
            wall: Default::default(),
            description: component::Description {
                name: "wall".to_string(),
                article: component::Article::A,
            },
        }
    }
}

#[derive(Bundle)]
pub struct Door {
    pub door: component::Door,
    pub room: component::Room,
    pub renderable: component::Renderable,
    pub position: component::Position,
    pub description: component::Description,
}

impl Default for Door {
    fn default() -> Self {
        Self {
            door: Default::default(),
            room: component::Room,
            renderable: component::Renderable::Door,
            position: Default::default(),
            description: component::Description {
                name: "door".to_string(),
                article: component::Article::A,
            },
        }
    }
}

#[derive(Bundle)]
pub struct MeleeWeapon {
    pub melee: component::MeleeWeapon,
    pub item: component::Item,
    pub renderable: component::Renderable,
    pub description: component::Description,
}

impl MeleeWeapon {
    pub fn lead_pipe() -> Self {
        Self {
            melee: component::MeleeWeapon {
                kind: component::MeleeWeaponKind::LeadPipe,
                damage: component::Damage {
                    attack: component::AttackKind::Kinetic,
                    amount: 15,
                    penetration: 40,
                    offense: 3,
                },
            },
            item: Default::default(),
            renderable: component::Renderable::Melee,
            description: component::Description {
                name: "lead pipe".to_string(),
                article: component::Article::A,
            },
        }
    }

    pub fn laser_rapier() -> Self {
        Self {
            melee: component::MeleeWeapon {
                kind: component::MeleeWeaponKind::LaserRapier,
                damage: component::Damage {
                    attack: component::AttackKind::Beam,
                    amount: 200,
                    penetration: 80,
                    offense: 5,
                },
            },
            item: Default::default(),
            renderable: component::Renderable::Melee,
            description: component::Description {
                name: "TS-04 Laser Rapier".to_string(),
                article: component::Article::A,
            },
        }
    }
}

#[derive(Bundle)]
pub struct ProjectileGun {
    pub gun: component::ProjectileGun,
    pub item: component::Item,
    pub renderable: component::Renderable,
    pub description: component::Description,
}

impl ProjectileGun {
    pub fn assault_rifle() -> Self {
        Self {
            gun: component::ProjectileGun {
                kind: component::ProjectileGunKind::AssaultRifle,
                operation: component::OperationKind::SemiAutomatic,
            },
            item: Default::default(),
            renderable: component::Renderable::ProjectileGun,
            description: component::Description {
                name: "MARK III Assault Rifle".to_string(),
                article: component::Article::A,
            },
        }
    }
}

#[derive(Bundle)]
pub struct Magazine {
    pub magazine: component::Magazine,
    pub item: component::Item,
    pub renderable: component::Renderable,
    pub description: component::Description,
}

impl Magazine {
    pub fn magnesium_tips() -> Self {
        Self {
            magazine: component::Magazine {
                kind: component::ProjectileGunKind::AssaultRifle,
                damage: component::Damage {
                    attack: component::AttackKind::Kinetic,
                    amount: 75,
                    penetration: 50,
                    offense: 4,
                },
                amount: 10,
                attached: None,
            },
            item: Default::default(),
            renderable: component::Renderable::Magazine,
            description: component::Description {
                name: "magazine of 10 magnesium-tipped rounds".to_string(),
                article: component::Article::A,
            },
        }
    }
}

#[derive(Bundle)]
pub struct EnergyGun {
    pub gun: component::EnergyGun,
    pub item: component::Item,
    pub renderable: component::Renderable,
    pub description: component::Description,
}

impl EnergyGun {
    pub fn ion_rifle() -> Self {
        Self {
            gun: component::EnergyGun {
                kind: component::EnergyGunKind::IonPulse,
                operation: component::OperationKind::SemiAutomatic,
                damage: component::Damage {
                    attack: component::AttackKind::Beam,
                    amount: 18,
                    penetration: 35,
                    offense: 6,
                },
                efficiency: 3.6,
                max: 10,
            },
            item: Default::default(),
            renderable: component::Renderable::EnergyGun,
            description: component::Description {
                name: "RW-45 Ion Rifle".to_string(),
                article: component::Article::An,
            },
        }
    }
}
