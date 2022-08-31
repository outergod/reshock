use api::spot_event::SpotSound;
use bevy_ecs::prelude::*;

use crate::game::component::*;
use crate::game::*;

pub fn behavior(
    action: Res<ActiveAction>,
    ai: Query<(&AI, &Memory, Option<&Description>)>,
    player: Query<Entity, With<Player>>,
    mut reactions: ResMut<Reactions>,
) -> Status {
    let ViewAction { sight, actor } = match &action.0 {
        Some(Action::View(Some(it))) => it,
        _ => return Status::Continue,
    };

    let (ai, memory, description) = match ai.get(*actor) {
        Ok(it) => it,
        Err(_) => return Status::Continue,
    };

    let player = player.single();

    if sight.seeing.contains(&player) && !memory.0.contains_key(&player) {
        match (ai, description) {
            (AI::ServBot, desc) => {
                let sound = SpotSound::ServBot;

                reactions.0.push(Action::Spot(SpotAction {
                    actor: *actor,
                    sound,
                }));

                if let Some(desc) = desc {
                    reactions.0.push(Action::Log(format!(
                        "{} has spotted you!",
                        desc.to_capitalized_string()
                    )));
                };
            }

            _ => {}
        }
    }

    Status::Continue
}
