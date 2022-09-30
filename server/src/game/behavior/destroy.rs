use bevy_ecs::prelude::*;

use crate::game::{component::*, Status, *};

pub fn behavior(
    action: Res<Action>,
    destructibles: Query<&Destructible>,
    vulnerables: Query<&Vulnerable>,
    descriptions: Query<&Description>,
    mut reactions: ResMut<Reactions>,
) -> Status {
    let actor = match action.as_ref() {
        Action::Destroy(it) => it.actor,
        Action::HealthLoss(HealthLossAction { actor, amount }) => {
            let vulnerable = vulnerables.get(*actor).unwrap();
            if let Ok(destructible) = destructibles.get(*actor) && vulnerable.hp.saturating_sub(*amount) == 0 {
                reactions.0.push(Action::Destroy(DestroyAction {
                    actor: *actor,
                    kind: *destructible,
                }));
            }
            return Status::Continue;
        }
        _ => return Status::Continue,
    };

    if let Ok(description) = descriptions.get(actor) {
        let log = Action::Log(format!(
            "{} is destroyed",
            description.to_capitalized_string()
        ));
        reactions.0.push(log);
    }

    Status::Continue
}
