use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::*;

use crate::game::{component::*, *};

pub fn behavior(
    action: Res<Action>,
    mut reactions: ResMut<Reactions>,
    viewers: Query<(&Position, &Sight, &Memory)>,
    sights: Query<(
        &Position,
        &Renderable,
        Option<&Door>,
        Option<&Wall>,
        Option<&Player>,
    )>,
) -> Status {
    let actor = match action.as_ref() {
        Action::Memorize(MemorizeAction::Intent { actor }) => *actor,
        Action::View(ViewAction::Update { actor, .. }) => {
            reactions
                .0
                .push(Action::Memorize(MemorizeAction::Intent { actor: *actor }));
            return Status::Continue;
        }
        _ => return Status::Continue,
    };

    let now = Instant::now();

    let (position, sight, memory) = viewers.get(actor).unwrap();
    let mut memory = memory.clone();

    let index: HashMap<IVec2, HashSet<Entity>> = memory
        .0
        .iter()
        .filter_map(|(e, cs)| {
            (cs.position.room == position.room)
                .then_some((cs.position.coordinates - position.coordinates, e))
        })
        .into_grouping_map()
        .collect();

    sight.mask.iter().for_each(|pos| {
        if let Some(entities) = index.get(&pos) {
            entities.iter().for_each(|e| {
                memory.0.remove(&e);
            })
        }
    });

    let view: HashMap<Entity, MemoryComponents> = sight
        .seeing
        .keys()
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

    memory.0.extend(view);

    reactions.0.push(Action::Memorize(MemorizeAction::Update {
        actor,
        memory: memory.clone(),
    }));

    let duration = Instant::now() - now;
    log::debug!("Time taken: {}µs", duration.as_micros());

    Status::Continue
}
