use bevy::prelude::*;

use crate::resource::{ReshockEvents, TransitionState};

pub fn system(mut reader: EventReader<api::MoveEvent>, mut events: ResMut<ReshockEvents>) {
    for _ in reader.iter() {
        events.state = TransitionState::Inactive;
    }
}

// pub fn system(
//     mut reader: EventReader<api::MoveEvent>,
//     mut events: ResMut<ReshockEvents>,
//     mut movables: Query<(&ReshockEntity, &mut Position)>,
// ) {
//     for api::MoveEvent { entity, x, y } in reader.iter() {
//         if let Some(mut position) = movables
//             .iter_mut()
//             .find_map(|(id, position)| (entity == &id.0).then_some(position))
//         {
//             position.0.x = *x;
//             position.0.y = *y;
//         }

//         events.state = TransitionState::Inactive;
//     }
// }
