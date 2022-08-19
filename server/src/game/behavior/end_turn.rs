use bevy_ecs::prelude::*;

use crate::game::{component::*, *};

pub fn behavior(
    action: Res<ActiveAction>,
    mut followups: ResMut<FollowUps>,
    ai: Query<Entity, With<AI>>,
) -> Status {
    match action.0 {
        Some(Action::EndTurn) => {}
        _ => {
            return Status::Continue;
        }
    }

    for entity in ai.iter() {
        followups.0.push(Action::AI(entity));
    }

    Status::Accept
}
