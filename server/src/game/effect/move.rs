use bevy_ecs::prelude::*;

use crate::game::component::*;
use crate::game::{Events, *};

pub fn effect(
    action: Res<ActiveAction>,
    sight: Query<&Sight, With<Player>>,
    mut positions: Query<&mut Position>,
    mut events: ResMut<Events>,
) {
    let MoveAction {
        actor,
        position: target,
    } = match &action.0 {
        Some(Action::Move(r#move)) => r#move,
        _ => return,
    };

    let mut position = positions.get_mut(*actor).unwrap();

    position.0 = *target;

    let sight = sight.single();

    if sight.seeing.contains(actor) {
        events.0.push(api::Event {
            event: Some(api::event::Event::Move(api::MoveEvent {
                actor: actor.id(),
                x: target.x,
                y: target.y,
            })),
        });
    }
}
