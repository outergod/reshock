use api::shoot_event::ShootKind;
use bevy_ecs::prelude::*;

use crate::game::{component::*, Events, *};

pub fn effect(
    action: Res<Action>,
    mut magazines: Query<&mut Magazine>,
    positions: Query<&Position>,
    weapons: Query<&RangedWeapon>,
    mut events: ResMut<Events>,
) {
    let (actor, target, weapon, magazine) = match action.as_ref() {
        Action::Shoot(ShootAction::DispatchProjectile {
            actor,
            target,
            weapon,
            magazine,
        }) => (actor, target, weapon, magazine),
        _ => return,
    };

    magazines.get_mut(*magazine).unwrap().amount -= 1;

    let source = positions.get(*actor).unwrap().into();
    let target = positions.get(*target).unwrap().into();
    let weapon = weapons.get(*weapon).unwrap();

    let (kind, sound) = match weapon {
        RangedWeapon::Projectile(gun) => match gun.kind {
            ProjectileGunKind::RiotGun => todo!(),
            ProjectileGunKind::DartPistol => todo!(),
            ProjectileGunKind::Minipistol => todo!(),
            ProjectileGunKind::Flechette => todo!(),
            ProjectileGunKind::Magnum => todo!(),
            ProjectileGunKind::Skorpion => todo!(),
            ProjectileGunKind::AssaultRifle => (
                ShootKind::Projectile,
                api::shoot_event::ShootSound::Mark3AssaultRifle,
            ),
            ProjectileGunKind::RailGun => todo!(),
        },
        RangedWeapon::Energy(gun) => match gun.kind {
            EnergyGunKind::StunGun => todo!(),
            EnergyGunKind::Sparq => todo!(),
            EnergyGunKind::Blaster => todo!(),
            EnergyGunKind::IonPulse => todo!(),
            EnergyGunKind::Plasma => todo!(),
        },
    };

    events.0.push(api::Event {
        event: Some(api::event::Event::Shoot(api::ShootEvent {
            source: Some(source),
            target: Some(target),
            kind: kind as i32,
            sound: sound as i32,
        })),
    });
}
