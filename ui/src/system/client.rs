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

                    if let Some(Ok(renderable)) = entity
                        .renderable
                        .map(|it| -> Result<component::Renderable, _> { it.try_into() })
                    {
                        e.insert(renderable);
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

                    if let Some(api::SightComponent { seeing }) = entity.sight {
                        e.insert(component::Sight {
                            seeing: seeing
                                .iter()
                                .map(|id| component::ReshockEntity(*id))
                                .collect(),
                        });
                    }

                    if let Some(api::MemoryComponent { entities }) = entity.memory {
                        let entities = entities
                            .into_iter()
                            .filter_map(|memory| {
                                let components: component::MemoryComponents =
                                    match memory.clone().try_into() {
                                        Ok(it) => it,
                                        Err(_) => return None,
                                    };
                                Some((component::ReshockEntity(memory.entity), components))
                            })
                            .collect();

                        e.insert(component::Memory {
                            color: Color::DARK_GRAY,
                            entities,
                        });
                    }
                }

                for y in 0..=100 {
                    for x in 0..=100 {
                        commands.spawn_bundle(bundle::Tile::new(
                            component::Position((x, y).into()),
                            &font,
                        ));
                    }
                }
            }
            Err(e) => {
                log::error!("Could not load Reshock server state, fatal: {}", e);
            }
        }
    });
}
