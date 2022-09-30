use bevy::prelude::*;

use crate::component::*;

pub fn system(
    mut commands: Commands,
    mut reader: EventReader<api::DestructionEvent>,
    entities: Query<(Entity, &ReshockEntity)>,
) {
    for api::DestructionEvent { actor } in reader.iter() {
        let entity = entities
            .iter()
            .find_map(|(here, there)| (&there.0 == actor).then_some(here))
            .unwrap();
        commands.entity(entity).despawn();
    }
}
