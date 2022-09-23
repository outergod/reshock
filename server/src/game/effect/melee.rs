use api::hit_event::*;
use bevy_ecs::prelude::*;

use crate::game::Events;
use crate::game::{component::*, *};

pub fn effect(
    action: Res<Action>,
    mut events: ResMut<Events>,
    sight: Query<&Sight, With<Player>>,
    weapons: Query<&MeleeWeapon>,
) {
    let HitAction {
        target,
        direction,
        weapon,
        ..
    } = match action.as_ref() {
        Action::Hit(it) => *it,
        _ => return,
    };

    let sight = sight.single();

    let positions = match sight.seeing.get(&target) {
        Some(it) => it,
        None => return,
    };

    let weapon = match weapons.get(weapon) {
        Ok(it) => it,
        Err(_) => return,
    };

    let kind = match weapon.kind {
        MeleeWeaponKind::LeadPipe => HitKind::LeadPipe,
        MeleeWeaponKind::LaserRapier => HitKind::LaserRapier,
        MeleeWeaponKind::Appendages => HitKind::Appendages,
    };

    events.0.push(api::Event {
        event: Some(api::event::Event::Hit(api::HitEvent {
            positions: positions.iter().cloned().map_into().collect(),
            kind: kind as i32,
            direction: direction as i32,
        })),
    });
}
