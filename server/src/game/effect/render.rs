use bevy_ecs::prelude::*;

use crate::game::{component::*, *};

pub fn effect(action: Res<ActiveAction>, mut renderables: Query<(&mut Renderable, &mut Ordering)>) {
    let DeathAction { actor, .. } = match &action.0 {
        Some(Action::Death(it)) => it,
        _ => return,
    };

    if let Ok((mut renderable, mut ordering)) = renderables.get_mut(*actor) {
        *renderable = Renderable::Corpse;
        *ordering = Ordering::Item;
    }
}
