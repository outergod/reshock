use super::{loader::*, *};

pub struct StorageRoom;

impl RoomLoader for StorageRoom {
    fn source() -> String {
        load_asset("storage.room")
    }

    fn spawn(room: &Room, commands: &mut Commands) {
        for (id, pos) in room.positions.iter() {
            let position = component::Position(*pos);

            commands.spawn().insert_bundle(bundle::Floor {
                position: position.clone(),
                ..Default::default()
            });

            match room.tiles.get(id).unwrap() {
                Tile::Wall => {
                    commands.spawn().insert_bundle(bundle::Wall {
                        position,
                        ..Default::default()
                    });
                }

                Tile::Door(kind) => {
                    let mut door = commands.spawn();

                    door.insert_bundle(bundle::Door {
                        position,
                        ..Default::default()
                    })
                    .insert(component::DoorKind::Heavy);

                    match kind {
                        Door::Open => {
                            door.insert(component::Door { open: true });
                        }
                        Door::Closed => {
                            door.insert(component::Door { open: false })
                                .insert(component::Solid)
                                .insert(component::Opaque);
                        }
                        Door::Spawner => {
                            door.insert(component::Door { open: false })
                                .insert(component::Solid)
                                .insert(component::Opaque)
                                .insert(component::RoomSpawner);
                        }
                    }
                }

                _ => {}
            }
        }
    }
}
