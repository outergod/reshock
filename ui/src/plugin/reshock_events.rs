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
            .add_event::<api::ViewUpdateEvent>()
            .add_event::<api::SpotEvent>()
            .add_system(system);
    }
}

pub fn system(
    mut events: ResMut<ReshockEvents>,
    mut r#move: EventWriter<api::MoveEvent>,
    mut door: EventWriter<api::DoorEvent>,
    mut view: EventWriter<api::ViewUpdateEvent>,
    mut spot: EventWriter<api::SpotEvent>,
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

    events.state = TransitionState::Active;

    match event {
        api::event::Event::Move(event) => {
            r#move.send(event);
        }
        api::event::Event::Door(event) => {
            door.send(event);
        }
        api::event::Event::View(event) => {
            view.send(event);
        }
        api::event::Event::Spot(event) => {
            spot.send(event);
        }
    }
}
