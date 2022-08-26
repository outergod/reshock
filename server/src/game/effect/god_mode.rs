use bevy_ecs::prelude::*;

use crate::game::component::*;
use crate::game::*;

pub fn effect(action: Res<ActiveAction>, mut commands: Commands, mut reactions: ResMut<Reactions>) {
    let GodModeAction { actor, activate } = match action.0 {
        Some(Action::GodMode(Some(it))) => it,
        _ => return,
    };

    let mut actor = commands.entity(actor);

    if activate {
        actor.insert(God);
    } else {
        actor.remove::<God>();
    }

    reactions.0.push(Action::View);
}
