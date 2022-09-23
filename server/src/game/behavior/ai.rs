use std::collections::HashSet;

use bevy_ecs::prelude::*;

use crate::game::{component::*, pathfinding::AStar, resource::Deltas, *};

pub fn behavior(
    action: Res<Action>,
    mut reactions: ResMut<Reactions>,
    ai: Query<(Entity, &AI, &Position, Option<&Sight>, Option<&Memory>)>,
    deltas: Res<Deltas>,
    obstacles: Query<&Position, With<Solid>>,
    player: Query<(Entity, &Position), With<Player>>,
    mut followups: ResMut<FollowUps>,
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

    let (actor, ai, viewer, sight, mem) = ai.get(*actor).unwrap();

    let obstacles: HashSet<_> = obstacles
        .iter()
        .filter_map(|pos| {
            (pos.room == viewer.room).then_some(viewer.coordinates - viewer.coordinates)
        })
        .collect();

    let enemy = {
        if sight.is_some_and(|sight| sight.seeing.contains_key(&player.0))
            || mem.is_some_and(|mem| mem.0.contains_key(&player.0))
        {
            Some(player.1)
        } else {
            None
        }
    };

    match (ai, enemy) {
        (AI::None, _) => {
            log::debug!("I'm dumb and can't do shit");
        }
        (AI::ServBot, None) => {
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
        (AI::ServBot, Some(enemy)) => {
            let astar = AStar::new(obstacles);
            match astar.search(viewer.coordinates, enemy.coordinates) {
                Some(path) => {
                    let delta = path[1] - viewer.coordinates;

                    if path.len() > 2 {
                        reactions
                            .0
                            .push(Action::Move(MoveAction::Intent { actor, delta }));
                    } else {
                        reactions.0.push(Action::Melee(MeleeAttackAction::Intent {
                            actor,
                            target: player.0,
                            direction: delta.into(),
                        }))
                    }
                }
                None => {
                    log::debug!("Can't get to player, sulking");
                }
            }
        }
    }

    Status::Continue
}
