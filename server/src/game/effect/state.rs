use std::time::Instant;

use bevy_ecs::prelude::*;

use crate::game::component::*;
use crate::game::{Events, *};

pub fn effect(
    action: Res<ActiveAction>,
    player: Query<(Entity, &Sight, &Memory), With<Player>>,
    entities: Query<(&Position, &Renderable, Option<&Door>, Option<&Wall>)>,
    mut state: ResMut<api::State>,
    mut events: ResMut<Events>,
) {
    match action.0 {
        Some(Action::View) => {}
        _ => return,
    }

    let now = Instant::now();

    let (player, sight, memory) = player.single();

    let view = sight.seeing.iter().filter_map(|e| {
        entities
            .get(*e)
            .ok()
            .map(|(position, renderable, door, wall)| {
                (
                    e.id(),
                    api::Components {
                        position: Some(position.into()),
                        renderable: Some(renderable.into()),
                        door: door.map(|it| it.into()),
                        wall: wall.map(|it| it.into()),
                        memory: None,
                    },
                )
            })
    });

    let entities = memory
        .0
        .iter()
        .map(|(e, cs)| (e.id(), cs.into()))
        .chain(view)
        .collect();

    *state = api::State { entities };

    events.0.push(api::Event {
        event: Some(api::event::Event::View(api::ViewUpdateEvent {
            player: player.id(),
            view: Some(state.clone()),
        })),
    });

    let duration = Instant::now() - now;
    log::debug!("Time taken: {}µs", duration.as_micros());
}
