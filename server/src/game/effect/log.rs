use bevy_ecs::prelude::*;

use crate::game::{resource::Log, Events, *};

pub fn effect(action: Res<Action>, mut log: ResMut<Log>, mut events: ResMut<Events>) {
    let entry = match action.as_ref() {
        Action::Log(it) => it.to_string(),
        _ => return,
    };

    log.add(entry.clone());

    events.0.push(api::Event {
        event: Some(api::event::Event::Log(api::LogEvent { entry })),
    });
}
