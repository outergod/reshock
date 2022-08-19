use bevy_ecs::prelude::*;

use crate::game::{component::*, *};

pub fn behavior(
    action: Res<ActiveAction>,
    mut reactions: ResMut<Reactions>,
    ai: Query<(Entity, &AI)>,
) -> Status {
    let entity = match &action.0 {
        Some(Action::AI(it)) => it,
        _ => return Status::Continue,
    };

    if let Some(ai) = ai.iter().find_map(|(e, ai)| (e == *entity).then_some(ai)) {
        match ai {
            AI::None => {
                log::debug!("I'm dumb and can't do shit");
            }
            AI::ServBot => {
                log::debug!("I'm not dumb and still can't do shit");
            }
        }
    }

    Status::Accept
}
