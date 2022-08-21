use bevy_ecs::prelude::*;

use crate::game::component::*;
use crate::game::*;

pub fn effect(action: Res<ActiveAction>, mut commands: Commands, mut reactions: ResMut<Reactions>) {
    let GodModeAction { entity, activate } = match action.0 {
        Some(Action::GodMode(Some(it))) => it,
        _ => return,
    };

    let mut entity = commands.entity(entity);

    if activate {
        entity.insert(God);
    } else {
        entity.remove::<God>();
    }

    reactions.0.push(Action::View);
}
