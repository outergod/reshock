use api::*;
use bevy::prelude::*;
use bevy_kira_audio::Audio;

use crate::resource::ReshockEvents;
use crate::resource::TransitionState;

const SERV_BOT_SPOT_SOUND: &'static str = "sshock/sounds/00275.wav";
// const SERV_BOT_SPOT_LENGTH: f32 = 0.845;

pub fn system(
    mut reader: EventReader<api::SpotEvent>,
    mut events: ResMut<ReshockEvents>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for SpotEvent { sound, .. } in reader.iter() {
        match spot_event::SpotSound::from_i32(*sound) {
            Some(spot_event::SpotSound::ServBot) => {
                audio.play(asset_server.load(SERV_BOT_SPOT_SOUND));
                // thread::sleep(Duration::from_secs_f32(SERV_BOT_SPOT_LENGTH));
            }
            _ => {}
        };
        events.state = TransitionState::Inactive;
    }
}
