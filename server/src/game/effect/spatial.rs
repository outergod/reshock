use std::collections::{HashMap, HashSet};
use std::time::Instant;

use bevy_ecs::prelude::*;

use crate::game::component::*;
use crate::game::resource::SpatialHash;
use crate::game::*;

pub fn effect(
    action: Res<ActiveAction>,
    sights: Query<(Entity, &Position), With<Renderable>>,
    mut spatial: ResMut<SpatialHash>,
) {
    match action.0 {
        Some(Action::View) => {}
        _ => return,
    }

    let now = Instant::now();

    let mut cells = HashMap::new();
    sights.for_each(|(entity, position)| {
        cells
            .entry(position.0)
            .or_insert_with(HashSet::new)
            .insert(entity);
    });
    spatial.cells = cells;

    let duration = Instant::now() - now;
    log::debug!("Time taken: {}µs", duration.as_micros());
}
