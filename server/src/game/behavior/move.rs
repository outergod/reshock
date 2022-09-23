use bevy_ecs::prelude::*;

use crate::game::{component::*, *};

pub fn behavior(
    action: Res<Action>,
    mut reactions: ResMut<Reactions>,
    mut followups: ResMut<FollowUps>,
    positions: Query<&Position>,
    gateways: Query<(&Position, &Gateway)>,
    obstacles: Query<(&Position, Option<&Description>), With<Solid>>,
    player: Query<(), With<Player>>,
) -> Status {
    let (actor, delta) = match action.as_ref() {
        Action::Move(MoveAction::Intent { actor, delta }) => (*actor, *delta),
        _ => return Status::Continue,
    };

    let mut position = positions.get(actor).unwrap();

    if let Some((_, gateway)) = gateways
        .iter()
        .find(|(pos, gateway)| *pos == position && gateway.passthrough(&delta))
    {
        position = gateways.get(gateway.twin).unwrap().0;
    }

    let target = *position + delta;

    if let Some(desc) = obstacles
        .iter()
        .find_map(|(p, d)| (p == &target).then_some(d))
    {
        let mut actions = Vec::new();

        player.contains(actor).then(|| {
            let object = match desc {
                Some(it) => it.to_string(),
                None => "something".to_string(),
            };

            actions.push(Action::Log(format!("You run into {}", object)));
        });

        return Status::Reject(actions);
    };

    reactions.0.push(Action::Move(MoveAction::Update {
        actor,
        delta,
        position: target,
    }));
    followups.0.push(Action::EndTurn(actor));

    Status::Continue
}
