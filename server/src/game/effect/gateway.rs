use bevy_ecs::prelude::*;

use crate::game::{component::*, *};

pub fn effect(
    action: Res<Action>,
    doors: Query<(Entity, &Position), With<Door>>,
    mut commands: Commands,
) {
    let GatewaySpawnAction {
        lhs,
        rhs,
        direction,
    } = match action.as_ref() {
        Action::SpawnGateway(it) => it,
        _ => return,
    };

    let lhs = doors
        .iter()
        .find_map(|(e, pos)| (pos == lhs).then_some(e))
        .unwrap();
    let rhs = doors
        .iter()
        .find_map(|(e, pos)| (pos == rhs).then_some(e))
        .unwrap();

    commands.entity(lhs).insert(Gateway {
        twin: rhs,
        direction: *direction,
    });

    commands.entity(rhs).insert(Gateway {
        twin: lhs,
        direction: direction.reverse(),
    });
}
