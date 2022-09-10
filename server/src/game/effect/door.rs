use bevy_ecs::prelude::*;
use bevy_hierarchy::Children;

use crate::game::Events;
use crate::game::{component::*, *};

pub fn open(
    action: Res<Action>,
    parents: Query<&Children, With<Door>>,
    mut doors: Query<(&mut Door, &DoorKind)>,
    mut commands: Commands,
    mut events: ResMut<Events>,
) {
    let OpenDoorAction { actor, target } = match action.as_ref() {
        Action::OpenDoor(it) => it,
        _ => return,
    };

    let (mut door, kind) = doors.get_mut(*target).unwrap();

    door.open = true;
    commands
        .entity(*target)
        .remove::<Solid>()
        .remove::<Opaque>();

    let doors = if let Ok(children) = parents.get(*target) {
        children.iter().map(|c| c.id()).collect()
    } else {
        vec![target.id()]
    };

    let sound = match kind {
        DoorKind::Heavy => api::door_event::DoorSound::Heavy,
        DoorKind::Bulkhead => api::door_event::DoorSound::Bulkhead,
    } as i32;

    events.0.push(api::Event {
        event: Some(api::event::Event::Door(api::DoorEvent {
            actor: actor.id(),
            doors,
            open: true,
            sound,
        })),
    });
}

pub fn close(
    action: Res<Action>,
    parents: Query<&Children, With<Door>>,
    mut doors: Query<(&mut Door, &DoorKind)>,
    mut commands: Commands,
    mut events: ResMut<Events>,
) {
    let CloseDoorAction { actor, target } = match action.as_ref() {
        Action::CloseDoor(it) => it,
        _ => return,
    };

    let (mut door, kind) = doors.get_mut(*target).unwrap();

    door.open = false;
    commands.entity(*target).insert(Solid).insert(Opaque);

    let doors = if let Ok(children) = parents.get(*target) {
        children.iter().map(|c| c.id()).collect()
    } else {
        vec![target.id()]
    };

    let sound = match kind {
        DoorKind::Heavy => api::door_event::DoorSound::Heavy,
        DoorKind::Bulkhead => api::door_event::DoorSound::Bulkhead,
    } as i32;

    events.0.push(api::Event {
        event: Some(api::event::Event::Door(api::DoorEvent {
            actor: actor.id(),
            doors,
            open: false,
            sound,
        })),
    });
}

pub fn propagate(
    parents: Query<(&Door, &Children)>,
    mut doors: Query<&mut Door, Without<Children>>,
    mut commands: Commands,
) {
    for (parent, children) in parents.iter() {
        for entity in children.iter() {
            let mut door = doors.get_mut(*entity).unwrap();

            if parent.open {
                door.open = true;
                commands
                    .entity(*entity)
                    .remove::<Solid>()
                    .remove::<Opaque>();
            } else {
                door.open = false;
                commands.entity(*entity).insert(Solid).insert(Opaque);
            }
        }
    }
}
