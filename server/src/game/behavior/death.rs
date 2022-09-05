use bevy_ecs::prelude::*;

use crate::game::{component::*, Status, *};

pub fn behavior(
    mut action: ResMut<Action>,
    alives: Query<&Alive>,
    descriptions: Query<&Description>,
    mut reactions: ResMut<Reactions>,
) -> Status {
    let mut action = match action.as_mut() {
        Action::Death(it) => it,
        _ => return Status::Continue,
    };

    action.kind = match alives.get(action.actor) {
        Ok(it) => Some(*it),
        Err(_) => {
            log::debug!("That is not dead which can eternal lie, / And with strange aeons even death may die.");
            return Status::Reject(vec![]);
        }
    };

    if let Ok(description) = descriptions.get(action.actor) {
        let log = Action::Log(format!("{} dies", description.to_capitalized_string()));
        reactions.0.push(log);
    }

    Status::Continue
}
