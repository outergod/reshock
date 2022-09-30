use bevy_ecs::prelude::*;

use crate::game::{component::*, *};

pub fn effect(action: Res<Action>, mut vulnerables: Query<&mut Vulnerable>) {
    let HealthLossAction { actor, amount } = match action.as_ref() {
        Action::HealthLoss(it) => it,
        _ => return,
    };

    let mut vulnerable = vulnerables.get_mut(*actor).unwrap();
    vulnerable.hp = vulnerable.hp.saturating_sub(*amount);
}
