use bevy_ecs::prelude::*;

use crate::game::{Action, ActiveAction, FollowUps, Status};

pub fn behavior(action: Res<ActiveAction>, mut followups: ResMut<FollowUps>) -> Status {
    match &action.0 {
        Some(Action::OpenDoor(_)) => {
            followups.0.push(Action::EndTurn);
            Status::Accept
        }
        _ => Status::Continue,
    }
}
