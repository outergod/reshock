use bevy::prelude::*;

use crate::{
    component::{Position, ReshockEntity},
    resource::{ReshockEvents, TransitionState},
};

// pub fn system(reader: EventReader<api::MoveEvent>, mut events: ResMut<ReshockEvents>) {
//     if reader.is_empty() {
//         return;
//     }

//     events.state = TransitionState::Inactive;
// }

pub fn system(
    mut reader: EventReader<api::MoveEvent>,
    mut events: ResMut<ReshockEvents>,
    mut movables: Query<(&ReshockEntity, &mut Position)>,
) {
    for api::MoveEvent { actor, x, y } in reader.iter() {
        if let Some(mut position) = movables
            .iter_mut()
            .find_map(|(id, position)| (actor == &id.0).then_some(position))
        {
            position.0.x = *x;
            position.0.y = *y;
        }

        events.state = TransitionState::Inactive;
    }
}
