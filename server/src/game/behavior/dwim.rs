use std::collections::HashMap;

use bevy_ecs::prelude::*;
use glam::ivec2;

use crate::game::{component::*, resource::*, *};

pub fn r#move(
    action: Res<ActiveAction>,
    mut reactions: ResMut<Reactions>,
    player: Query<(Entity, &Position), With<Player>>,
    doors: Query<(Entity, &Position, &Door), Without<Player>>,
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

    let player = player.single();
    let (actor, position) = player;

    let doors: HashMap<_, _> = doors
        .iter()
        .filter_map(|(e, p, d)| {
            let direction = p.0 - position.0;
            if direction.x.abs() <= 1 && direction.y.abs() <= 1 && !d.open {
                Some((direction, e))
            } else {
                None
            }
        })
        .collect();

    match doors.get(&direction) {
        Some(entity) => {
            reactions.0.push(Action::OpenDoor(OpenDoorAction {
                entity: *entity,
                actor,
            }));

            Status::Accept
        }
        None => {
            reactions.0.push(Action::Move(MoveAction {
                entity: actor,
                position: position.0 + direction,
            }));
            Status::Accept
        }
    }
}

pub fn close(
    action: Res<ActiveAction>,
    mut reactions: ResMut<Reactions>,
    player: Query<(Entity, &Position), With<Player>>,
    doors: Query<(Entity, &Position, &Door), Without<Player>>,
    deltas: Res<Deltas>,
) -> Status {
    match &action.0 {
        Some(Action::Dwim(DwimAction::Close)) => {}
        _ => return Status::Continue,
    };

    let player = player.single();
    let (actor, position) = player;

    match doors
        .iter()
        .find_map(|(e, p, d)| (d.open && deltas.0.contains(&(p.0 - position.0))).then_some(e))
    {
        Some(entity) => {
            reactions
                .0
                .push(Action::CloseDoor(CloseDoorAction { entity, actor }));

            Status::Accept
        }
        None => Status::Reject(None),
    }
}
