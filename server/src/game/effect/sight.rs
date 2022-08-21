use std::collections::HashSet;
use std::time::Instant;

use bevy_ecs::prelude::*;

use crate::game::component::*;
use crate::game::resource::*;
use crate::game::*;

pub fn effect(
    action: Res<ActiveAction>,
    mut viewers: Query<(&Position, &mut Sight, Option<&God>)>,
    obstacles: Query<&Position, With<Opaque>>,
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
                let obstacles: HashSet<_> = obstacles.iter().map(|p| p.0 - position.0).collect();
                lines
                    .0
                    .iter()
                    .filter_map(|(pos, paths)| {
                        paths
                            .iter()
                            .any(|path| !path.iter().any(|p| obstacles.contains(p)))
                            .then_some(position.0 + *pos)
                    })
                    .collect()
            }
        };

        mask.insert(position.0);

        let view = mask
            .iter()
            .flat_map(|pos| spatial.cells.get(pos).cloned().unwrap_or_default())
            .collect();
        sight.mask = mask;
        sight.seeing = view;
    }

    let duration = Instant::now() - now;
    log::debug!("Time taken: {}µs", duration.as_micros());
}
