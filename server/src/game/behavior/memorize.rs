use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::*;
use itertools::Itertools;

use crate::game::{component::*, *};

pub fn behavior(
    mut action: ResMut<ActiveAction>,
    viewers: Query<&Sight>,
    sights: Query<(
        &Position,
        &Renderable,
        Option<&Door>,
        Option<&Wall>,
        Option<&Player>,
    )>,
    memories: Query<&Memory>,
    mut reactions: ResMut<Reactions>,
) -> Status {
    let action = match action.0.as_mut() {
        Some(Action::Memorize(it)) => it,
        Some(Action::View(ViewAction::Update { actor, .. })) => {
            reactions.0.push(Action::Memorize(MemorizeAction {
                actor: *actor,
                memory: Default::default(),
            }));
            return Status::Continue;
        }
        _ => return Status::Continue,
    };

    let now = Instant::now();

    let sight = viewers.get(action.actor).unwrap();
    action.memory = memories.get(action.actor).unwrap().clone();

    let index: HashMap<IVec2, HashSet<Entity>> = action
        .memory
        .0
        .iter()
        .map(|(e, cs)| (cs.position.0, e))
        .into_grouping_map()
        .collect();

    sight.mask.iter().for_each(|pos| {
        if let Some(entities) = index.get(&pos) {
            entities.iter().for_each(|e| {
                action.memory.0.remove(&e);
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
                .map(|(position, renderable, door, wall, player)| {
                    (
                        *e,
                        MemoryComponents {
                            position: position.clone(),
                            renderable: renderable.clone(),
                            door: door.cloned(),
                            wall: wall.cloned(),
                            player: player.cloned(),
                        },
                    )
                })
        })
        .collect();

    action.memory.0.extend(view);

    let duration = Instant::now() - now;
    log::debug!("Time taken: {}µs", duration.as_micros());

    Status::Continue
}
