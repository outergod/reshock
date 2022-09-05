use std::ops::Not;

use bevy_ecs::prelude::*;
use glam::ivec2;

use crate::game::{component::*, resource::*, *};

pub fn r#move(
    action: Res<Action>,
    mut reactions: ResMut<Reactions>,
    player: Query<(Entity, &Position), With<Player>>,
    doors: Query<&Door>,
    spatial: Res<SpatialHash>,
) -> Status {
    let direction = match action.as_ref() {
        Action::Dwim(DwimAction::UpLeft) => ivec2(-1, 1),
        Action::Dwim(DwimAction::Up) => ivec2(0, 1),
        Action::Dwim(DwimAction::UpRight) => ivec2(1, 1),
        Action::Dwim(DwimAction::Right) => ivec2(1, 0),
        Action::Dwim(DwimAction::DownRight) => ivec2(1, -1),
        Action::Dwim(DwimAction::Down) => ivec2(0, -1),
        Action::Dwim(DwimAction::DownLeft) => ivec2(-1, -1),
        Action::Dwim(DwimAction::Left) => ivec2(-1, 0),
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
        reactions
            .0
            .push(Action::Melee(MeleeAttackAction::Intent { target, actor }));
    } else {
        reactions.0.push(Action::Move(MoveAction {
            actor,
            position: target,
        }));
    }

    Status::Continue
}

pub fn close(
    action: Res<Action>,
    mut reactions: ResMut<Reactions>,
    player: Query<(Entity, &Position), With<Player>>,
    doors: Query<&Door>,
    deltas: Res<Deltas>,
    spatial: Res<SpatialHash>,
) -> Status {
    match action.as_ref() {
        Action::Dwim(DwimAction::Close) => {}
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

            Status::Continue
        }
        None => {
            let action = Action::Log("There is no door to close nearby".to_string());
            Status::Reject(vec![action])
        }
    }
}

pub fn shoot(
    action: Res<Action>,
    mut reactions: ResMut<Reactions>,
    player: Query<(Entity, &Sight), With<Player>>,
    npcs: Query<Entity, (With<Vulnerable>, Without<Player>)>,
) -> Status {
    match action.as_ref() {
        Action::Dwim(DwimAction::Shoot) => {}
        _ => return Status::Continue,
    };

    let (actor, sight) = player.single();

    match npcs.iter().find(|e| sight.seeing.contains(&e)) {
        Some(target) => {
            reactions
                .0
                .push(Action::Shoot(ShootAction::Intent { actor, target }));

            Status::Continue
        }
        None => {
            let action = Action::Log("No one to shoot at in sight".to_string());
            Status::Reject(vec![action])
        }
    }
}
