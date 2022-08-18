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
                let StateDumpResponse {
                    player,
                    dimensions,
                    view,
                } = response.into_inner();
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
            }
            Err(e) => {
                log::error!("Could not load Reshock server state, fatal: {}", e);
            }
        }
    });
}
