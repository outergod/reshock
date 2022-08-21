use std::collections::HashMap;

use bevy_ecs::prelude::*;
use glam::IVec2;

use crate::game::bundle::{Door, Floor, Player, Wall, NPC};
use crate::game::component;

pub struct Room(HashMap<IVec2, char>);

impl From<String> for Room {
    fn from(s: String) -> Self {
        let room = s
            .lines()
            .rev()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, c)| ((x as i32, y as i32).into(), c))
            })
            .collect();

        Self(room)
    }
}

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
                    description: "Serv-Bot unit".into(),
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
