use bevy_ecs::prelude::*;

use crate::game::{component::*, Status, *};

pub fn intent(
    action: Res<Action>,
    weapons: Query<(Entity, &Item), (With<Equipped>, With<MeleeWeapon>)>,
    descriptions: Query<&Description>,
    mut reactions: ResMut<Reactions>,
    mut followups: ResMut<FollowUps>,
) -> Status {
    let (actor, target, direction) = match action.as_ref() {
        Action::Melee(MeleeAttackAction::Intent {
            actor,
            target,
            direction,
        }) => (actor, target, direction),
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
                direction: *direction,
                weapon,
            });
            reactions.0.push(action);

            followups.0.push(Action::EndTurn(*actor));

            Status::Continue
        }
        None => {
            let mut actions = Vec::new();

            if let Ok(s) = descriptions.get(*actor) {
                actions.push(Action::Log(format!("{} has no melee weapon equipped", s)));
            };

            Status::Reject(actions)
        }
    }
}

pub fn attack(
    action: Res<Action>,
    weapons: Query<&MeleeWeapon>,
    mut reactions: ResMut<Reactions>,
) -> Status {
    let (actor, target, direction, weapon) = match action.as_ref() {
        Action::Melee(MeleeAttackAction::Attack {
            actor,
            target,
            direction,
            weapon,
        }) => (*actor, *target, *direction, *weapon),
        _ => return Status::Continue,
    };

    let damage = weapons.get(weapon).unwrap().damage;

    // TODO hit chance, use position instead of target etc

    let action = Action::Hit(HitAction {
        actor,
        target,
        direction,
        weapon,
        damage,
    });
    reactions.0.push(action);

    Status::Continue
}
