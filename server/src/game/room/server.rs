use crate::game::{bundle, component};

use super::{loader::*, *};

pub struct ServerRoom;

impl RoomLoader for ServerRoom {
    fn source() -> String {
        load_asset("server.room")
    }

    fn spawn(room: &Room, room_id: RoomId, commands: &mut Commands) {
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

                Tile::Object(_) => {
                    commands
                        .spawn()
                        .insert_bundle(bundle::Object {
                            renderable: component::Renderable::Server,
                            position,
                            description: component::Description {
                                name: "server node".to_string(),
                                article: component::Article::A,
                            },
                        })
                        .insert(component::Solid)
                        .insert(component::Vulnerable {
                            kind: component::VulnerableKind::Robot,
                            hp: 50,
                            max: 50,
                            defense: 0,
                            armor: 0,
                        })
                        .insert(component::Destructible::Server);
                }

                _ => {}
            }
        }
    }
}
