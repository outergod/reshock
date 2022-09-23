use std::collections::HashSet;

use bevy_ecs::prelude::*;
use glam::ivec2;

use crate::game::{component::*, resource::*, *};

pub fn r#move(
    action: Res<Action>,
    mut reactions: ResMut<Reactions>,
    player: Query<(Entity, &Position), With<Player>>,
    doors: Query<(Entity, &Position, &Door)>,
    gateways: Query<(&Position, &Gateway)>,
    vulnerables: Query<(Entity, &Position), With<Vulnerable>>,
    switches: Query<(Entity, &Position), With<Switch>>,
) -> Status {
    let delta = match action.as_ref() {
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

    // If passing through a gateway, get the future position from the gateway's twin
    let position = match gateways
        .iter()
        .find(|(pos, gateway)| *pos == position && gateway.passthrough(&delta))
    {
        Some((_, gateway)) => gateways.get(gateway.twin).unwrap().0,
        None => position,
    };

    let target = *position + delta;

    if let Some(target) = doors
        .iter()
        .find_map(|(entity, pos, door)| (pos == &target && !door.open).then_some(entity))
    {
        reactions
            .0
            .push(Action::OpenDoor(OpenDoorAction { target, actor }));
    } else if let Some(target) = vulnerables
        .iter()
        .find_map(|(entity, pos)| (pos == &target).then_some(entity))
    {
        reactions.0.push(Action::Melee(MeleeAttackAction::Intent {
            target,
            actor,
            direction: delta.into(),
        }));
    } else if let Some(target) = switches
        .iter()
        .find_map(|(entity, pos)| (pos == &target).then_some(entity))
    {
        reactions
            .0
            .push(Action::ToggleSwitch(ToggleSwitchAction { target, actor }));
    } else {
        reactions
            .0
            .push(Action::Move(MoveAction::Intent { actor, delta }));
    }

    Status::Continue
}

pub fn close(
    action: Res<Action>,
    mut reactions: ResMut<Reactions>,
    player: Query<(Entity, &Position), With<Player>>,
    gateways: Query<(&Position, &Gateway)>,
    doors: Query<(Entity, &Position, &Door)>,
    deltas: Res<Deltas>,
) -> Status {
    match action.as_ref() {
        Action::Dwim(DwimAction::Close) => {}
        _ => return Status::Continue,
    };

    let (actor, position) = player.single();

    let neighbors: HashSet<_> = match gateways.iter().find(|(pos, _)| *pos == position) {
        Some((pos, gateway)) => {
            let twin = gateways.get(gateway.twin).unwrap().0;
            deltas
                .0
                .iter()
                .map(|d| {
                    if gateway.passthrough(d) {
                        *twin + *d
                    } else {
                        *pos + *d
                    }
                })
                .collect()
        }
        None => deltas.0.iter().map(|d| *position + *d).collect(),
    };

    match neighbors.into_iter().find_map(|neighbor| {
        doors
            .iter()
            .find_map(|(entity, pos, door)| (pos == &neighbor && door.open).then_some(entity))
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

    match npcs.iter().find(|e| sight.seeing.contains_key(&e)) {
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
