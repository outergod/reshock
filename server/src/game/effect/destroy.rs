use bevy_ecs::prelude::*;

use crate::game::{component::*, Events, *};

pub fn effect(
    action: Res<Action>,
    sight: Query<&Sight, With<Player>>,
    mut events: ResMut<Events>,
    mut commands: Commands,
) {
    let (actor, kind) = match action.as_ref() {
        Action::Destroy(DestroyAction { actor, kind }) => (actor, kind),
        _ => return,
    };

    commands.entity(*actor).despawn();

    let sight = sight.single();

    if !sight.seeing.contains_key(actor) {
        return;
    }

    events.0.push(api::Event {
        event: Some(api::event::Event::Destruction(api::DestructionEvent {
            actor: actor.id(),
        })),
    });
}
