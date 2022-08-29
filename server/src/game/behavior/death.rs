use bevy_ecs::prelude::*;

use crate::game::{component::*, Status, *};

pub fn behavior(
    mut action: ResMut<ActiveAction>,
    alives: Query<&Alive>,
    descriptions: Query<&Description>,
    mut reactions: ResMut<Reactions>,
) -> Status {
    let mut action = match action.0.as_mut() {
        Some(Action::Death(it)) => it,
        _ => return Status::Continue,
    };

    action.kind = match alives.get(action.actor) {
        Ok(it) => Some(*it),
        Err(_) => {
            log::debug!("That is not dead which can eternal lie, / And with strange aeons even death may die.");
            return Status::Reject(None);
        }
    };

    if let Ok(description) = descriptions.get(action.actor) {
        let log = Action::Log(format!("{} dies", description.to_capitalized_string()));
        reactions.0.push(log);
    }

    Status::Accept
}
