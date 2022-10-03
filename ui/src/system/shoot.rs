use api::shoot_event::*;
use bevy::{math::vec2, prelude::*};
use bevy_kira_audio::Audio;

use crate::{bundle, resource::*};

const ASSAULT_RIFLE_SOUND: &'static str = "sshock/sounds/00218.wav";
const BULLET_SPEED: f32 = 2000.0;
const BULLET_SIZE: f32 = 10.0;

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
        let start = match source {
            Some(pos) => vec2(pos.x as f32 * width, pos.y as f32 * height),
            None => continue,
        };

        let end = match target {
            Some(pos) => vec2(pos.x as f32 * width, pos.y as f32 * height),
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
                commands.spawn_bundle(bundle::Projectile::new(
                    start,
                    end,
                    BULLET_SPEED,
                    BULLET_SIZE,
                ));
            }
            _ => {
                events.transitions -= 1;
            }
        }
    }
}
