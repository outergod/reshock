use std::collections::HashMap;

use bevy_ecs::prelude::*;
use glam::IVec2;

use super::bundle::{Door, Floor, Player, Wall, NPC};
use super::component;

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

        match c {
            '@' => {
                world.spawn().insert_bundle(Player {
                    position: component::Position(*pos),
                    ..Default::default()
                });
            }
            'b' => {
                world.spawn().insert_bundle(NPC {
                    position: component::Position(*pos),
                    ai: component::AI::ServBot,
                    renderable: component::Renderable::ServBot,
                    ..Default::default()
                });
            }
            'X' => {
                world.spawn().insert_bundle(Wall {
                    position: component::Position(*pos),
                    ..Default::default()
                });
            }
            'O' => {
                world.spawn().insert_bundle(Door {
                    door: component::Door {
                        open: true,
                        ..Default::default()
                    },
                    position: component::Position(*pos),
                    ..Default::default()
                });
            }
            'o' => {
                world.spawn().insert_bundle(Door {
                    door: component::Door {
                        open: false,
                        ..Default::default()
                    },
                    position: component::Position(*pos),
                    ..Default::default()
                });
            }
            '·' | ' ' => {}
            _ => {
                log::error!("Unknown room char {}", c);
            }
        }
    }
}
