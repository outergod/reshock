use api::reshock_client::ReshockClient;
use api::*;
use bevy::log;
use bevy::prelude::*;
use tokio::runtime::Runtime;
use tonic::transport::Channel;

use crate::bundle::*;
use crate::component::*;
use crate::resource::ReshockFont;

pub fn setup(
    mut commands: Commands,
    runtime: Res<Runtime>,
    mut client: ResMut<ReshockClient<Channel>>,
    font: Res<ReshockFont>,
    mut writer: EventWriter<api::ViewUpdateEvent>,
) {
    runtime.block_on(async move {
        match client.dump_state(api::Empty {}).await {
            Ok(response) => {
                let StateDumpResponse { player, view } = response.into_inner();
                writer.send(api::ViewUpdateEvent { player, view });

                for y in 0..=100 {
                    for x in 0..=100 {
                        commands.spawn_bundle(Tile::new(Position((x, y).into()), &font));
                    }
                }
            }
            Err(e) => {
                log::error!("Could not load Reshock server state, fatal: {}", e);
            }
        }
    });
}
