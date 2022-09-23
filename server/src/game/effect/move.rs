use bevy_ecs::prelude::*;

use crate::game::component::*;
use crate::game::*;

pub fn effect(action: Res<Action>, mut positions: Query<&mut Position>) {
    match action.as_ref() {
        Action::Move(MoveAction::Update {
            actor, position, ..
        }) => {
            *positions.get_mut(*actor).unwrap() = *position;
        }
        _ => {}
    };
}
