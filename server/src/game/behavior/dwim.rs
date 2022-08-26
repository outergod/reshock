use std::ops::Not;

use bevy_ecs::prelude::*;
use glam::ivec2;

use crate::game::{component::*, resource::*, *};

pub fn r#move(
    action: Res<ActiveAction>,
    mut reactions: ResMut<Reactions>,
    player: Query<(Entity, &Position), With<Player>>,
    doors: Query<&Door>,
    spatial: Res<SpatialHash>,
) -> Status {
    let direction = match &action.0 {
        Some(Action::Dwim(DwimAction::UpLeft)) => ivec2(-1, 1),
        Some(Action::Dwim(DwimAction::Up)) => ivec2(0, 1),
        Some(Action::Dwim(DwimAction::UpRight)) => ivec2(1, 1),
        Some(Action::Dwim(DwimAction::Right)) => ivec2(1, 0),
        Some(Action::Dwim(DwimAction::DownRight)) => ivec2(1, -1),
        Some(Action::Dwim(DwimAction::Down)) => ivec2(0, -1),
        Some(Action::Dwim(DwimAction::DownLeft)) => ivec2(-1, -1),
        Some(Action::Dwim(DwimAction::Left)) => ivec2(-1, 0),
        _ => return Status::Continue,
    };

    let (actor, position) = player.single();
    let target = position.0 + direction;

    if let Some(target) = spatial
        .door_at(&target)
        .and_then(|entity| doors.get(entity).unwrap().open.not().then_some(entity))
    {
        reactions
            .0
            .push(Action::OpenDoor(OpenDoorAction { target, actor }));
    } else if let Some(target) = spatial.vulnerable_at(&target) {
        reactions.0.push(Action::Melee(MeleeAttackAction {
            target,
            actor,
            weapon: None,
        }));
    } else {
        reactions.0.push(Action::Move(MoveAction {
            actor,
            position: target,
        }));
    }

    Status::Accept
}

pub fn close(
    action: Res<ActiveAction>,
    mut reactions: ResMut<Reactions>,
    player: Query<(Entity, &Position), With<Player>>,
    doors: Query<&Door>,
    deltas: Res<Deltas>,
    spatial: Res<SpatialHash>,
) -> Status {
    match &action.0 {
        Some(Action::Dwim(DwimAction::Close)) => {}
        _ => return Status::Continue,
    };

    let (actor, position) = player.single();

    match deltas.0.iter().find_map(|delta| {
        spatial
            .door_at(&(position.0 + *delta))
            .and_then(|entity| doors.get(entity).unwrap().open.then_some(entity))
    }) {
        Some(target) => {
            reactions
                .0
                .push(Action::CloseDoor(CloseDoorAction { target, actor }));

            Status::Accept
        }
        None => {
            let action = Action::Log("There is no door to close nearby".to_string());
            Status::Reject(Some(action))
        }
    }
}
