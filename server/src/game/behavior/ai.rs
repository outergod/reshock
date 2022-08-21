use std::collections::HashSet;

use bevy_ecs::prelude::*;

use crate::game::{component::*, resource::Deltas, *};

pub fn behavior(
    action: Res<ActiveAction>,
    mut reactions: ResMut<Reactions>,
    ai: Query<(Entity, &AI, Option<&Position>)>,
    deltas: Res<Deltas>,
    obstacles: Query<&Position, With<Solid>>,
) -> Status {
    let entity = match &action.0 {
        Some(Action::AI(it)) => it,
        _ => return Status::Continue,
    };

    let mut rng = thread_rng();

    let obstacles: HashSet<IVec2> = obstacles.iter().map(|pos| pos.0).collect();

    if let Ok((actor, ai, pos)) = ai.get(*entity) {
        match ai {
            AI::None => {
                log::debug!("I'm dumb and can't do shit");
            }
            AI::ServBot => {
                if let Some(Position(pos)) = pos {
                    if let Some(pos) = deltas
                        .0
                        .iter()
                        .map(|delta| *pos + *delta)
                        .filter(|pos| !obstacles.contains(pos))
                        .choose(&mut rng)
                    {
                        reactions.0.push(Action::Move(MoveAction {
                            entity: actor,
                            position: pos,
                        }));
                    }
                }
            }
        }
    }

    Status::Accept
}
