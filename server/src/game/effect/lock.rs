use bevy_ecs::prelude::*;

use crate::game::{component::*, *};

pub fn activate(action: Res<Action>, mut locks: Query<&mut Lock>) {
    let ActivateLockAction { target, .. } = match action.as_ref() {
        Action::ActivateLock(it) => *it,
        _ => return,
    };

    let mut lock = locks.get_mut(target).unwrap();

    lock.active = true;
}

pub fn deactivate(action: Res<Action>, mut locks: Query<&mut Lock>) {
    let DeactivateLockAction { target, .. } = match action.as_ref() {
        Action::DeactivateLock(it) => *it,
        _ => return,
    };

    let mut lock = locks.get_mut(target).unwrap();

    lock.active = false;
}
