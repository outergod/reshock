use std::time::Duration;

use api::hit_event::{HitDirection, HitKind};
use bevy::{math::ivec2, prelude::*};
use bevy_kira_audio::Audio;

use crate::{bundle, component::*};

const LEAD_PIPE_SOUND: &'static str = "sshock/sounds/00222.wav";
const LASER_RAPIER_SOUND: &'static str = "sshock/sounds/00232.wav";
const APPENDAGES_SOUND: &'static str = "sshock/sounds/00256.wav";

pub fn system(
    mut commands: Commands,
    mut reader: EventReader<api::HitEvent>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for api::HitEvent {
        kind,
        direction,
        position,
        ..
    } in reader.iter()
    {
        let (x, y) = match position {
            Some(pos) => (pos.x, pos.y),
            None => continue,
        };

        if let Some(sound) = match HitKind::from_i32(*kind) {
            Some(HitKind::LeadPipe) => Some(LEAD_PIPE_SOUND),
            Some(HitKind::LaserRapier) => Some(LASER_RAPIER_SOUND),
            Some(HitKind::Appendages) => Some(APPENDAGES_SOUND),
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

        commands.spawn_bundle(bundle::Effect {
            position: Position(ivec2(x, y)),
            renderable: Renderable {
                char,
                ordering: Ordering::Effect,
                ..default()
            },
            effect: Effect {
                lifetime: Timer::new(Duration::from_secs_f32(0.3), false),
            },
        });
    }
}
