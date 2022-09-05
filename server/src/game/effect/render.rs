use bevy_ecs::prelude::*;

use crate::game::{component::*, *};

pub fn effect(action: Res<Action>, mut renderables: Query<&mut Renderable>) {
    let DeathAction { actor, .. } = match action.as_ref() {
        Action::Death(it) => it,
        _ => return,
    };

    if let Ok(mut renderable) = renderables.get_mut(*actor) {
        *renderable = Renderable::Corpse;
    }
}
