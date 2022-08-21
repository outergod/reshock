use bevy_ecs::prelude::*;

use crate::game::component::*;
use crate::game::{Events, *};

pub fn effect(
    action: Res<ActiveAction>,
    sight: Query<&Sight, With<Player>>,
    mut positions: Query<(Entity, &mut Position)>,
    mut events: ResMut<Events>,
    mut reactions: ResMut<Reactions>,
) {
    let MoveAction {
        entity,
        position: target,
    } = match &action.0 {
        Some(Action::Move(r#move)) => r#move,
        _ => return,
    };

    match positions
        .iter_mut()
        .find_map(|(e, p)| if e == *entity { Some(p) } else { None })
    {
        Some(mut position) => {
            position.0 = *target;
            reactions.0.push(Action::View);

            if let Ok(sight) = sight.get_single() {
                if sight.seeing.contains(entity) {
                    events.0.push(api::Event {
                        event: Some(api::event::Event::Move(api::MoveEvent {
                            entity: entity.id(),
                            x: target.x,
                            y: target.y,
                        })),
                    });
                }
            }
        }
        None => {
            log::warn!(
                "Invalid move action, entity {:?} does not have Position component",
                entity
            );
        }
    }
}
