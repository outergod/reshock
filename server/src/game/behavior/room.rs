use bevy_ecs::prelude::*;
use bevy_hierarchy::Children;

use crate::game::resource::*;
use crate::game::room::{FindSite, Room};
use crate::game::{component::*, *};

pub fn behavior(
    action: Res<Action>,
    spawners: Query<(), With<RoomSpawner>>,
    player: Query<(), With<Player>>,
    positions: Query<&Position>,
    spatial: Res<SpatialHash>,
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

    let mut rng = thread_rng();
    let room: Room = rng.gen();
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
