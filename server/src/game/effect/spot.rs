use bevy_ecs::prelude::*;
use itertools::Itertools;

use crate::game::component::*;
use crate::game::{Events, *};

pub fn effect(
    action: Res<ActiveAction>,
    viewers: Query<(Entity, &Sight, &Memory), With<AI>>,
    player: Query<Entity, With<Player>>,
    mut events: ResMut<Events>,
    mut reactions: ResMut<Reactions>,
) {
    match action.0 {
        Some(Action::View) => {
            let player = match player.get_single() {
                Ok(it) => it,
                Err(_) => return,
            };

            for (actor, sight, memory) in viewers.iter() {
                let mut entities = memory.0.keys();

                if sight.seeing.contains(&player) && !entities.contains(&player) {
                    reactions
                        .0
                        .push(Action::Spot(SpotAction { actor, sound: None }));
                }
            }
        }
        Some(Action::Spot(SpotAction {
            actor,
            sound: Some(sound),
        })) => {
            events.0.push(api::Event {
                event: Some(api::event::Event::Spot(api::SpotEvent {
                    actor: actor.id(),
                    sound: sound as i32,
                })),
            });
        }
        _ => return,
    }
}
