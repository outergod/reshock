use api::renderable_component::Renderable as ApiRenderable;
use api::*;
use bevy::log;
use bevy::math::ivec2;
use bevy::prelude::*;
use bevy::utils::HashSet;

use crate::component::{Position, *};
use crate::{bundle, resource::*};

pub fn system(
    mut commands: Commands,
    entities: Query<Entity, With<ReshockEntity>>,
    tiles: Query<&Position, With<Tile>>,
    mut reader: EventReader<api::StateUpdateEvent>,
    mut events: ResMut<ReshockEvents>,
    font: Res<ReshockFont>,
) {
    for api::StateUpdateEvent { player, state } in reader.iter() {
        let state = match state {
            Some(it) => it,
            None => {
                log::error!("Received empty state from Reshock");
                return;
            }
        };

        for entity in entities.iter() {
            commands.entity(entity).despawn();
        }

        let tiles: HashSet<_> = tiles.iter().collect();

        for (entity, components) in state.entities.clone() {
            let Components {
                positions,
                renderable,
                door,
                memory,
                wall,
            } = components;

            for PositionComponent { x, y } in positions {
                let pos = Position(ivec2(x, y));
                if !tiles.contains(&pos) {
                    commands.spawn_bundle(bundle::Tile::new(pos, &font));
                }

                let mut e = commands.spawn();

                if entity == *player {
                    e.insert(Player);
                    e.insert(Focus);
                }

                e.insert(ReshockEntity(entity));
                e.insert(Position(ivec2(x, y)));

                let memory = memory.is_some();

                if let Some(RenderableComponent { renderable }) = renderable {
                    if let Some(renderable) = match ApiRenderable::from_i32(renderable) {
                        Some(ApiRenderable::None) => Some(Renderable::default()),
                        Some(ApiRenderable::Wall) => Some(Renderable {
                            char: ' ',
                            color: if memory {
                                Color::DARK_GRAY
                            } else {
                                Color::rgb(0.169, 0.173, 0.29)
                            },
                            ordering: Ordering::Wall,
                        }),
                        Some(ApiRenderable::Door) => Some(Renderable {
                            char: ' ',
                            color: if memory {
                                Color::DARK_GRAY
                            } else {
                                Color::WHITE
                            },
                            ordering: Ordering::Door,
                        }),
                        Some(ApiRenderable::Human) => Some(Renderable {
                            char: '@',
                            color: if memory {
                                Color::DARK_GRAY
                            } else {
                                Color::WHITE
                            },
                            ordering: Ordering::Actor,
                        }),
                        Some(ApiRenderable::ServBot) => Some(Renderable {
                            char: 'b',
                            color: if memory {
                                Color::DARK_GRAY
                            } else {
                                Color::ORANGE_RED
                            },
                            ordering: Ordering::Actor,
                        }),
                        Some(ApiRenderable::Floor) => Some(Renderable {
                            char: '·',
                            color: if memory {
                                Color::DARK_GRAY
                            } else {
                                Color::rgb(0.169, 0.173, 0.29)
                            },
                            ordering: Ordering::Floor,
                        }),
                        Some(ApiRenderable::Corpse) => Some(Renderable {
                            char: '%',
                            color: if memory {
                                Color::DARK_GRAY
                            } else {
                                Color::WHITE
                            },
                            ordering: Ordering::Item,
                        }),
                        _ => None,
                    } {
                        e.insert(renderable);
                    }
                }

                if let Some(DoorComponent { open }) = door {
                    e.insert(Door {
                        open,
                        open_color: Color::DARK_GRAY,
                        close_color: Color::WHITE,
                    });
                }

                if let Some(WallComponent {}) = wall {
                    e.insert(Wall);
                }

                if memory {
                    e.insert(Memory);
                }
            }
        }

        events.state = TransitionState::Inactive;
    }
}
