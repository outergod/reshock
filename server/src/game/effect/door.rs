use bevy_ecs::prelude::*;

use crate::game::Events;
use crate::game::{component::*, *};

pub fn open(
    action: Res<ActiveAction>,
    mut doors: Query<&mut Door>,
    mut commands: Commands,
    mut events: ResMut<Events>,
) {
    let OpenDoorAction { actor, entity } = match &action.0 {
        Some(Action::OpenDoor(it)) => it,
        _ => return,
    };

    match doors.get_mut(*entity) {
        Ok(mut door) => {
            door.open = true;
            commands
                .entity(*entity)
                .remove::<Solid>()
                .remove::<Opaque>();
            events.0.push(api::Event {
                event: Some(api::event::Event::Door(api::DoorEvent {
                    actor: actor.id(),
                    door: entity.id(),
                    open: true,
                })),
            });
        }
        Err(_) => {
            log::warn!(
                "Invalid open door action, entity {:?} does not have Door component",
                entity
            );
        }
    }
}

pub fn close(
    action: Res<ActiveAction>,
    mut doors: Query<&mut Door>,
    mut commands: Commands,
    mut events: ResMut<Events>,
) {
    let CloseDoorAction { actor, entity } = match &action.0 {
        Some(Action::CloseDoor(it)) => it,
        _ => return,
    };

    match doors.get_mut(*entity) {
        Ok(mut door) => {
            door.open = false;
            commands.entity(*entity).insert(Solid).insert(Opaque);
            events.0.push(api::Event {
                event: Some(api::event::Event::Door(api::DoorEvent {
                    actor: actor.id(),
                    door: entity.id(),
                    open: false,
                })),
            });
        }
        Err(_) => {
            log::warn!(
                "Invalid close door action, entity {:?} does not have Door component",
                entity
            );
        }
    }
}
