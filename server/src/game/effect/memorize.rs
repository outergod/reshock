use bevy_ecs::prelude::*;

use crate::game::component::*;
use crate::game::*;

pub fn effect(action: Res<Action>, mut memories: Query<&mut Memory>) {
    let (actor, memory) = match action.as_ref() {
        Action::Memorize(MemorizeAction { actor, memory }) => (*actor, memory.clone()),
        _ => return,
    };

    *memories.get_mut(actor).unwrap() = memory;
}

pub fn ai(action: Res<Action>, mut memories: Query<&mut AIMemory>) {
    let (actor, memory) = match action.as_ref() {
        Action::AIMemorize(AIMemorizeAction { actor, memory }) => (*actor, memory.clone()),
        _ => return,
    };

    *memories.get_mut(actor).unwrap() = memory;
}
