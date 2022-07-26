use api::death_event::DeathSound;
use bevy_ecs::prelude::*;

use crate::game::{component::*, Events, *};

pub fn effect(
    action: Res<Action>,
    sight: Query<&Sight, With<Player>>,
    mut events: ResMut<Events>,
    mut commands: Commands,
) {
    let (actor, kind) = match action.as_ref() {
        Action::Death(DeathAction { actor, kind }) => (actor, kind),
        _ => return,
    };

    commands
        .entity(*actor)
        .remove_bundle::<(Alive, Solid, Opaque, Vulnerable, Sight, Memory, AI)>();

    let sight = sight.single();

    if !sight.seeing.contains_key(actor) {
        return;
    }

    let sound = match kind {
        Alive::Human => DeathSound::Human,
        Alive::ServBot => DeathSound::ServBot,
    };

    events.0.push(api::Event {
        event: Some(api::event::Event::Death(api::DeathEvent {
            actor: actor.id(),
            sound: sound as i32,
        })),
    });
}
