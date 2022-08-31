use bevy_ecs::prelude::*;

use crate::game::{component::*, Status, *};

pub fn behavior(
    mut action: ResMut<ActiveAction>,
    mut followups: ResMut<FollowUps>,
    weapons: Query<(Entity, &Item), (With<MeleeWeapon>, With<Equipped>)>,
    descriptions: Query<&Description>,
) -> Status {
    let mut action = match action.0.as_mut() {
        Some(Action::Melee(it)) => it,
        _ => return Status::Continue,
    };

    match weapons.iter().find_map(|(entity, item)| {
        item.owner
            .is_some_and(|owner| owner == &action.actor)
            .then_some(entity)
    }) {
        Some(weapon) => {
            action.weapon = Some(weapon);

            followups.0.push(Action::EndTurn(action.actor));

            Status::Continue
        }
        None => {
            let action = descriptions
                .get(action.actor)
                .ok()
                .map(|s| Action::Log(format!("{} has no melee weapon equipped", s)));

            Status::Reject(action)
        }
    }
}

pub fn hit(
    action: Res<ActiveAction>,
    descriptions: Query<&Description>,
    weapons: Query<&MeleeWeapon>,
    vulnerables: Query<(), With<Vulnerable>>,
    mut reactions: ResMut<Reactions>,
) -> Status {
    let (actor, target, weapon) = match &action.0 {
        Some(Action::Melee(MeleeAttackAction {
            actor,
            target,
            weapon: Some(weapon),
        })) => (actor, target, weapon),
        _ => return Status::Continue,
    };

    if vulnerables.contains(*target) {
        let damage = weapons.get(*weapon).unwrap().damage;

        let action = Action::Damage(DamageAction {
            actor: *actor,
            target: *target,
            weapon: *weapon,
            damage,
        });

        reactions.0.push(action);
    } else {
        if let (Ok(actor), Ok(target), Ok(weapon)) = (
            descriptions.get(*actor),
            descriptions.get(*target),
            descriptions.get(*weapon),
        ) {
            let action = Action::Log(format!(
                "{} strikes {} with {}, but it doesn't leave a scratch",
                actor, target, weapon
            ));

            reactions.0.push(action);
        }
    }

    Status::Continue
}
