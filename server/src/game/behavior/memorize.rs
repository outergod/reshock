use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::*;

use crate::game::{component::*, *};

pub fn behavior(
    action: Res<Action>,
    mut reactions: ResMut<Reactions>,
    memories: Query<&Memory>,
    sights: Query<(
        &Position,
        &Renderable,
        Option<&Door>,
        Option<&Wall>,
        Option<&Player>,
    )>,
) -> Status {
    let (actor, sight) = match action.as_ref() {
        Action::View(ViewAction::Update { actor, sight }) => (*actor, sight),
        _ => return Status::Continue,
    };

    let memory = match memories.get(actor) {
        Ok(it) => it,
        Err(_) => return Status::Continue,
    };

    let now = Instant::now();

    let mut memory = memory.clone();

    let index: HashMap<Position, HashSet<Entity>> = memory
        .0
        .iter()
        .map(|(e, cs)| (cs.position, e))
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

    reactions
        .0
        .push(Action::Memorize(MemorizeAction { actor, memory }));

    let duration = Instant::now() - now;
    log::debug!("Time taken: {}µs", duration.as_micros());

    Status::Continue
}

pub fn ai(
    action: Res<Action>,
    mut reactions: ResMut<Reactions>,
    ai: Query<&AIMemory>,
    player: Query<Entity, With<Player>>,
) -> Status {
    let (actor, memory) = match action.as_ref() {
        Action::Memorize(MemorizeAction { actor, memory }) => (*actor, memory),
        _ => return Status::Continue,
    };

    if !ai.contains(actor) {
        return Status::Continue;
    }

    let player = player.single();

    if let Some(position) = memory.0.get(&player).map(|cs| cs.position) {
        reactions.0.push(Action::AIMemorize(AIMemorizeAction {
            actor,
            memory: AIMemory {
                enemy: Some(position),
            },
        }));
    }

    Status::Continue
}
