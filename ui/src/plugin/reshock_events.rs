use bevy::log;
use bevy::prelude::*;

use crate::resource::ReshockEvents;

pub struct ReshockEventsPlugin;

impl Plugin for ReshockEventsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ReshockEvents>()
            .add_event::<api::DoorEvent>()
            .add_event::<api::StateUpdateEvent>()
            .add_event::<api::SpotEvent>()
            .add_event::<api::LogEvent>()
            .add_event::<api::HitEvent>()
            .add_event::<api::DeathEvent>()
            .add_event::<api::ShootEvent>()
            .add_event::<api::DestructionEvent>()
            .add_system(system);
    }
}

pub fn system(
    mut events: ResMut<ReshockEvents>,
    mut door: EventWriter<api::DoorEvent>,
    mut state: EventWriter<api::StateUpdateEvent>,
    mut spot: EventWriter<api::SpotEvent>,
    mut log: EventWriter<api::LogEvent>,
    mut hit: EventWriter<api::HitEvent>,
    mut death: EventWriter<api::DeathEvent>,
    mut shoot: EventWriter<api::ShootEvent>,
    mut destruction: EventWriter<api::DestructionEvent>,
) {
    if events.transitions > 0 {
        match events.queue.front() {
            Some(api::Event {
                event: Some(api::event::Event::Log(_)),
            }) => {}
            Some(api::Event {
                event: Some(api::event::Event::Death(_)),
            }) => {}
            Some(api::Event {
                event: Some(api::event::Event::Destruction(_)),
            }) => {}
            _ => return,
        }
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
        api::event::Event::Door(event) => {
            events.transitions += 1;
            door.send(event);
        }
        api::event::Event::State(event) => {
            events.transitions += 1;
            state.send(event);
        }
        api::event::Event::Spot(event) => {
            events.transitions += 1;
            spot.send(event);
        }
        api::event::Event::Log(event) => {
            log.send(event);
        }
        api::event::Event::Hit(event) => {
            events.transitions += 1;
            hit.send(event);
        }
        api::event::Event::Death(event) => {
            death.send(event);
        }
        api::event::Event::Destruction(event) => {
            destruction.send(event);
        }
        api::event::Event::Shoot(event) => {
            events.transitions += 1;
            shoot.send(event);
        }
    }
}
