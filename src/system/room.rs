use bevy::log;
use bevy::prelude::*;

use crate::asset::Room;
use crate::bundle::Door;
use crate::bundle::{Floor, Player, Tile, Wall};
use crate::component::Position;
use crate::resource::ReshockFont;

pub fn loaded(
    mut commands: Commands,
    mut events: EventReader<AssetEvent<Room>>,
    font: Res<ReshockFont>,
    rooms: ResMut<Assets<Room>>,
) {
    for event in events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                let room = rooms.get(handle).unwrap();

                for (pos, c) in room.0.iter() {
                    if *c != ' ' {
                        commands.spawn_bundle(Tile::new(Position(*pos), &font));
                        commands.spawn_bundle(Floor::new(Position(*pos)));
                    }

                    match c {
                        '@' => {
                            commands.spawn_bundle(Player::new(Position(*pos)));
                        }
                        'X' => {
                            commands.spawn_bundle(Wall::new(Position(*pos)));
                        }
                        'O' => {
                            commands.spawn_bundle(Door::new(Position(*pos), true));
                        }
                        'o' => {
                            commands.spawn_bundle(Door::new(Position(*pos), false));
                        }
                        '·' | ' ' => {}
                        _ => {
                            log::error!("Unknown room char {}", c);
                        }
                    }
                }
            }
            AssetEvent::Modified { .. } | AssetEvent::Removed { .. } => (),
        }
    }
}
