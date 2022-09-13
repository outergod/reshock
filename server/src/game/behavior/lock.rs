use bevy_ecs::prelude::*;

use crate::game::{component::*, *};

pub fn door(
    action: Res<Action>,
    player: Query<(), With<Player>>,
    locks: Query<&Lock>,
    descriptions: Query<&Description>,
) -> Status {
    let OpenDoorAction { actor, target } = match action.as_ref() {
        Action::OpenDoor(it) => *it,
        _ => return Status::Continue,
    };

    if locks
        .iter()
        .any(|lock| lock.locked.contains(&target) && lock.active)
    {
        let mut actions = Vec::new();

        if player.contains(actor) && let Ok(desc) = descriptions.get(target) {
            actions.push(Action::Log(format!("{} is locked and can't be opened directly", desc)));
        };

        Status::Reject(actions)
    } else {
        Status::Continue
    }
}

pub fn close(
    action: Res<Action>,
    locks: Query<&Lock>,
    doors: Query<&Door>,
    mut reactions: ResMut<Reactions>,
) -> Status {
    let ActivateLockAction { actor, target } = match action.as_ref() {
        Action::ActivateLock(it) => *it,
        _ => return Status::Continue,
    };

    let lock = locks.get(target).unwrap();

    for target in lock.locked.iter().cloned() {
        if doors.get(target).is_ok_and(|door| door.open) {
            reactions
                .0
                .push(Action::CloseDoor(CloseDoorAction { actor, target }));
        }
    }

    Status::Continue
}
