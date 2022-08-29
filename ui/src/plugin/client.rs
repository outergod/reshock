use api::reshock_client::ReshockClient;
use api::StateDumpResponse;
use bevy::log;
use bevy::prelude::*;
use tokio::runtime::Runtime;
use tonic::transport::Channel;

use crate::bundle::*;
use crate::component::*;
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
    mut commands: Commands,
    mut log_res: ResMut<Log>,
    mut state: ResMut<GameState>,
    font: Res<ReshockFont>,
    mut writer: EventWriter<api::ViewUpdateEvent>,
) {
    let StateDumpResponse {
        player,
        dimensions,
        view,
        log,
    } = match state.0.to_owned() {
        Some(it) => it,
        None => return,
    };

    writer.send(api::ViewUpdateEvent { player, view });

    if let Some(api::Dimensions { x, y }) = dimensions {
        for y in 0..=y {
            for x in 0..=x {
                commands.spawn_bundle(Tile::new(Position((x, y).into()), &font));
            }
        }
    } else {
        log::error!("Received empty dimensions, fatal");
    }

    if let Some(api::Log { entries }) = log {
        log_res.0 = entries;
    } else {
        log::warn!("Received empty log, suspicious");
    }

    *state = GameState(None);
}
