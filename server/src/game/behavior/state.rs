use std::time::Instant;

use bevy_ecs::prelude::*;

use crate::game::component::*;
use crate::game::*;

pub fn behavior(
    action: Res<Action>,
    player: Query<(&Sight, &Memory), With<Player>>,
    entities: Query<(&Position, &Renderable, Option<&Door>, Option<&Wall>)>,
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

    let (sight, memory) = player.single();

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

    reactions.0.push(Action::State(StateAction::Update {
        state: api::State { entities },
    }));

    let duration = Instant::now() - now;
    log::debug!("Time taken: {}µs", duration.as_micros());

    Status::Continue
}
