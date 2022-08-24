use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;

use bevy_ecs::prelude::*;
use glam::IVec2;
use itertools::Itertools;

use crate::game::component::*;
use crate::game::*;

pub fn effect(
    action: Res<ActiveAction>,
    mut viewers: Query<(&Sight, &mut Memory)>,
    sights: Query<(
        &Position,
        &Renderable,
        &Ordering,
        Option<&Door>,
        Option<&Wall>,
        Option<&Player>,
    )>,
) {
    match action.0 {
        Some(Action::View) => {}
        _ => return,
    }

    let now = Instant::now();

    for (sight, mut memory) in viewers.iter_mut() {
        let index: HashMap<IVec2, HashSet<Entity>> = memory
            .0
            .iter()
            .map(|(e, cs)| (cs.position.0, e))
            .into_grouping_map()
            .collect();

        sight.mask.iter().for_each(|pos| {
            if let Some(entities) = index.get(&pos) {
                entities.iter().for_each(|e| {
                    memory.0.remove(&e);
                })
            }
        });

        let view: HashMap<Entity, MemoryComponents> = sight
            .seeing
            .iter()
            .filter_map(|e| {
                sights
                    .get(*e)
                    .ok()
                    .map(|(position, renderable, ordering, door, wall, player)| {
                        (
                            *e,
                            MemoryComponents {
                                position: position.clone(),
                                renderable: renderable.clone(),
                                ordering: ordering.clone(),
                                door: door.cloned(),
                                wall: wall.cloned(),
                                player: player.cloned(),
                            },
                        )
                    })
            })
            .collect();

        memory.0.extend(view);
    }

    let duration = Instant::now() - now;
    log::debug!("Time taken: {}µs", duration.as_micros());
}
