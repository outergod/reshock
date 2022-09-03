use api::hit_event::*;
use bevy_ecs::prelude::*;

use crate::game::Events;
use crate::game::{component::*, *};

pub fn effect(
    action: Res<ActiveAction>,
    sight: Query<&Sight, With<Player>>,
    weapons: Query<&MeleeWeapon>,
    positions: Query<&Position>,
    mut events: ResMut<Events>,
) {
    let (actor, target, weapon) = match &action.0 {
        Some(Action::Hit(HitAction {
            actor,
            target,
            weapon,
            ..
        })) => (actor, target, weapon),
        _ => return,
    };

    let sight = sight.single();

    if !sight.seeing.contains(target) {
        return;
    }

    let weapon = match weapons.get(*weapon) {
        Ok(it) => it,
        Err(_) => return,
    };

    let kind = match weapon.kind {
        MeleeWeaponKind::LeadPipe => HitKind::LeadPipe,
        MeleeWeaponKind::LaserRapier => HitKind::LaserRapier,
        MeleeWeaponKind::Appendages => HitKind::Appendages,
    };

    let pos = positions.get(*target).unwrap().0;

    let direction = {
        let actor = positions.get(*actor).unwrap();
        let (x, y) = (pos - actor.0).into();
        match (x, y) {
            (0, 1) => HitDirection::Top,
            (1, 1) => HitDirection::TopRight,
            (1, 0) => HitDirection::Right,
            (1, -1) => HitDirection::BottomRight,
            (0, -1) => HitDirection::Bottom,
            (-1, -1) => HitDirection::BottomLeft,
            (-1, 0) => HitDirection::Left,
            (-1, 1) => HitDirection::TopLeft,
            _ => HitDirection::None,
        }
    };

    events.0.push(api::Event {
        event: Some(api::event::Event::Hit(api::HitEvent {
            position: Some(api::Position { x: pos.x, y: pos.y }),
            kind: kind as i32,
            direction: direction as i32,
        })),
    });
}
