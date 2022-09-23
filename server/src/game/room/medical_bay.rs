use super::{loader::*, *};

pub struct MedicalBayRoom;

impl RoomLoader for MedicalBayRoom {
    fn source() -> String {
        load_asset("medical-bay.room")
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

                Tile::NPC(NPC::ServBot) => {
                    let npc = commands
                        .spawn()
                        .insert_bundle(bundle::NPC {
                            position,
                            ai: component::AI::ServBot,
                            renderable: component::Renderable::ServBot,
                            sight: component::Sight {
                                kind: component::SightKind::Eyes,
                                ..Default::default()
                            },
                            description: component::Description {
                                name: "Serv-Bot unit".into(),
                                article: component::Article::A,
                            },
                            vulnerable: component::Vulnerable {
                                kind: component::VulnerableKind::Robot,
                                hp: 20,
                                max: 20,
                                defense: 2,
                                armor: 20,
                            },
                            ..Default::default()
                        })
                        .insert(component::Alive::ServBot)
                        .id();

                    commands
                        .spawn()
                        .insert_bundle(bundle::NaturalMeleeWeapon::appendages())
                        .insert(component::Item { owner: Some(npc) })
                        .insert(component::Equipped);
                }

                _ => {}
            }
        }
    }
}
