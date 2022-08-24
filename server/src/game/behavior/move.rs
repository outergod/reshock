use bevy_ecs::prelude::*;

use crate::game::{component::*, resource::*, *};

pub fn behavior(
    action: Res<ActiveAction>,
    mut followups: ResMut<FollowUps>,
    positions: Query<&Position>,
    obstacles: Query<&Position, With<Solid>>,
    deltas: Res<Deltas>,
) -> Status {
    let MoveAction {
        entity,
        position: target,
    } = match &action.0 {
        Some(Action::Move(r#move)) => r#move,
        _ => return Status::Continue,
    };

    if obstacles.iter().find(|p| p.0 == *target).is_some() {
        log::info!("Entity can't move to {:?}", target);
        return Status::Reject;
    };

    let position = match positions.get(*entity) {
        Ok(it) => it.0,
        Err(_) => {
            log::warn!(
                "Invalid move action, entity {:?} does not have Position component",
                entity
            );
            return Status::Reject;
        }
    };

    if !deltas.0.iter().any(|d| position + *d == *target) {
        log::info!("Invalid move action, {:?} is out of reach", target);
        return Status::Reject;
    }

    followups.0.push(Action::EndTurn(*entity));

    Status::Accept
}
