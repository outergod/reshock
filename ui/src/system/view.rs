use api::ordering_component::Ordering as ApiOrdering;
use api::renderable_component::Renderable as ApiRenderable;
use api::*;
use bevy::log;
use bevy::prelude::*;

use crate::component::*;

pub fn system(
    mut commands: Commands,
    entities: Query<Entity, With<ReshockEntity>>,
    mut reader: EventReader<api::ViewUpdateEvent>,
) {
    for api::ViewUpdateEvent { player, view } in reader.iter() {
        let view = match view {
            Some(it) => it,
            None => {
                log::error!("Received empty view state from Reshock");
                return;
            }
        };

        for entity in entities.iter() {
            commands.entity(entity).despawn();
        }

        for (entity, components) in view.entities.clone() {
            let Components {
                position,
                renderable,
                ordering,
                door,
                memory,
                wall,
            } = components;
            let mut e = commands.spawn();

            if entity == *player {
                e.insert(Player);
            }

            e.insert(ReshockEntity(entity));

            if let Some(PositionComponent { x, y }) = position {
                e.insert(Position((x, y).into()));
            }

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
                    }),
                    Some(ApiRenderable::Door) => Some(Renderable {
                        char: ' ',
                        color: if memory {
                            Color::DARK_GRAY
                        } else {
                            Color::WHITE
                        },
                    }),
                    Some(ApiRenderable::Human) => Some(Renderable {
                        char: '@',
                        color: if memory {
                            Color::DARK_GRAY
                        } else {
                            Color::WHITE
                        },
                    }),
                    Some(ApiRenderable::ServBot) => Some(Renderable {
                        char: 'b',
                        color: if memory {
                            Color::DARK_GRAY
                        } else {
                            Color::ORANGE_RED
                        },
                    }),
                    Some(ApiRenderable::Floor) => Some(Renderable {
                        char: '·',
                        color: if memory {
                            Color::DARK_GRAY
                        } else {
                            Color::rgb(0.169, 0.173, 0.29)
                        },
                    }),
                    None => None,
                } {
                    e.insert(renderable);
                }
            }

            if let Some(OrderingComponent { ordering }) = ordering {
                if let Some(ordering) = match ApiOrdering::from_i32(ordering) {
                    Some(ApiOrdering::Floor) => Some(Ordering::Floor),
                    Some(ApiOrdering::Door) => Some(Ordering::Door),
                    Some(ApiOrdering::Wall) => Some(Ordering::Wall),
                    Some(ApiOrdering::Other) => Some(Ordering::Other),
                    _ => None,
                } {
                    e.insert(ordering);
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
}
