use std::collections::HashMap;

use bevy_ecs::prelude::*;

use crate::game::component::*;
use crate::game::event::ToggleDoor;

pub fn toggle(mut reader: EventReader<ToggleDoor>, mut doors: Query<(Entity, &mut Door)>) {
    let events: HashMap<_, _> = reader.iter().map(|e| (e.entity, e.open)).collect();

    for (entity, mut door) in doors.iter_mut() {
        if let Some(open) = events.get(&entity) {
            door.open = *open;
        }
    }
}

pub fn open(mut doors: Query<(&Door, &mut Opaque, &mut Obstacle)>) {
    for (door, mut opaque, mut obstacle) in doors.iter_mut() {
        if door.open {
            opaque.0 = false;
            obstacle.0 = false;
        } else {
            opaque.0 = true;
            obstacle.0 = true;
        }
    }
}
