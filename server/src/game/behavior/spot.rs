use api::spot_event::SpotSound;
use bevy_ecs::prelude::*;

use crate::game::component::*;
use crate::game::*;

pub fn behavior(
    action: Res<Action>,
    ai: Query<(&AI, &Memory, Option<&Description>)>,
    player: Query<Entity, With<Player>>,
    mut followups: ResMut<FollowUps>,
) -> Status {
    let (actor, sight) = match action.as_ref() {
        Action::View(ViewAction::Update { actor, sight }) => (actor, sight),
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

                followups.0.push(Action::Spot(SpotAction {
                    actor: *actor,
                    sound,
                }));

                if let Some(desc) = desc {
                    followups.0.push(Action::Log(format!(
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
