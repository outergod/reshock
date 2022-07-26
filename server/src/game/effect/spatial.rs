use std::collections::HashMap;
use std::time::Instant;

use bevy_ecs::prelude::*;

use crate::game::component::*;
use crate::game::resource::{Cell, SpatialHash};
use crate::game::*;

pub fn effect(
    entities: Query<
        (
            Entity,
            &Position,
            Option<&Door>,
            Option<&Wall>,
            Option<&Solid>,
            Option<&Opaque>,
            Option<&Vulnerable>,
            Option<&Switch>,
        ),
        With<Renderable>,
    >,
    mut spatial: ResMut<SpatialHash>,
) {
    let now = Instant::now();

    let mut cells: HashMap<IVec2, Cell> = HashMap::new();
    entities.for_each(
        |(entity, position, door, wall, solid, opaque, vulnerable, switch)| {
            let mut cell = cells.entry(position.0).or_default();
            cell.visible.insert(entity);
            cell.door = door.map(|_| entity);
            cell.wall = wall.map(|_| entity);
            cell.solid = solid.map(|_| entity);
            if opaque.is_some() {
                cell.opaque.insert(entity);
            }
            if cell.vulnerable.is_none() {
                cell.vulnerable = solid.and(vulnerable.map(|_| entity));
            }
            cell.switch = switch.map(|_| entity);
        },
    );
    spatial.cells = cells;

    let duration = Instant::now() - now;
    log::debug!("Time taken: {}µs", duration.as_micros());
}
