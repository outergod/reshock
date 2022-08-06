use bevy_ecs::prelude::*;

use crate::game::{component::*, Action, ActiveAction, FollowUps, MoveAction, Status};

pub fn behavior(
    action: Res<ActiveAction>,
    mut followups: ResMut<FollowUps>,
    positions: Query<Entity, With<Position>>,
    obstacles: Query<&Position, With<Solid>>,
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

    if positions.iter().find(|e| e == entity).is_none() {
        log::warn!(
            "Invalid move action, entity {:?} does not have Position component",
            entity
        );
        return Status::Reject;
    }

    followups.0.push(Action::EndTurn);

    Status::Accept
}
