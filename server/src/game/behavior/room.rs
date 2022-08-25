use bevy_ecs::prelude::*;

use crate::game::bundle::{Door, Floor, Player, Wall, NPC};
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
                world.spawn().insert_bundle(Player {
                    position,
                    ..Default::default()
                });
            }
            'b' => {
                world.spawn().insert_bundle(NPC {
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
                    ..Default::default()
                });
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
