use api::shoot_event::ShootKind;
use bevy_ecs::prelude::*;

use crate::game::{component::*, Events, *};

pub fn effect(
    action: Res<Action>,
    mut events: ResMut<Events>,
    mut magazines: Query<&mut Magazine>,
    sight: Query<&Sight, With<Player>>,
    weapons: Query<&RangedWeapon>,
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

    let sight = sight.single();

    let source = match sight.seeing.get(actor) {
        Some(it) => it.iter().next().unwrap().to_owned().into(),
        None => return,
    };

    let target = match sight.seeing.get(target) {
        Some(it) => it.iter().next().unwrap().to_owned().into(),
        None => return,
    };

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
