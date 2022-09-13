use bevy_ecs::prelude::*;

use crate::game::{component::*, *};

pub fn behavior(
    action: Res<Action>,
    switches: Query<&Switch>,
    doors: Query<&Door>,
    locks: Query<&Lock>,
    mut reactions: ResMut<Reactions>,
    mut followups: ResMut<FollowUps>,
) -> Status {
    let ToggleSwitchAction { actor, target } = match action.as_ref() {
        Action::ToggleSwitch(it) => *it,
        _ => return Status::Continue,
    };

    let switch = switches.get(target).unwrap();

    for entity in switch.targets.iter().copied() {
        if let Ok(door) = doors.get(entity) {
            if door.open {
                reactions.0.push(Action::CloseDoor(CloseDoorAction {
                    actor,
                    target: entity,
                }));
            } else {
                reactions.0.push(Action::OpenDoor(OpenDoorAction {
                    actor,
                    target: entity,
                }));
            }
        }

        if let Ok(lock) = locks.get(entity) {
            if lock.active {
                reactions
                    .0
                    .push(Action::DeactivateLock(DeactivateLockAction {
                        actor,
                        target: entity,
                    }));
            } else {
                reactions.0.push(Action::ActivateLock(ActivateLockAction {
                    actor,
                    target: entity,
                }));
            }
        }
    }

    followups.0.push(Action::EndTurn(actor));

    Status::Continue
}
