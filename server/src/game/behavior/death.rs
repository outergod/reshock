use bevy_ecs::prelude::*;

use crate::game::{component::*, Status, *};

pub fn behavior(
    action: Res<Action>,
    alives: Query<&Alive>,
    vulnerables: Query<&Vulnerable>,
    descriptions: Query<&Description>,
    mut reactions: ResMut<Reactions>,
) -> Status {
    let actor = match action.as_ref() {
        Action::Death(it) => it.actor,
        Action::HealthLoss(HealthLossAction { actor, amount }) => {
            let vulnerable = vulnerables.get(*actor).unwrap();
            if let Ok(alive) = alives.get(*actor) && vulnerable.hp.saturating_sub(*amount) == 0 {
                reactions.0.push(Action::Death(DeathAction {
                    actor: *actor,
                    kind: *alive,
                }));
            }
            return Status::Continue;
        }
        _ => return Status::Continue,
    };

    if let Ok(description) = descriptions.get(actor) {
        let log = Action::Log(format!("{} dies", description.to_capitalized_string()));
        reactions.0.push(log);
    }

    Status::Continue
}
