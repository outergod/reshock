use bevy_ecs::prelude::*;

use crate::game::{component::*, Status, *};

pub fn intent(
    action: Res<ActiveAction>,
    weapons: Query<(Entity, &Item, &RangedWeapon), With<Equipped>>,
    descriptions: Query<&Description>,
    mut reactions: ResMut<Reactions>,
) -> Status {
    let (actor, target) = match &action.0 {
        Some(Action::Shoot(ShootAction::Intent { actor, target })) => (actor, target),
        _ => return Status::Continue,
    };

    match weapons.iter().find_map(|(entity, item, weapon)| {
        item.owner
            .is_some_and(|owner| owner == actor)
            .then_some((entity, weapon))
    }) {
        Some((entity, weapon)) => {
            match weapon {
                RangedWeapon::Projectile(_) => {
                    reactions.0.push(Action::Shoot(ShootAction::ProjectileGun {
                        actor: *actor,
                        target: *target,
                        weapon: entity,
                    }));
                }
                RangedWeapon::Energy(_) => todo!(),
            }
            Status::Continue
        }
        None => {
            let action = descriptions
                .get(*actor)
                .ok()
                .map(|s| Action::Log(format!("{} has no ranged weapon equipped", s)));

            Status::Reject(action)
        }
    }
}

pub fn shoot_projectile(
    action: Res<ActiveAction>,
    descriptions: Query<&Description>,
    weapons: Query<&RangedWeapon>,
    magazines: Query<(Entity, &Magazine)>,
    mut reactions: ResMut<Reactions>,
    mut followups: ResMut<FollowUps>,
) -> Status {
    let (actor, target, weapon) = match &action.0 {
        Some(Action::Shoot(ShootAction::ProjectileGun {
            actor,
            target,
            weapon,
        })) => (actor, target, weapon),
        _ => return Status::Continue,
    };

    let description = descriptions.get(*weapon);

    match magazines.iter().find(|(_, m)| m.attached == Some(*weapon)) {
        Some((entity, magazine)) => {
            let w = weapons
                .get(*weapon)
                .ok()
                .and_then(|w| w.projectile())
                .unwrap();

            if magazine.amount > 0 {
                let amount = w.operation.amount().min(magazine.amount);

                let action = Action::Shoot(ShootAction::DispatchProjectile {
                    actor: *actor,
                    target: *target,
                    weapon: *weapon,
                    magazine: entity,
                });

                for _ in 0..amount {
                    reactions.0.push(action.clone());
                }

                followups.0.push(Action::EndTurn(*actor));

                Status::Continue
            } else {
                let action = description
                    .ok()
                    .map(|s| Action::Log(format!("{} has an empty magazine attached", s)));

                Status::Reject(action)
            }
        }
        None => {
            let action = description
                .ok()
                .map(|s| Action::Log(format!("{} has no magazine attached", s)));

            Status::Reject(action)
        }
    }
}

pub fn dispatch_projectile(
    action: Res<ActiveAction>,
    magazines: Query<&Magazine>,
    mut reactions: ResMut<Reactions>,
) -> Status {
    let (actor, target, weapon, magazine) = match &action.0 {
        Some(Action::Shoot(ShootAction::DispatchProjectile {
            actor,
            target,
            weapon,
            magazine,
        })) => (actor, target, weapon, magazine),
        _ => return Status::Continue,
    };

    // TODO hit chance, use position instead of target etc

    let magazine = magazines.get(*magazine).unwrap();

    let action = Action::Hit(HitAction {
        actor: *actor,
        target: *target,
        weapon: *weapon,
        damage: magazine.projectile.damage,
    });
    reactions.0.push(action);

    Status::Continue
}
