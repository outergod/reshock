use bevy_ecs::prelude::*;

use crate::game::{Status, *};

pub fn behavior(action: Res<ActiveAction>) -> Status {
    match action.0 {
        Some(Action::HealthLoss(_)) => Status::Accept,
        _ => Status::Continue,
    }
}
