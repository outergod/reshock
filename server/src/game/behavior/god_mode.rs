use bevy_ecs::prelude::*;

use crate::game::{component::*, *};

pub fn behavior(
    mut action: ResMut<Action>,
    mut reactions: ResMut<Reactions>,
    player: Query<(Entity, Option<&God>), With<Player>>,
) -> Status {
    let (player, god) = match player.get_single() {
        Ok(it) => it,
        Err(_) => return Status::Continue,
    };

    match action.as_mut() {
        Action::GodMode(GodModeAction::Intent) => {
            reactions.0.push(Action::GodMode(GodModeAction::Activate {
                actor: player,
                activate: god.is_none(),
            }));

            Status::Continue
        }
        Action::HealthLoss(HealthLossAction { actor, amount }) => {
            if actor == &player && god.is_some() {
                *amount = 0;
            }

            Status::Continue
        }
        Action::Death(DeathAction { actor, .. }) => {
            if actor == &player && god.is_some() {
                Status::Reject(vec![])
            } else {
                Status::Continue
            }
        }
        _ => Status::Continue,
    }
}
