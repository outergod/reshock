use bevy_ecs::prelude::*;

use crate::game::component::*;
use crate::game::Events;

pub fn effect(
    player: Query<(Entity, &Sight, &Memory), With<Player>>,
    entities: Query<(
        Entity,
        &Position,
        &Renderable,
        &Ordering,
        Option<&Door>,
        Option<&Wall>,
    )>,
    mut state: ResMut<api::State>,
    mut events: ResMut<Events>,
) {
    let (player, sight, memory) = player.get_single().expect("Player not found");

    let memory = memory
        .0
        .iter()
        .filter_map(|(e, cs)| (!sight.seeing.contains(&e)).then_some((e.id(), cs.into())));

    let entities = entities
        .iter()
        .filter_map(|(entity, position, renderable, ordering, door, wall)| {
            sight.seeing.contains(&entity).then_some((
                entity.id(),
                api::Components {
                    position: Some(position.into()),
                    renderable: Some(renderable.into()),
                    ordering: Some(ordering.into()),
                    door: door.map(|it| it.into()),
                    wall: wall.map(|it| it.into()),
                    memory: None,
                },
            ))
        })
        .chain(memory)
        .collect();

    *state = api::State { entities };

    events.0.push(api::Event {
        event: Some(api::event::Event::View(api::ViewUpdateEvent {
            player: player.id(),
            view: Some(state.clone()),
        })),
    });
}
