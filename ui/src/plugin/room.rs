use bevy::log;
use bevy::prelude::*;

use crate::asset::Room;
use crate::asset::RoomLoader;
use crate::bundle::Door;
use crate::bundle::NPC;
use crate::bundle::{Floor, Player, Tile, Wall};
use crate::component;
use crate::component::Position;
use crate::component::Renderable;
use crate::component::AI;
use crate::resource::ReshockFont;

pub struct RoomPlugin;

impl Plugin for RoomPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<Room>()
            .init_asset_loader::<RoomLoader>()
            .add_system(loaded);
    }
}

fn loaded(
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
                        commands.spawn_bundle(Floor {
                            position: Position(*pos),
                            ..Default::default()
                        });
                    }

                    match c {
                        '@' => {
                            commands.spawn_bundle(Player {
                                position: Position(*pos),
                                ..Default::default()
                            });
                        }
                        'b' => {
                            commands.spawn_bundle(NPC {
                                position: Position(*pos),
                                ai: AI::ServBot,
                                renderable: Renderable {
                                    char: 'b',
                                    color: Color::ORANGE_RED,
                                },
                                ..Default::default()
                            });
                        }
                        'X' => {
                            commands.spawn_bundle(Wall {
                                position: Position(*pos),
                                ..Default::default()
                            });
                        }
                        'O' => {
                            commands.spawn_bundle(Door {
                                door: component::Door {
                                    open_color: Color::DARK_GRAY,
                                    close_color: Color::WHITE,
                                    open: true,
                                    ..Default::default()
                                },
                                position: Position(*pos),
                                ..Default::default()
                            });
                        }
                        'o' => {
                            commands.spawn_bundle(Door {
                                door: component::Door {
                                    open_color: Color::DARK_GRAY,
                                    close_color: Color::WHITE,
                                    open: false,
                                    ..Default::default()
                                },
                                position: Position(*pos),
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
            AssetEvent::Modified { .. } | AssetEvent::Removed { .. } => (),
        }
    }
}
