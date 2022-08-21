use bevy_ecs::prelude::*;

use crate::game::{component::*, *};

pub fn behavior(
    action: Res<ActiveAction>,
    mut followups: ResMut<FollowUps>,
    player: Query<(), With<Player>>,
    ai: Query<Entity, With<AI>>,
) -> Status {
    let entity = match action.0 {
        Some(Action::EndTurn(it)) => it,
        _ => {
            return Status::Continue;
        }
    };

    if player.contains(entity) {
        for entity in ai.iter() {
            followups.0.push(Action::AI(entity));
        }
    }

    Status::Accept
}
