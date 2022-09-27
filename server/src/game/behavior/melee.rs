use bevy_ecs::prelude::*;

use crate::game::{component::*, Status, *};

pub fn intent(
    action: Res<Action>,
    mut reactions: ResMut<Reactions>,
    mut followups: ResMut<FollowUps>,
    weapons: Query<(Entity, &Item), (With<Equipped>, With<MeleeWeapon>)>,
    descriptions: Query<&Description>,
) -> Status {
    let (actor, direction) = match action.as_ref() {
        Action::Melee(MeleeAttackAction::Intent { actor, direction }) => (*actor, direction),
        _ => return Status::Continue,
    };

    match weapons.iter().find_map(|(entity, item)| {
        item.owner
            .is_some_and(|owner| owner == &actor)
            .then_some(entity)
    }) {
        Some(weapon) => {
            let action = Action::Melee(MeleeAttackAction::Attack {
                actor,
                direction: *direction,
                weapon,
            });
            reactions.0.push(action);

            followups.0.push(Action::EndTurn(actor));

            Status::Continue
        }
        None => {
            let mut actions = Vec::new();

            if let Ok(s) = descriptions.get(actor) {
                actions.push(Action::Log(format!("{} has no melee weapon equipped", s)));
            };

            Status::Reject(actions)
        }
    }
}

pub fn attack(
    action: Res<Action>,
    mut reactions: ResMut<Reactions>,
    positions: Query<&Position>,
    weapons: Query<&MeleeWeapon>,
    gateways: Query<(&Position, &Gateway)>,
    obstacles: Query<(Entity, &Position), With<Solid>>,
) -> Status {
    let (actor, direction, weapon) = match action.as_ref() {
        Action::Melee(MeleeAttackAction::Attack {
            actor,
            direction,
            weapon,
        }) => (*actor, *direction, *weapon),
        _ => return Status::Continue,
    };

    let position = positions.get(actor).unwrap();
    let target = *position + direction.to_owned().into();

    let targets = match gateways.iter().find(|(pos, _)| *pos == &target) {
        Some((pos, gateway)) => {
            let twin = gateways.get(gateway.twin).unwrap().0;
            vec![*pos, *twin]
        }
        None => vec![target],
    }
    .into_iter()
    .find_map(|target| {
        obstacles
            .iter()
            .find_map(|(entity, pos)| (pos == &target).then_some(entity))
    });

    let damage = weapons.get(weapon).unwrap().damage;

    // TODO hit chance etc

    for target in targets {
        let action = Action::Hit(HitAction {
            actor,
            target,
            direction,
            weapon,
            damage,
        });
        reactions.0.push(action);
    }

    Status::Continue
}
