use bevy_ecs::prelude::*;

use crate::game::{component::*, *};

pub fn behavior(
    action: Res<Action>,
    mut followups: ResMut<FollowUps>,
    blockers: Query<&Position, With<Solid>>,
    doors: Query<&Position, With<Door>>,
) -> Status {
    match action.as_ref() {
        Action::OpenDoor(OpenDoorAction { actor, .. }) => {
            followups.0.push(Action::EndTurn(*actor));
            Status::Continue
        }
        Action::CloseDoor(CloseDoorAction { actor, target }) => {
            let position = match doors.get(*target) {
                Ok(it) => it,
                Err(_) => {
                    log::warn!("Tried to close door {:?} without Position", target);
                    return Status::Reject(vec![]);
                }
            };

            if blockers.iter().any(|pos| pos == position) {
                let action = Action::Log("Door is blocked, can't close it".to_string());
                Status::Reject(vec![action])
            } else {
                followups.0.push(Action::EndTurn(*actor));
                Status::Continue
            }
        }
        _ => Status::Continue,
    }
}
