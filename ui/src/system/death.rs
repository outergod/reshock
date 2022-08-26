use api::death_event::DeathSound;
use bevy::prelude::*;
use bevy_kira_audio::Audio;

use crate::{
    component::*,
    resource::{ReshockEvents, TransitionState},
};

const SERV_BOT_SOUND: &'static str = "sshock/sounds/00211.wav";

pub fn system(
    mut reader: EventReader<api::DeathEvent>,
    mut events: ResMut<ReshockEvents>,
    mut renderables: Query<(&ReshockEntity, &mut Renderable)>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for api::DeathEvent { actor, sound } in reader.iter() {
        if let Some(sound) = match DeathSound::from_i32(*sound) {
            Some(DeathSound::ServBot) => Some(SERV_BOT_SOUND),
            _ => None,
        } {
            audio.play(asset_server.load(sound));
        }

        let mut renderable = renderables
            .iter_mut()
            .find_map(|(entity, r)| (&entity.0 == actor).then_some(r))
            .unwrap();

        renderable.char = '%';
        renderable.color = Color::WHITE;

        events.state = TransitionState::Inactive;
    }
}
