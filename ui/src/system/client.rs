use api::reshock_client::ReshockClient;
use api::StateDumpResponse;
use bevy::log;
use bevy::prelude::*;
use bevy::utils::HashSet;
use tokio::runtime::Runtime;
use tonic::transport::Channel;

use crate::bundle;
use crate::component;
use crate::resource::ReshockFont;

pub fn setup(
    mut commands: Commands,
    runtime: Res<Runtime>,
    mut client: ResMut<ReshockClient<Channel>>,
    font: Res<ReshockFont>,
) {
    runtime.block_on(async move {
        match client.dump_state(api::Empty {}).await {
            Ok(response) => {
                let StateDumpResponse { entities } = response.into_inner();
                let mut positions = HashSet::new();

                for entity in entities {
                    let mut e = commands.spawn();

                    e.insert(component::ReshockEntity(entity.entity));

                    if entity.player.is_some() {
                        e.insert(component::Player);
                    }

                    if entity.wall.is_some() {
                        e.insert(component::Wall);
                    }

                    if entity.room.is_some() {
                        e.insert(component::Room);
                    }

                    if let Some(door) = entity.door {
                        e.insert(component::Door {
                            open: door.open,
                            toggle: false,
                            open_color: Color::DARK_GRAY,
                            close_color: Color::WHITE,
                        });
                    }

                    if let Some(api::RenderableComponent { renderable }) = entity.renderable {
                        use api::renderable_component::Renderable;
                        match Renderable::from_i32(renderable) {
                            Some(Renderable::None)
                            | Some(Renderable::Wall)
                            | Some(Renderable::Door) => {
                                e.insert(component::Renderable::default());
                            }
                            Some(Renderable::Human) => {
                                e.insert(component::Renderable {
                                    char: '@',
                                    color: Color::WHITE,
                                });
                            }
                            Some(Renderable::ServBot) => {
                                e.insert(component::Renderable {
                                    char: 'b',
                                    color: Color::ORANGE_RED,
                                });
                            }
                            Some(Renderable::Floor) => {
                                e.insert(component::Renderable {
                                    char: '·',
                                    color: Color::ALICE_BLUE,
                                });
                            }
                            None => {
                                log::warn!("Received unknown renderable ID {}", renderable);
                            }
                        }
                    }

                    if let Some(obstacle) = entity.obstacle {
                        e.insert(component::Obstacle(obstacle.obstacle));
                    }

                    if let Some(api::OrderingComponent { ordering }) = entity.ordering {
                        match <i32 as TryInto<component::Ordering>>::try_into(ordering) {
                            Ok(ordering) => {
                                e.insert(ordering);
                            }
                            Err(_) => {
                                log::warn!("Received unknown ordering ID {}", ordering);
                            }
                        }
                    }

                    if let Some(api::PositionComponent { x, y }) = entity.position {
                        let position = component::Position((x, y).into());
                        e.insert(position.clone());
                        positions.insert(position.clone());
                    }

                    if let Some(api::SightComponent { kind, seeing }) = entity.sight {
                        match <i32 as TryInto<component::SightKind>>::try_into(kind) {
                            Ok(kind) => {
                                e.insert(component::Sight {
                                    kind,
                                    seeing: seeing
                                        .iter()
                                        .map(|id| component::ReshockEntity(*id))
                                        .collect(),
                                });
                            }
                            Err(_) => {
                                log::warn!("Received unknown sight kind ID {}", kind);
                            }
                        }
                    }

                    if let Some(api::MemoryComponent { entities }) = entity.memory {
                        e.insert(component::Memory {
                            color: Color::DARK_GRAY,
                            ..Default::default()
                        });
                    }
                }

                for position in positions {
                    commands.spawn_bundle(bundle::Tile::new(position, &font));
                }
            }
            Err(e) => {
                log::error!("Could not load Reshock server state, fatal: {}", e);
            }
        }
    });
}
