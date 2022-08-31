use bevy_ecs::prelude::*;

use crate::game::component::*;
use crate::game::{Events, *};

pub fn effect(
    action: Res<ActiveAction>,
    player: Query<Entity, With<Player>>,
    mut state_res: ResMut<api::State>,
    mut events: ResMut<Events>,
) {
    let state = match &action.0 {
        Some(Action::State(Some(it))) => it,
        _ => return,
    };

    let player = player.single();

    *state_res = state.clone();

    events.0.push(api::Event {
        event: Some(api::event::Event::State(api::StateUpdateEvent {
            player: player.id(),
            state: Some(state.clone()),
        })),
    });
}
