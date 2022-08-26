use std::collections::HashSet;
use std::time::Instant;

use bevy_ecs::prelude::*;

use crate::game::component::*;
use crate::game::resource::*;
use crate::game::*;

pub fn effect(
    action: Res<ActiveAction>,
    mut viewers: Query<(&Position, &mut Sight, Option<&God>)>,
    lines: Res<RadialLines>,
    spatial: Res<SpatialHash>,
) {
    match action.0 {
        Some(Action::View) => {}
        _ => return,
    }

    let now = Instant::now();

    for (position, mut sight, god) in viewers.iter_mut() {
        let kind = if god.is_some() {
            &SightKind::Omniscience
        } else {
            &sight.kind
        };

        let mut mask = match kind {
            SightKind::Blind => HashSet::new(),
            SightKind::Omniscience => spatial.cells.keys().cloned().collect(),
            SightKind::Eyes => {
                let viewer = position.0;

                spatial
                    .cells
                    .keys()
                    .cloned()
                    .filter_map(|pos| {
                        let shifted = pos - viewer;
                        lines.0.get(&shifted).and_then(|lines| {
                            lines
                                .iter()
                                .any(|line| {
                                    !line.iter().any(|pos| spatial.is_opaque(&(*pos + viewer)))
                                })
                                .then_some(pos)
                        })
                    })
                    .collect()
            }
        };

        mask.insert(position.0);

        let view = mask
            .iter()
            .flat_map(|pos| spatial.entities_at(pos))
            .collect();
        sight.mask = mask;
        sight.seeing = view;
    }

    let duration = Instant::now() - now;
    log::debug!("Time taken: {}µs", duration.as_micros());
}
