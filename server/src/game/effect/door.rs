use bevy_ecs::prelude::*;

use crate::game::Events;
use crate::game::{component::*, *};

pub fn open(
    action: Res<ActiveAction>,
    mut doors: Query<&mut Door>,
    mut commands: Commands,
    mut events: ResMut<Events>,
) {
    let OpenDoorAction { actor, target } = match &action.0 {
        Some(Action::OpenDoor(it)) => it,
        _ => return,
    };

    let mut door = doors.get_mut(*target).unwrap();

    door.open = true;
    commands
        .entity(*target)
        .remove::<Solid>()
        .remove::<Opaque>();

    events.0.push(api::Event {
        event: Some(api::event::Event::Door(api::DoorEvent {
            actor: actor.id(),
            door: target.id(),
            open: true,
        })),
    });
}

pub fn close(
    action: Res<ActiveAction>,
    mut doors: Query<&mut Door>,
    mut commands: Commands,
    mut events: ResMut<Events>,
) {
    let CloseDoorAction { actor, target } = match &action.0 {
        Some(Action::CloseDoor(it)) => it,
        _ => return,
    };

    let mut door = doors.get_mut(*target).unwrap();

    door.open = false;
    commands.entity(*target).insert(Solid).insert(Opaque);
    events.0.push(api::Event {
        event: Some(api::event::Event::Door(api::DoorEvent {
            actor: actor.id(),
            door: target.id(),
            open: false,
        })),
    });
}
