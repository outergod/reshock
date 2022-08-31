use bevy_ecs::prelude::*;

use crate::game::{Events, *};

pub fn effect(action: Res<ActiveAction>, mut events: ResMut<Events>) {
    let SpotAction { actor, sound } = match &action.0 {
        Some(Action::Spot(it)) => it,
        _ => return,
    };

    events.0.push(api::Event {
        event: Some(api::event::Event::Spot(api::SpotEvent {
            actor: actor.id(),
            sound: *sound as i32,
        })),
    });
}
