use std::collections::HashSet;

use bevy_ecs::prelude::*;

use crate::game::{component::*, pathfinding::AStar, resource::Deltas, *};

pub fn behavior(
    action: Res<Action>,
    mut reactions: ResMut<Reactions>,
    mut followups: ResMut<FollowUps>,
    ai: Query<(Entity, &AI, &Position, Option<&AIMemory>, Option<&Sight>)>,
    deltas: Res<Deltas>,
    obstacles: Query<&Position, With<Solid>>,
    player: Query<(Entity, &Position), With<Player>>,
) -> Status {
    let actor = match action.as_ref() {
        Action::AI(it) => it,
        Action::EndTurn(entity) => {
            if player.contains(*entity) {
                for (entity, ..) in ai.iter() {
                    followups.0.push(Action::AI(entity));
                }
            }
            return Status::Continue;
        }
        _ => return Status::Continue,
    };

    let mut rng = thread_rng();

    let player = player.single();

    let (actor, ai, viewer, mem, sight) = ai.get(*actor).unwrap();

    let obstacles: HashSet<_> = obstacles
        .iter()
        .filter_map(|pos| (pos.room == viewer.room).then_some(pos.coordinates))
        .collect();

    let (enemy, is_seen) = if sight.is_some_and(|sight| sight.seeing.contains_key(&player.0)) {
        (Some(player.1), true)
    } else if let Some(enemy) = mem.and_then(|mem| mem.enemy.as_ref()) {
        (Some(enemy), false)
    } else {
        (None, false)
    };

    match (ai, enemy) {
        (AI::None, _) => {
            log::debug!("I'm dumb and can't do shit");
        }
        (AI::ServBot, Some(enemy)) if enemy.room == viewer.room => {
            // TODO get through rooms
            let astar = AStar::new(obstacles);
            match astar.search(viewer.coordinates, enemy.coordinates) {
                Some(path) if path.len() > 1 => {
                    log::debug!("Path to player is {:?}", path);
                    let delta = path[1] - viewer.coordinates;

                    if !is_seen || path.len() > 2 {
                        reactions
                            .0
                            .push(Action::Move(MoveAction::Intent { actor, delta }));
                    } else {
                        reactions.0.push(Action::Melee(MeleeAttackAction::Intent {
                            actor,
                            direction: delta.into(),
                        }))
                    }
                }
                _ => {
                    if let Some(mem) = mem {
                        let mut memory = mem.clone();
                        memory.enemy = None;
                        reactions
                            .0
                            .push(Action::AIMemorize(AIMemorizeAction { actor, memory }));
                    }
                    log::debug!("Can't get to player, sulking");
                }
            }
        }
        (AI::ServBot, _) => {
            if let Some(delta) = deltas
                .0
                .clone()
                .into_iter()
                .filter(|delta| !obstacles.contains(&(viewer.coordinates + *delta)))
                .choose(&mut rng)
            {
                reactions
                    .0
                    .push(Action::Move(MoveAction::Intent { actor, delta }));
            } else {
                log::debug!("I'm stuck");
            }
        }
    }

    Status::Continue
}
