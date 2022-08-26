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
        Some(Action::Melee(MeleeAttackAction {
            actor,
            target,
            weapon: Some(weapon),
        })) => (actor, target, weapon),
        _ => return,
    };

    let sight = sight.single();

    if !sight.seeing.contains(target) {
        return;
    }

    let weapon = weapons.get(*weapon).unwrap();
    let kind = match weapon.kind {
        MeleeWeaponKind::LeadPipe => HitKind::LeadPipe,
        MeleeWeaponKind::LaserRapier => HitKind::LaserRapier,
    };

    let direction = {
        let actor = positions.get(*actor).unwrap();
        let target = positions.get(*target).unwrap();
        let (x, y) = (target.0 - actor.0).into();
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
            target: target.id(),
            kind: kind as i32,
            direction: direction as i32,
        })),
    });
}
