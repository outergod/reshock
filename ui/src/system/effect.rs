use bevy::prelude::*;

use crate::{
    component::*,
    resource::{ReshockEvents, TransitionState},
};

pub fn system(
    mut commands: Commands,
    mut effects: Query<(Entity, &mut Effect)>,
    mut events: ResMut<ReshockEvents>,
    time: Res<Time>,
) {
    for (entity, mut effect) in effects.iter_mut() {
        effect.lifetime.tick(time.delta());

        if effect.lifetime.finished() {
            commands.entity(entity).despawn();
            events.state = TransitionState::Inactive;
        }
    }
}
