use std::time::Duration;

use api::*;
use bevy::prelude::*;
use bevy_kira_audio::Audio;

use crate::component::Effect;
use crate::resource::ReshockEvents;

const SERV_BOT_SPOT_SOUND: &'static str = "sshock/sounds/00275.wav";
const SERV_BOT_SPOT_LENGTH: f32 = 0.845;

pub fn system(
    mut commands: Commands,
    mut reader: EventReader<api::SpotEvent>,
    mut events: ResMut<ReshockEvents>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for SpotEvent { sound, .. } in reader.iter() {
        match spot_event::SpotSound::from_i32(*sound) {
            Some(spot_event::SpotSound::ServBot) => {
                audio.play(asset_server.load(SERV_BOT_SPOT_SOUND));
                commands.spawn().insert(Effect {
                    lifetime: Timer::new(Duration::from_secs_f32(SERV_BOT_SPOT_LENGTH), false),
                    remove: true,
                });
            }
            _ => {
                events.transitions -= 1;
            }
        };
    }
}
