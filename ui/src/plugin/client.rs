use api::reshock_client::ReshockClient;
use api::StateDumpResponse;
use bevy::log;
use bevy::prelude::*;
use tokio::runtime::Runtime;
use tonic::transport::Channel;

use crate::resource::*;

pub struct RestartEvent;

#[derive(Default)]
pub struct GameState(Option<StateDumpResponse>);

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RestartEvent>()
            .insert_resource(GameState(None))
            .add_startup_system(setup)
            .add_system(restart)
            .add_system(load);
    }
}

pub fn setup(
    mut client: ResMut<ReshockClient<Channel>>,
    runtime: Res<Runtime>,
    mut state: ResMut<GameState>,
) {
    runtime.block_on(async move {
        match client.dump_state(api::Empty {}).await {
            Ok(response) => {
                *state = GameState(Some(response.into_inner()));
            }
            Err(e) => {
                log::error!("Could not load Reshock server state, fatal: {}", e);
            }
        }
    });
}

pub fn restart(
    reader: EventReader<RestartEvent>,
    mut client: ResMut<ReshockClient<Channel>>,
    runtime: Res<Runtime>,
    mut state: ResMut<GameState>,
) {
    if reader.is_empty() {
        return;
    }

    runtime.block_on(async move {
        match client.restart(api::Empty {}).await {
            Ok(response) => {
                *state = GameState(Some(response.into_inner()));
            }
            Err(e) => {
                log::error!("Could not restart Reshock server, fatal: {}", e);
            }
        }
    });
}

pub fn load(
    mut log_res: ResMut<Log>,
    mut game_state: ResMut<GameState>,
    mut writer: EventWriter<api::StateUpdateEvent>,
    mut events: ResMut<ReshockEvents>,
) {
    let StateDumpResponse { player, state, log } = match game_state.0.to_owned() {
        Some(it) => it,
        None => return,
    };

    events.transitions += 1;
    writer.send(api::StateUpdateEvent { player, state });

    if let Some(api::Log { entries }) = log {
        log_res.0 = entries;
    } else {
        log::warn!("Received empty log, suspicious");
    }

    *game_state = GameState(None);
}
