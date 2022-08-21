use std::collections::HashMap;
use std::time::Instant;

use bevy_ecs::prelude::*;
use glam::IVec2;

use crate::game::component::*;
use crate::game::resource::*;
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
    )>,
    spatial: Res<SpatialHash>,
) {
    match action.0 {
        Some(Action::View) => {}
        _ => return,
    }

    let now = Instant::now();

    for (sight, mut memory) in viewers.iter_mut() {
        sight.mask.iter().for_each(|pos| {
            memory.0.remove(&pos);
        });

        let view: HashMap<IVec2, Vec<MemoryComponents>> = sight
            .mask
            .iter()
            .filter_map(|pos| {
                spatial.cells.get(pos).map(|entities| {
                    (
                        pos.clone(),
                        entities
                            .iter()
                            .filter_map(|e| {
                                sights.get(*e).ok().map(
                                    |(position, renderable, ordering, door, wall)| {
                                        MemoryComponents {
                                            entity: *e,
                                            position: position.clone(),
                                            renderable: renderable.clone(),
                                            ordering: ordering.clone(),
                                            door: door.cloned(),
                                            wall: wall.cloned(),
                                        }
                                    },
                                )
                            })
                            .collect(),
                    )
                })
            })
            .collect();

        memory.0.extend(view);
    }

    let duration = Instant::now() - now;
    log::debug!("Time taken: {}µs", duration.as_micros());
}
