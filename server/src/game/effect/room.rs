use bevy_ecs::prelude::*;

use crate::game::{component::*, *};

pub fn effect(action: Res<Action>, mut commands: Commands) {
    let RoomSpawnAction { target, room } = match action.as_ref() {
        Action::SpawnRoom(it) => it,
        _ => return,
    };

    commands.entity(*target).remove::<RoomSpawner>();
    room.spawn(&mut commands);
}
