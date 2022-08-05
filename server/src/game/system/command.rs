use std::collections::HashMap;

use bevy_ecs::prelude::*;
use glam::ivec2;

use crate::game::component::*;
use crate::game::Command;
use crate::game::GameEvents;

pub fn system(
    mut commands: EventReader<Command>,
    mut events: ResMut<GameEvents>,
    mut player: Query<(Entity, &mut Position), With<Player>>,
    mut obstacles: Query<(Entity, &Obstacle, &Position, Option<&mut Door>), Without<Player>>,
) {
    log::debug!("???");

    let command = match commands.iter().next() {
        Some(command) => command,
        None => return,
    };

    let player = player.single_mut();
    let (actor, mut position) = player;

    let direction = match command {
        Command::UpLeft => ivec2(-1, 1),
        Command::Up => ivec2(0, 1),
        Command::UpRight => ivec2(1, 1),
        Command::Right => ivec2(1, 0),
        Command::DownRight => ivec2(1, -1),
        Command::Down => ivec2(0, -1),
        Command::DownLeft => ivec2(-1, -1),
        Command::Left => ivec2(-1, 0),
    };

    let mut neighbors: HashMap<_, _> = obstacles
        .iter_mut()
        .filter_map(|(e, o, p, d)| {
            if !o.0 {
                return None;
            }

            let direction = p.0 - position.0;
            if direction.x.abs() <= 1 && direction.y.abs() <= 1 {
                Some((direction, (e, d)))
            } else {
                None
            }
        })
        .collect();

    match neighbors.get_mut(&direction) {
        Some((entity, Some(door))) => {
            door.open = true;
            events.0.push(api::Event {
                event: Some(api::event::Event::Door(api::DoorEvent {
                    actor: actor.id(),
                    door: entity.id(),
                    open: true,
                })),
            });
        }
        Some(_) => {
            // TODO In-game logging
            log::info!("Player can't move to {:?}", position.0 + direction);
        }
        None => {
            position.0 += direction;
            events.0.push(api::Event {
                event: Some(api::event::Event::Move(api::MoveEvent {
                    entity: actor.id(),
                    x: position.0.x,
                    y: position.0.y,
                })),
            });
        }
    }
}
