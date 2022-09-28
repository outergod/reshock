use bevy::prelude::*;

use crate::{component::*, resource::*};

pub fn system(
    mut commands: Commands,
    mut projectiles: Query<(Entity, &Projectile, &Velocity, &mut Transform)>,
    time: Res<Time>,
    mut events: ResMut<ReshockEvents>,
) {
    for (entity, projectile, velocity, mut transform) in projectiles.iter_mut() {
        let v = velocity.0 * time.delta_seconds();
        let position = transform.translation.truncate() + v;

        if (position - projectile.start).length() + projectile.size
            >= (projectile.end - projectile.start).length()
        {
            commands.entity(entity).despawn();
            events.transitions -= 1;
        } else {
            transform.translation = (position, 1.0).into();
        }
    }
}
