use bevy_ecs::prelude::*;

use crate::game::{component::*, Status, *};

pub fn intent(
    action: Res<ActiveAction>,
    weapons: Query<(Entity, &Item), (With<Equipped>, With<MeleeWeapon>)>,
    descriptions: Query<&Description>,
    mut reactions: ResMut<Reactions>,
    mut followups: ResMut<FollowUps>,
) -> Status {
    let (actor, target) = match &action.0 {
        Some(Action::Melee(MeleeAttackAction::Intent { actor, target })) => (actor, target),
        _ => return Status::Continue,
    };

    match weapons.iter().find_map(|(entity, item)| {
        item.owner
            .is_some_and(|owner| owner == actor)
            .then_some(entity)
    }) {
        Some(weapon) => {
            let action = Action::Melee(MeleeAttackAction::Attack {
                actor: *actor,
                target: *target,
                weapon,
            });
            reactions.0.push(action);

            followups.0.push(Action::EndTurn(*actor));

            Status::Continue
        }
        None => {
            let action = descriptions
                .get(*actor)
                .ok()
                .map(|s| Action::Log(format!("{} has no melee weapon equipped", s)));

            Status::Reject(action)
        }
    }
}

pub fn attack(
    action: Res<ActiveAction>,
    weapons: Query<&MeleeWeapon>,
    mut reactions: ResMut<Reactions>,
) -> Status {
    let (actor, target, weapon) = match &action.0 {
        Some(Action::Melee(MeleeAttackAction::Attack {
            actor,
            target,
            weapon,
        })) => (actor, target, weapon),
        _ => return Status::Continue,
    };

    let damage = weapons.get(*weapon).unwrap().damage;

    // TODO hit chance, use position instead of target etc

    let action = Action::Hit(HitAction {
        actor: *actor,
        target: *target,
        weapon: *weapon,
        damage,
    });
    reactions.0.push(action);

    Status::Continue
}
