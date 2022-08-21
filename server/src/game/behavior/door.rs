use bevy_ecs::prelude::*;

use crate::game::{Action, ActiveAction, FollowUps, OpenDoorAction, Status};

pub fn behavior(action: Res<ActiveAction>, mut followups: ResMut<FollowUps>) -> Status {
    match &action.0 {
        Some(Action::OpenDoor(OpenDoorAction { actor, .. })) => {
            followups.0.push(Action::EndTurn(*actor));
            Status::Accept
        }
        _ => Status::Continue,
    }
}
