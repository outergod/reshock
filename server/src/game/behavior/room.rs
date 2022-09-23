use std::collections::HashSet;
use std::convert::TryInto;
use std::ops::Not;

use bevy_ecs::prelude::*;

use crate::game::resource::*;
use crate::game::room::{Room, Rooms};
use crate::game::{component::*, *};

pub fn behavior(
    action: Res<Action>,
    rooms: Res<Rooms>,
    room_index: Res<RoomId>,
    spawners: Query<(), With<RoomSpawner>>,
    positions: Query<&Position>,
    // bulkhead_doors: Query<&Children, With<Door>>,
) -> Status {
    let target = match action.as_ref() {
        Action::OpenDoor(OpenDoorAction { target, .. }) if spawners.contains(*target) => target,
        _ => return Status::Continue,
    };

    // let start = match bulkhead_doors.get(*target) {
    //     Ok(children) => positions.get(children[0]).unwrap(),
    //     Err(_) => positions.get(*target).unwrap(),
    // };

    let position = positions.get(*target).unwrap();

    let coordinates: HashSet<_> = positions
        .iter()
        .filter_map(|p| (p.room == position.room).then_some(p.coordinates))
        .collect();

    let direction = Deltas::cross()
        .0
        .into_iter()
        .find_map(|delta| -> Option<Direction> {
            let pos = position.coordinates + delta;
            coordinates
                .contains(&pos)
                .not()
                .then_some((delta.x, delta.y))
                .and_then(|vec| vec.try_into().ok())
        })
        .unwrap()
        .reverse();

    // Prevent the map turning into a dead end
    let predicate = if spawners.iter().len() > 1 {
        |_room: &Room| true
    } else {
        |room: &Room| !room.is_dead_end()
    };

    let mut rng = thread_rng();
    let mut room = match rooms.random(&mut rng, predicate) {
        Some(it) => it,
        None => {
            log::error!("Could not find a room fulfilling the predicate");
            return Status::Reject(vec![]);
        }
    };

    let spawner = room.random_spawner(&mut rng).unwrap().to_owned();

    room.turn_towards(&spawner, &direction).unwrap();
    room.erase_player();
    room.release_spawner(&spawner);

    let id = room_index.next();

    let coordinates = room.position_of(&spawner).unwrap().to_owned();

    let spawn = Action::SpawnRoom(RoomSpawnAction {
        target: *target,
        id,
        room,
    });

    let gateway = Action::SpawnGateway(GatewaySpawnAction {
        lhs: *position,
        rhs: Position {
            coordinates,
            room: id,
        },
        direction: direction.reverse(),
    });

    Status::Reject(vec![spawn, gateway, action.to_owned()])
}
