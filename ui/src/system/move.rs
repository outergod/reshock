use bevy::prelude::*;

use crate::component::{Position, ReshockEntity};

pub fn system(
    mut reader: EventReader<api::MoveEvent>,
    mut movables: Query<(&ReshockEntity, &mut Position)>,
) {
    for api::MoveEvent { entity, x, y } in reader.iter() {
        for (id, mut position) in movables.iter_mut() {
            if entity == &id.0 {
                position.0.x = *x;
                position.0.y = *y;
                continue;
            }
        }
    }
}
