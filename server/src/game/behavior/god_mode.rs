use bevy_ecs::prelude::*;

use crate::game::{component::*, *};

pub fn behavior(
    mut action: ResMut<ActiveAction>,
    player: Query<(Entity, Option<&God>), With<Player>>,
) -> Status {
    match action.0.as_mut() {
        Some(Action::GodMode(action)) => {
            let (player, god) = player.single();

            *action = Some(GodModeAction {
                entity: player,
                activate: god.is_none(),
            });

            Status::Accept
        }
        _ => Status::Continue,
    }
}
