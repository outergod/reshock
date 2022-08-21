use bevy_ecs::prelude::*;

use crate::game::*;

pub fn behavior(action: Res<ActiveAction>) -> Status {
    match action.0 {
        Some(Action::View) => Status::Accept,
        _ => Status::Continue,
    }
}
