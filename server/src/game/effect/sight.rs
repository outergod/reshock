use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::*;

use crate::game::component::*;
use crate::game::resource::RadialLines;

pub fn effect(
    mut viewers: Query<(Entity, &Position, &mut Sight, &mut Memory)>,
    sights: Query<(Entity, &Position, &Renderable, &Ordering, Option<&Door>)>,
    obstacles: Query<&Position, With<Opaque>>,
    lines: Res<RadialLines>,
) {
    for (entity, position, mut sight, mut memory) in viewers.iter_mut() {
        let obstacles: HashSet<_> = obstacles.iter().map(|p| p.0 - position.0).collect();

        let view: HashMap<Entity, MemoryComponents> = match sight.kind {
            SightKind::Blind => HashMap::new(),
            SightKind::Omniscience => sights
                .iter()
                .map(|(entity, position, renderable, ordering, door)| {
                    (
                        entity.clone(),
                        MemoryComponents {
                            position: position.clone(),
                            renderable: renderable.clone(),
                            ordering: ordering.clone(),
                            door: door.cloned(),
                        },
                    )
                })
                .collect(),
            SightKind::Eyes => {
                let empty = HashSet::new();

                sights
                    .iter()
                    .filter_map(|(seen_entity, seen_position, renderable, ordering, door)| {
                        let position = seen_position.0 - position.0;
                        if lines
                            .0
                            .get(&position)
                            .unwrap_or(&empty)
                            .iter()
                            .any(|path| !path.iter().any(|p| obstacles.contains(p)))
                        {
                            Some((
                                seen_entity.clone(),
                                MemoryComponents {
                                    position: seen_position.clone(),
                                    renderable: renderable.clone(),
                                    ordering: ordering.clone(),
                                    door: door.cloned(),
                                },
                            ))
                        } else {
                            None
                        }
                    })
                    .collect()
            }
        };

        sight.seeing = view.keys().cloned().collect();
        sight.seeing.insert(entity);
        memory.0.extend(view);
    }
}
