use std::time::Instant;

use bevy_ecs::prelude::*;

use crate::game::component::*;
use crate::game::*;

pub fn behavior(
    action: Res<Action>,
    player: Query<(&Position, &Sight, &Memory), With<Player>>,
    entities: Query<(&Renderable, Option<&Door>, Option<&Wall>)>,
    mut reactions: ResMut<Reactions>,
) -> Status {
    match action.as_ref() {
        Action::State(StateAction::Intent) => {}
        Action::View(ViewAction::Update { actor, .. }) => {
            if player.contains(*actor) {
                reactions.0.push(Action::State(StateAction::Intent));
            }
            return Status::Continue;
        }
        _ => return Status::Continue,
    };

    let now = Instant::now();

    let (position, sight, memory) = player.single();

    let view = sight.seeing.iter().filter_map(|(e, pos)| {
        entities.get(*e).ok().map(|(renderable, door, wall)| {
            (
                e.id(),
                api::Components {
                    positions: pos.iter().cloned().map_into().collect(),
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
        .filter(|(_, cs)| cs.position.room == position.room)
        .map(|(e, cs)| {
            let pos = cs.position.coordinates - position.coordinates;
            (
                e.id(),
                api::Components {
                    positions: vec![api::PositionComponent { x: pos.x, y: pos.y }],
                    renderable: Some((&cs.renderable).into()),
                    door: cs.door.as_ref().map(|it| it.into()),
                    wall: cs.wall.as_ref().map(|it| it.into()),
                    memory: Some(api::MemoryComponent {}),
                },
            )
        })
        .chain(view)
        .collect();

    reactions.0.push(Action::State(StateAction::Update {
        state: api::State { entities },
    }));

    let duration = Instant::now() - now;
    log::debug!("Time taken: {}µs", duration.as_micros());

    Status::Continue
}
