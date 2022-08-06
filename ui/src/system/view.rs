use bevy::prelude::*;

use crate::component::{ReshockEntity, Sight};

pub fn system(
    mut reader: EventReader<api::ViewUpdateEvent>,
    mut viewers: Query<(&ReshockEntity, &mut Sight)>,
) {
    for api::ViewUpdateEvent { entity, seeing } in reader.iter() {
        for (id, mut sight) in viewers.iter_mut() {
            if entity == &id.0 {
                sight.seeing = seeing.into_iter().map(|id| ReshockEntity(*id)).collect();
                continue;
            }
        }
    }
}
