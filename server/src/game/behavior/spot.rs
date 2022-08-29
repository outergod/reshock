use bevy_ecs::prelude::*;

use crate::game::component::*;
use crate::game::*;

pub fn behavior(
    mut action: ResMut<ActiveAction>,
    entities: Query<(&AI, Option<&Description>)>,
    mut reactions: ResMut<Reactions>,
) -> Status {
    match action.0.as_mut() {
        Some(Action::Spot(SpotAction { actor, sound })) => match entities.get(*actor) {
            Ok((AI::ServBot, desc)) => {
                *sound = Some(api::spot_event::SpotSound::ServBot);
                if let Some(desc) = desc {
                    reactions.0.push(Action::Log(format!(
                        "{} has spotted you!",
                        desc.to_capitalized_string()
                    )));
                };

                Status::Accept
            }
            _ => Status::Reject(None),
        },
        _ => Status::Continue,
    }
}
