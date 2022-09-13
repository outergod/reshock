use crate::game::{bundle, component};

use super::{loader::*, *};

pub struct CyberspaceCacheRoom;

impl RoomLoader for CyberspaceCacheRoom {
    fn source() -> String {
        load_asset("cyberspace-cache.room")
    }

    fn char_tile(c: char) -> Option<Tile> {
        match c {
            '#' | 'x' => Some(Tile::Wall),
            '1' => Some(Tile::Door(Door::Closed)),
            '-' | '|' => Some(Tile::Door(Door::Spawner)),
            '·' => Some(Tile::Floor),
            ' ' => None,
            _ => {
                log::error!("Unknown room char {}", c);
                None
            }
        }
    }

    fn spawn(room: &Room, commands: &mut Commands) {
        let mut locked = Vec::new();

        let lock = commands.spawn().id();

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

                    commands.spawn().insert_bundle(bundle::Switch {
                        position,
                        renderable: component::Renderable::WallSwitch,
                        switch: component::Switch {
                            targets: vec![lock],
                        },
                        description: component::Description {
                            name: "wall switch".to_string(),
                            article: component::Article::A,
                        },
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

                    if room.chars.get(id).unwrap() == &'1' {
                        locked.push(door.id());
                    }
                }

                _ => {}
            }
        }

        commands.entity(lock).insert(component::Lock {
            active: true,
            locked,
        });
    }
}
