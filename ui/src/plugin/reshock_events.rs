use bevy::log;
use bevy::prelude::*;

use crate::resource::ReshockEvents;
use crate::resource::TransitionState;

pub struct ReshockEventsPlugin;

impl Plugin for ReshockEventsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ReshockEvents>()
            .add_event::<api::MoveEvent>()
            .add_event::<api::DoorEvent>()
            .add_event::<api::StateUpdateEvent>()
            .add_event::<api::SpotEvent>()
            .add_event::<api::LogEvent>()
            .add_event::<api::HitEvent>()
            .add_event::<api::DeathEvent>()
            .add_system(system);
    }
}

pub fn system(
    mut events: ResMut<ReshockEvents>,
    mut r#move: EventWriter<api::MoveEvent>,
    mut door: EventWriter<api::DoorEvent>,
    mut state: EventWriter<api::StateUpdateEvent>,
    mut spot: EventWriter<api::SpotEvent>,
    mut log: EventWriter<api::LogEvent>,
    mut hit: EventWriter<api::HitEvent>,
    mut death: EventWriter<api::DeathEvent>,
) {
    if events.state == TransitionState::Active {
        return;
    }

    let event = match events.queue.pop_front() {
        Some(api::Event { event: Some(it) }) => it,
        Some(api::Event { event: None }) => {
            log::warn!("Received empty event");
            return;
        }
        None => return,
    };

    log::debug!("Processing event {}", event);

    match event {
        api::event::Event::Move(event) => {
            events.state = TransitionState::Active;
            r#move.send(event);
        }
        api::event::Event::Door(event) => {
            events.state = TransitionState::Active;
            door.send(event);
        }
        api::event::Event::State(event) => {
            events.state = TransitionState::Active;
            state.send(event);
        }
        api::event::Event::Spot(event) => {
            events.state = TransitionState::Active;
            spot.send(event);
        }
        api::event::Event::Log(event) => {
            log.send(event);
        }
        api::event::Event::Hit(event) => {
            hit.send(event);
        }
        api::event::Event::Death(event) => {
            events.state = TransitionState::Active;
            death.send(event);
        }
    }
}
