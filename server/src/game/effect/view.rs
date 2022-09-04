use bevy_ecs::prelude::*;

use crate::game::component::*;
use crate::game::*;

pub fn effect(action: Res<ActiveAction>, mut viewers: Query<&mut Sight>) {
    let (actor, sight) = match &action.0 {
        Some(Action::View(ViewAction::Update { actor, sight })) => (actor, sight),
        _ => return,
    };

    let mut view = viewers.get_mut(*actor).unwrap();

    view.seeing = sight.seeing.clone();
    view.mask = sight.mask.clone();
}
