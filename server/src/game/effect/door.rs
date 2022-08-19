use bevy_ecs::prelude::*;

use crate::game::Events;
use crate::game::{component::*, Action, ActiveAction, OpenDoorAction};

pub fn effect(
    action: Res<ActiveAction>,
    mut doors: Query<(Entity, &mut Door)>,
    mut commands: Commands,
    mut events: ResMut<Events>,
) {
    let OpenDoorAction { actor, entity } = match &action.0 {
        Some(Action::OpenDoor(it)) => it,
        _ => return,
    };

    match doors
        .iter_mut()
        .find_map(|(e, d)| if e == *entity { Some(d) } else { None })
    {
        Some(mut door) => {
            door.open = true;
            commands
                .entity(*entity)
                .remove::<Solid>()
                .remove::<Opaque>();
            events.0.push(api::Event {
                event: Some(api::event::Event::Door(api::DoorEvent {
                    actor: actor.id(),
                    door: entity.id(),
                    open: true,
                })),
            });
        }
        None => {
            log::warn!(
                "Invalid open door action, entity {:?} does not have Door component",
                entity
            );
        }
    }
}
