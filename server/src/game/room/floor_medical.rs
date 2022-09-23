use crate::game::{bundle, component};

use super::{loader::*, *};

pub struct FloorMedicalRoom;

impl RoomLoader for FloorMedicalRoom {
    fn source() -> String {
        load_asset("floor-medical.room")
    }

    fn spawn(room: &Room, room_id: RoomId, commands: &mut Commands) {
        let bulkhead_doors =
            room.bulkhead_doors
                .iter()
                .fold(HashMap::new(), |mut map, (left, right)| {
                    let door = commands
                        .spawn()
                        .insert(component::Description {
                            name: "bulkhead door".to_string(),
                            article: component::Article::A,
                        })
                        .id();
                    map.insert(*left, door);
                    map.insert(*right, door);
                    map
                });

        for (id, pos) in room.positions.iter() {
            let position = component::Position {
                coordinates: *pos,
                room: room_id,
            };

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
                    let door = commands
                        .spawn()
                        .insert_bundle(bundle::Door {
                            position,
                            ..Default::default()
                        })
                        .id();

                    let mut door = match bulkhead_doors.get(id) {
                        Some(entity) => {
                            let mut entity = commands.entity(*entity);
                            entity.insert(component::DoorKind::Bulkhead);
                            entity.add_child(door);
                            entity
                        }
                        None => {
                            let mut entity = commands.entity(door);
                            entity.insert(component::DoorKind::Heavy);
                            entity
                        }
                    };

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
