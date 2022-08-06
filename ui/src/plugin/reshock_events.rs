use bevy::log;
use bevy::prelude::*;

use crate::resource::ReshockEvents;

pub struct ReshockEventsPlugin;

impl Plugin for ReshockEventsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ReshockEvents>()
            .add_event::<api::MoveEvent>()
            .add_event::<api::DoorEvent>()
            .add_event::<api::ViewUpdateEvent>()
            .add_event::<api::MemoryUpdateEvent>()
            .add_system(system);
    }
}

pub fn system(
    mut events: ResMut<ReshockEvents>,
    mut r#move: EventWriter<api::MoveEvent>,
    mut door: EventWriter<api::DoorEvent>,
    mut view: EventWriter<api::ViewUpdateEvent>,
    mut memory: EventWriter<api::MemoryUpdateEvent>,
) {
    let event = match events.0.pop_front() {
        Some(api::Event { event: Some(it) }) => it,
        Some(api::Event { event: None }) => {
            log::warn!("Received empty event");
            return;
        }
        None => return,
    };

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
        api::event::Event::Memory(event) => {
            memory.send(event);
        }
    }
}
