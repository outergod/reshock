use bevy_ecs::prelude::*;

use crate::game::component::*;
use crate::game::*;

pub fn behavior(mut action: ResMut<ActiveAction>, entities: Query<&AI>) -> Status {
    match action.0.as_mut() {
        Some(Action::Spot(SpotAction { entity, sound })) => match entities.get(*entity) {
            Ok(AI::ServBot) => {
                *sound = Some(api::spot_event::SpotSound::ServBot);
                Status::Accept
            }
            _ => Status::Reject,
        },
        _ => Status::Continue,
    }
}
