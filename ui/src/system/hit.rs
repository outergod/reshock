use std::time::Duration;

use api::hit_event::{HitDirection, HitKind};
use bevy::prelude::*;
use bevy_kira_audio::Audio;

use crate::{bundle, component::*};

const LEAD_PIPE_SOUND: &'static str = "sshock/sounds/00222.wav";
const LASER_RAPIER_SOUND: &'static str = "sshock/sounds/00232.wav";

pub fn system(
    mut commands: Commands,
    mut reader: EventReader<api::HitEvent>,
    positions: Query<(&ReshockEntity, &Position)>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for api::HitEvent {
        target,
        kind,
        direction,
    } in reader.iter()
    {
        if let Some(sound) = match HitKind::from_i32(*kind) {
            Some(HitKind::LeadPipe) => Some(LEAD_PIPE_SOUND),
            Some(HitKind::LaserRapier) => Some(LASER_RAPIER_SOUND),
            _ => None,
        } {
            audio.play(asset_server.load(sound));
        }

        let char = match api::hit_event::HitDirection::from_i32(*direction) {
            Some(HitDirection::Top) | Some(HitDirection::Bottom) => '-',
            Some(HitDirection::TopRight) | Some(HitDirection::BottomLeft) => '\\',
            Some(HitDirection::Right) | Some(HitDirection::Left) => '|',
            Some(HitDirection::BottomRight) | Some(HitDirection::TopLeft) => '/',
            _ => '*',
        };

        let position = positions
            .iter()
            .find_map(|(entity, pos)| (&entity.0 == target).then_some(*pos))
            .unwrap();

        commands.spawn_bundle(bundle::Effect {
            position,
            renderable: Renderable { char, ..default() },
            ordering: Ordering::Effect,
            effect: Effect {
                lifetime: Timer::new(Duration::from_secs_f32(0.3), false),
            },
        });
    }
}
