use bevy_ecs::prelude::*;

use crate::game::component::*;
use crate::game::*;

pub fn effect(action: Res<ActiveAction>, mut positions: Query<&mut Position>) {
    let MoveAction {
        actor,
        position: target,
    } = match &action.0 {
        Some(Action::Move(r#move)) => r#move,
        _ => return,
    };

    positions.get_mut(*actor).unwrap().0 = *target;
}
