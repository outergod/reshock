use bevy_ecs::prelude::*;

use crate::game::{component::*, *};

pub fn behavior(
    action: Res<Action>,
    mut reactions: ResMut<Reactions>,
    doors: Query<&Door>,
    gateways: Query<&Gateway>,
) -> Status {
    match action.as_ref() {
        Action::OpenDoor(OpenDoorAction { actor, target }) => {
            if let Ok(twin) = gateways.get(*target).map(|gateway| gateway.twin) {
                let door = doors.get(twin).unwrap();

                if !door.open {
                    reactions.0.push(Action::OpenDoor(OpenDoorAction {
                        actor: *actor,
                        target: twin,
                    }));
                }
            }
        }
        Action::CloseDoor(CloseDoorAction { actor, target }) => {
            if let Ok(twin) = gateways.get(*target).map(|gateway| gateway.twin) {
                let door = doors.get(twin).unwrap();

                if door.open {
                    reactions.0.push(Action::CloseDoor(CloseDoorAction {
                        actor: *actor,
                        target: twin,
                    }));
                }
            }
        }
        _ => {}
    }

    Status::Continue
}
