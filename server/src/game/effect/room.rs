use bevy_ecs::prelude::*;

use crate::game::{component::*, *};

pub fn effect(action: Res<Action>, mut room_index: ResMut<RoomId>, mut commands: Commands) {
    let RoomSpawnAction { target, id, room } = match action.as_ref() {
        Action::SpawnRoom(it) => it,
        _ => return,
    };

    room.spawn(*id, &mut commands);
    *room_index = *id;
    commands.entity(*target).remove::<RoomSpawner>();
}
