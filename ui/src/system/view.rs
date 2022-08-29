use api::renderable_component::Renderable as ApiRenderable;
use api::*;
use bevy::log;
use bevy::prelude::*;

use crate::component::*;
use crate::resource::ReshockEvents;
use crate::resource::TransitionState;

pub fn system(
    mut commands: Commands,
    entities: Query<Entity, With<ReshockEntity>>,
    mut reader: EventReader<api::ViewUpdateEvent>,
    mut events: ResMut<ReshockEvents>,
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
                door,
                memory,
                wall,
            } = components;
            let mut e = commands.spawn();

            if entity == *player {
                e.insert(Player);
                e.insert(Focus);
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

        events.state = TransitionState::Inactive;
    }
}
