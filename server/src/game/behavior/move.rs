use bevy_ecs::prelude::*;

use crate::game::{component::*, resource::*, *};

trait ArticleFor {
    fn article_for(&self) -> String;
}

impl ArticleFor for &str {
    fn article_for(&self) -> String {
        match self.chars().next() {
            Some('a') | Some('e') | Some('i') | Some('o') => "an",
            _ => "a",
        }
        .to_string()
    }
}

pub fn behavior(
    action: Res<ActiveAction>,
    mut followups: ResMut<FollowUps>,
    positions: Query<&Position>,
    obstacles: Query<(&Position, Option<&Description>), With<Solid>>,
    deltas: Res<Deltas>,
    player: Query<(), With<Player>>,
) -> Status {
    let MoveAction {
        entity,
        position: target,
    } = match &action.0 {
        Some(Action::Move(r#move)) => r#move,
        _ => return Status::Continue,
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

    if let Some(desc) = obstacles
        .iter()
        .find_map(|(p, d)| (p.0 == *target).then_some(d))
    {
        if player.contains(*entity) {
            let object = match desc {
                Some(it) => it.to_string(),
                None => "something".to_string(),
            };

            followups
                .0
                .push(Action::Log(format!("You run into {}", object)));
        }

        log::info!("Entity can't move to {:?}", target);
        return Status::Reject;
    };

    followups.0.push(Action::EndTurn(*entity));

    Status::Accept
}
