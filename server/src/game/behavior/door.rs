use std::collections::HashSet;

use bevy_ecs::prelude::*;
use bevy_hierarchy::{Children, Parent};

use crate::game::{component::*, *};

pub fn behavior(
    action: Res<Action>,
    mut followups: ResMut<FollowUps>,
    blockers: Query<&Position, With<Solid>>,
    doors: Query<&Position, With<Door>>,
    parents: Query<&Children, With<Door>>,
    children: Query<&Parent, With<Door>>,
    player: Query<(), With<Player>>,
) -> Status {
    match action.as_ref() {
        Action::OpenDoor(OpenDoorAction { actor, target }) => {
            if let Ok(parent) = children.get(*target) {
                Status::Reject(vec![Action::OpenDoor(OpenDoorAction {
                    actor: *actor,
                    target: **parent,
                })])
            } else {
                followups.0.push(Action::EndTurn(*actor));
                Status::Continue
            }
        }
        Action::CloseDoor(CloseDoorAction { actor, target }) => {
            if let Ok(parent) = children.get(*target) {
                Status::Reject(vec![Action::CloseDoor(CloseDoorAction {
                    actor: *actor,
                    target: **parent,
                })])
            } else {
                let positions: HashSet<_> = if let Ok(children) = parents.get(*target) {
                    children
                        .iter()
                        .map(|entity| doors.get(*entity).unwrap())
                        .collect()
                } else {
                    [doors.get(*target).unwrap()].into_iter().collect()
                };

                if blockers.iter().any(|pos| positions.contains(pos)) {
                    let actions = if player.get(*actor).is_ok() {
                        vec![Action::Log("Door is blocked, can't close it".to_string())]
                    } else {
                        Vec::new()
                    };

                    Status::Reject(actions)
                } else {
                    followups.0.push(Action::EndTurn(*actor));
                    Status::Continue
                }
            }
        }
        _ => Status::Continue,
    }
}
