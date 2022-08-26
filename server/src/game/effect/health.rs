use bevy_ecs::prelude::*;

use crate::game::{component::*, *};

pub fn effect(
    action: Res<ActiveAction>,
    mut vulnerables: Query<&mut Vulnerable>,
    mut reactions: ResMut<Reactions>,
) {
    let HealthLossAction { actor, amount } = match &action.0 {
        Some(Action::HealthLoss(it)) => it,
        _ => return,
    };

    let mut vulnerable = vulnerables.get_mut(*actor).unwrap();
    vulnerable.hp = vulnerable.hp.saturating_sub(*amount);

    if vulnerable.hp == 0 {
        reactions.0.push(Action::Death(DeathAction {
            actor: *actor,
            kind: None,
        }));
    }
}
