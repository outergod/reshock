use bevy::prelude::*;

use crate::component::{Memory, MemoryComponents, ReshockEntity};

pub fn system(
    mut reader: EventReader<api::MemoryUpdateEvent>,
    mut memories: Query<(&ReshockEntity, &mut Memory)>,
) {
    for api::MemoryUpdateEvent {
        entity,
        memory: update,
    } in reader.iter()
    {
        for (id, mut memory) in memories.iter_mut() {
            if entity == &id.0 {
                memory.entities = update
                    .into_iter()
                    .filter_map(|memory| {
                        let components: MemoryComponents = match memory.clone().try_into() {
                            Ok(it) => it,
                            Err(_) => return None,
                        };
                        Some((ReshockEntity(memory.entity), components))
                    })
                    .collect();
                continue;
            }
        }
    }
}
