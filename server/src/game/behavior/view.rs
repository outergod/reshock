use std::collections::HashSet;

use bevy_ecs::prelude::*;

use crate::game::{component::*, resource::*, *};

pub fn view_all(
    action: Res<ActiveAction>,
    viewers: Query<(Entity, &Sight)>,
    mut reactions: ResMut<Reactions>,
) -> Status {
    match action.0 {
        Some(Action::View(None)) => {}
        _ => return Status::Continue,
    };

    for (actor, sight) in viewers.iter() {
        reactions.0.push(Action::View(Some(ViewAction {
            actor,
            sight: Sight {
                kind: sight.kind,
                ..Default::default()
            },
        })));
    }

    Status::Continue
}

pub fn behavior(
    mut action: ResMut<ActiveAction>,
    positions: Query<&Position>,
    lines: Res<RadialLines>,
    spatial: Res<SpatialHash>,
    mut reactions: ResMut<Reactions>,
) -> Status {
    let ViewAction { sight, actor } = match action.0.as_mut() {
        Some(Action::View(Some(it))) => it,
        Some(Action::EndTurn(_)) => {
            reactions.0.push(Action::View(None));
            return Status::Continue;
        }
        _ => return Status::Continue,
    };

    let now = Instant::now();

    let position = positions.get(*actor).unwrap();

    let mut mask = match sight.kind {
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
                            .any(|line| !line.iter().any(|pos| spatial.is_opaque(&(*pos + viewer))))
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

    let duration = Instant::now() - now;
    log::debug!("Time taken: {}µs", duration.as_micros());

    Status::Continue
}
