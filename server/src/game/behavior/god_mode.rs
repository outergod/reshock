use bevy_ecs::prelude::*;

use crate::game::{component::*, *};

pub fn behavior(
    mut action: ResMut<ActiveAction>,
    player: Query<(Entity, Option<&God>), With<Player>>,
) -> Status {
    let (player, god) = player.single();

    match action.0.as_mut() {
        Some(Action::GodMode(action)) => {
            *action = Some(GodModeAction {
                actor: player,
                activate: god.is_none(),
            });

            Status::Continue
        }
        Some(Action::View(Some(ViewAction { actor, sight }))) => {
            if actor == &player && god.is_some() {
                sight.kind = SightKind::Omniscience;
            }

            Status::Continue
        }
        Some(Action::HealthLoss(HealthLossAction { actor, amount })) => {
            if actor == &player && god.is_some() {
                *amount = 0;
            }

            Status::Continue
        }
        Some(Action::Death(DeathAction { actor, .. })) => {
            if actor == &player && god.is_some() {
                Status::Reject(None)
            } else {
                Status::Continue
            }
        }
        _ => Status::Continue,
    }
}
