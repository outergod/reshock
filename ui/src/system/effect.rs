use bevy::prelude::*;

use crate::{component::*, resource::ReshockEvents};

pub fn system(
    mut commands: Commands,
    mut effects: Query<(Entity, &mut Effect)>,
    mut events: ResMut<ReshockEvents>,
    time: Res<Time>,
) {
    for (entity, mut effect) in effects.iter_mut() {
        effect.lifetime.tick(time.delta());

        if effect.lifetime.finished() {
            if effect.remove {
                commands.entity(entity).despawn();
            } else {
                commands.entity(entity).remove::<Effect>();
            }
            events.transitions -= 1;
        }
    }
}
