use std::time::Duration;

use api::shoot_event::*;
use bevy::{math::vec2, prelude::*};
use bevy_kira_audio::Audio;
use bevy_prototype_lyon::prelude::*;

use crate::{component::*, resource::*};

const ASSAULT_RIFLE_SOUND: &'static str = "sshock/sounds/00218.wav";

pub fn system(
    mut commands: Commands,
    mut reader: EventReader<api::ShootEvent>,
    dimensions: Res<TileDimensions>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut events: ResMut<ReshockEvents>,
) {
    let Size { width, height } = match dimensions.0 {
        Some(it) => it,
        None => return,
    };

    for api::ShootEvent {
        source,
        target,
        kind,
        sound,
    } in reader.iter()
    {
        let (x1, y1) = match source {
            Some(pos) => ((pos.x as f32 + 0.5) * width, (pos.y as f32 - 0.5) * height),
            None => continue,
        };

        let (x2, y2) = match target {
            Some(pos) => ((pos.x as f32 + 0.5) * width, (pos.y as f32 - 0.5) * height),
            None => continue,
        };

        if let Some(sound) = match ShootSound::from_i32(*sound) {
            Some(ShootSound::Mark3AssaultRifle) => Some(ASSAULT_RIFLE_SOUND),
            _ => None,
        } {
            audio.play(asset_server.load(sound));
        }

        match ShootKind::from_i32(*kind) {
            Some(ShootKind::Projectile) => {
                let shape = shapes::Line(vec2(x1, y1), vec2(x2, y2));
                commands
                    .spawn_bundle(GeometryBuilder::build_as(
                        &shape,
                        DrawMode::Stroke(StrokeMode::new(Color::GRAY, 1.0)),
                        Transform::default(),
                    ))
                    .insert(Effect {
                        lifetime: Timer::new(Duration::from_secs_f32(0.05), false),
                        remove: true,
                    });
            }
            _ => {
                events.transitions -= 1;
            }
        }
    }
}
