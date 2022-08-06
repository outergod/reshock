use bevy_ecs::prelude::*;

use crate::game::{component::*, Action, ActiveAction, MoveAction};

pub fn effect(action: Res<ActiveAction>, mut positions: Query<(Entity, &mut Position)>) {
    let MoveAction {
        entity,
        position: target,
    } = match &action.0 {
        Some(Action::Move(r#move)) => r#move,
        _ => return,
    };

    match positions
        .iter_mut()
        .find_map(|(e, p)| if e == *entity { Some(p) } else { None })
    {
        Some(mut position) => {
            position.0 = *target;
        }
        None => {
            log::warn!(
                "Invalid move action, entity {:?} does not have Position component",
                entity
            );
        }
    }
}
