use bevy_ecs::prelude::*;

use crate::game::bundle::*;
use crate::game::component;
use crate::game::resource::Room;

pub fn setup(world: &mut World, room: Room) {
    for (pos, c) in room.0.iter() {
        if *c != ' ' {
            world.spawn().insert_bundle(Floor {
                position: component::Position(*pos),
                ..Default::default()
            });
        }

        let position = component::Position(*pos);

        match c {
            '@' => {
                let player = world
                    .spawn()
                    .insert_bundle(Player {
                        position,
                        ..Default::default()
                    })
                    .id();

                world
                    .spawn()
                    .insert_bundle(MeleeWeapon::laser_rapier())
                    .insert(component::Item {
                        owner: Some(player),
                    })
                    .insert(component::Equipped);

                let rifle = world
                    .spawn()
                    .insert_bundle(ProjectileGun::assault_rifle())
                    .insert(component::Item {
                        owner: Some(player),
                    })
                    .insert(component::Equipped)
                    .id();

                let mut magazine = Magazine::magnesium_tips();
                magazine.magazine.attached = Some(rifle);
                world.spawn().insert_bundle(magazine);
            }
            'b' => {
                let npc = world
                    .spawn()
                    .insert_bundle(NPC {
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

                world
                    .spawn()
                    .insert_bundle(NaturalMeleeWeapon::appendages())
                    .insert(component::Item { owner: Some(npc) })
                    .insert(component::Equipped);
            }
            'X' => {
                world.spawn().insert_bundle(Wall {
                    position,
                    ..Default::default()
                });
            }
            'O' => {
                world.spawn().insert_bundle(Door {
                    position,
                    door: component::Door {
                        open: true,
                        ..Default::default()
                    },
                    ..Default::default()
                });
            }
            'o' => {
                world
                    .spawn()
                    .insert_bundle(Door {
                        position,
                        door: component::Door {
                            open: false,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(component::Solid)
                    .insert(component::Opaque);
            }
            '·' | ' ' => {}
            _ => {
                log::error!("Unknown room char {}", c);
            }
        }
    }
}
