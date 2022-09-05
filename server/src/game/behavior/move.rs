use bevy_ecs::prelude::*;

use crate::game::{component::*, resource::*, *};

pub fn behavior(
    action: Res<Action>,
    mut followups: ResMut<FollowUps>,
    positions: Query<&Position>,
    obstacles: Query<(&Position, Option<&Description>), With<Solid>>,
    deltas: Res<Deltas>,
    player: Query<(), With<Player>>,
) -> Status {
    let MoveAction {
        actor,
        position: target,
    } = match action.as_ref() {
        Action::Move(r#move) => r#move,
        _ => return Status::Continue,
    };

    let position = match positions.get(*actor) {
        Ok(it) => it.0,
        Err(_) => {
            log::warn!(
                "Invalid move action, entity {:?} does not have Position component",
                actor
            );
            return Status::Reject(vec![]);
        }
    };

    if !deltas.0.iter().any(|d| position + *d == *target) {
        log::info!("Invalid move action, {:?} is out of reach", target);
        return Status::Reject(vec![]);
    }

    if let Some(desc) = obstacles
        .iter()
        .find_map(|(p, d)| (p.0 == *target).then_some(d))
    {
        let mut actions = Vec::new();

        player.contains(*actor).then(|| {
            let object = match desc {
                Some(it) => it.to_string(),
                None => "something".to_string(),
            };

            actions.push(Action::Log(format!("You run into {}", object)));
        });

        log::info!("Entity can't move to {:?}", target);
        return Status::Reject(actions);
    };

    followups.0.push(Action::EndTurn(*actor));

    Status::Continue
}
