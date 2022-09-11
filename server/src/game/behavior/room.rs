use bevy_ecs::prelude::*;
use bevy_hierarchy::Children;

use crate::game::resource::*;
use crate::game::room::{FindSite, Room, Rooms};
use crate::game::{component::*, *};

pub fn behavior(
    action: Res<Action>,
    spatial: Res<SpatialHash>,
    rooms: Res<Rooms>,
    spawners: Query<(), With<RoomSpawner>>,
    player: Query<(), With<Player>>,
    positions: Query<&Position>,
    bulkhead_doors: Query<&Children, With<Door>>,
) -> Status {
    let target = match action.as_ref() {
        Action::OpenDoor(OpenDoorAction { target, .. }) if spawners.contains(*target) => target,
        _ => return Status::Continue,
    };

    let start = match bulkhead_doors.get(*target) {
        Ok(children) => positions.get(children[0]).unwrap(),
        Err(_) => positions.get(*target).unwrap(),
    };

    let direction = Deltas::cross()
        .0
        .into_iter()
        .find_map(|delta| {
            let pos = start.0 + delta;
            spatial.entities_at(&pos).is_empty().then_some(pos)
        })
        .unwrap();

    log::debug!("{:?} {:?}", start, direction);

    // Prevent the map turning into a dead end
    let predicate = if spawners.iter().len() > 1 {
        None
    } else {
        Some(|room: &Room| !room.is_dead_end())
    };

    let mut rng = thread_rng();
    let room = match rooms.random(&mut rng, predicate) {
        Some(it) => it,
        None => {
            log::error!("Could not find a room fulfilling the predicate");
            return Status::Reject(vec![]);
        }
    };

    let find = FindSite::new(spatial.to_owned());

    let mut room = find.find_site(&room, (&start.0, &direction)).unwrap();

    if !player.is_empty() {
        room.erase_player();
    }

    let spawn = Action::SpawnRoom(RoomSpawnAction {
        target: *target,
        room,
    });

    Status::Reject(vec![spawn, action.to_owned()])
}
