use std::collections::HashMap;

use bevy_ecs::prelude::*;
use glam::ivec2;

use crate::game::{
    component::*, Action, ActiveAction, DwimAction, MoveAction, OpenDoorAction, Reactions, Status,
};

pub fn behavior(
    action: Res<ActiveAction>,
    mut reactions: ResMut<Reactions>,
    player: Query<(Entity, &Position), With<Player>>,
    doors: Query<(Entity, &Position, &Door), Without<Player>>,
) -> Status {
    let dwim = match &action.0 {
        Some(Action::Dwim(dwim)) => dwim,
        _ => return Status::Continue,
    };

    let player = player.single();
    let (actor, position) = player;

    let direction = match dwim {
        DwimAction::UpLeft => ivec2(-1, 1),
        DwimAction::Up => ivec2(0, 1),
        DwimAction::UpRight => ivec2(1, 1),
        DwimAction::Right => ivec2(1, 0),
        DwimAction::DownRight => ivec2(1, -1),
        DwimAction::Down => ivec2(0, -1),
        DwimAction::DownLeft => ivec2(-1, -1),
        DwimAction::Left => ivec2(-1, 0),
    };

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
