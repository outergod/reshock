use bevy_ecs::prelude::*;

use crate::game::component::*;
use crate::game::*;

pub fn effect(action: Res<ActiveAction>, mut memories: Query<&mut Memory>) {
    let MemorizeAction { actor, memory, .. } = match &action.0 {
        Some(Action::Memorize(it)) => it,
        _ => return,
    };

    *memories.get_mut(*actor).unwrap() = memory.clone();
}
