use crate::game::{bundle, component};

use super::{loader::*, *};

pub struct HibernationRoom;

impl RoomLoader for HibernationRoom {
    fn source() -> String {
        load_asset("hibernation.room")
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

                Tile::Player => {
                    let player = commands
                        .spawn()
                        .insert_bundle(bundle::Player {
                            position,
                            ..Default::default()
                        })
                        .id();

                    commands
                        .spawn()
                        .insert_bundle(bundle::MeleeWeapon::laser_rapier())
                        .insert(component::Item {
                            owner: Some(player),
                        })
                        .insert(component::Equipped);

                    let rifle = commands
                        .spawn()
                        .insert_bundle(bundle::ProjectileGun::assault_rifle())
                        .insert(component::Item {
                            owner: Some(player),
                        })
                        .insert(component::Equipped)
                        .id();

                    let mut magazine = bundle::Magazine::magnesium_tips();
                    magazine.magazine.attached = Some(rifle);
                    commands.spawn().insert_bundle(magazine);
                }

                _ => {}
            }
        }
    }
}
