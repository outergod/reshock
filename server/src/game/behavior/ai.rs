use std::collections::HashSet;

use bevy_ecs::prelude::*;

use crate::game::{component::*, pathfinding::AStar, resource::Deltas, *};

pub fn behavior(
    action: Res<Action>,
    mut reactions: ResMut<Reactions>,
    ai: Query<(
        Entity,
        &AI,
        Option<&Position>,
        Option<&Sight>,
        Option<&Memory>,
    )>,
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

    let obstacles: HashSet<IVec2> = obstacles.iter().map(|pos| pos.0).collect();

    let (actor, ai, pos, sight, mem) = ai.get(*actor).unwrap();

    let enemy = {
        if sight.is_some_and(|sight| sight.seeing.contains(&player.0))
            || mem.is_some_and(|mem| mem.0.contains_key(&player.0))
        {
            Some(player.1)
        } else {
            None
        }
    };

    match (ai, pos, enemy) {
        (AI::None, ..) => {
            log::debug!("I'm dumb and can't do shit");
        }
        (AI::ServBot, None, None) => {
            log::debug!("I'm stuck");
        }
        (AI::ServBot, None, Some(_)) => todo!(),
        (AI::ServBot, Some(pos), None) => {
            if let Some(pos) = deltas
                .0
                .iter()
                .map(|delta| pos.0 + *delta)
                .filter(|pos| !obstacles.contains(pos))
                .choose(&mut rng)
            {
                reactions.0.push(Action::Move(MoveAction {
                    actor,
                    position: pos,
                }));
            } else {
                log::debug!("I'm stuck");
            }
        }
        (AI::ServBot, Some(pos), Some(enemy)) => {
            let astar = AStar::new(obstacles);
            match astar.search(pos.0, enemy.0) {
                Some(path) => {
                    if path.len() > 2 {
                        reactions.0.push(Action::Move(MoveAction {
                            actor,
                            position: path[1],
                        }));
                    } else {
                        reactions.0.push(Action::Melee(MeleeAttackAction::Intent {
                            actor,
                            target: player.0,
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
