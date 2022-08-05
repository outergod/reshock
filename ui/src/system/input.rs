use api::reshock_client::ReshockClient;
use bevy::log;
use bevy::prelude::*;
use tokio::runtime::Runtime;
use tonic::transport::Channel;

use crate::resource::ReshockEvents;

pub fn system(
    keys: Res<Input<KeyCode>>,
    runtime: Res<Runtime>,
    mut client: ResMut<ReshockClient<Channel>>,
    mut events: ResMut<ReshockEvents>,
) {
    if !events.0.is_empty() {
        return;
    }

    if let Some(command) = if keys.just_pressed(KeyCode::A) {
        Some(api::command_request::Command::Left)
    } else if keys.just_pressed(KeyCode::E) {
        Some(api::command_request::Command::Right)
    } else if keys.just_pressed(KeyCode::Comma) {
        Some(api::command_request::Command::Up)
    } else if keys.just_pressed(KeyCode::Q) {
        Some(api::command_request::Command::Down)
    } else if keys.just_pressed(KeyCode::Apostrophe) {
        Some(api::command_request::Command::UpLeft)
    } else if keys.just_pressed(KeyCode::Period) {
        Some(api::command_request::Command::UpRight)
    } else if keys.just_pressed(KeyCode::Semicolon) {
        Some(api::command_request::Command::DownLeft)
    } else if keys.just_pressed(KeyCode::J) {
        Some(api::command_request::Command::DownRight)
    } else {
        None
    } {
        runtime.block_on(async move {
            match client
                .process_command(api::CommandRequest {
                    command: command as i32,
                })
                .await
            {
                Ok(response) => {
                    events.0 = response.into_inner().events.into();
                }
                Err(e) => {
                    log::warn!("Couldn't process command {}", e);
                }
            }
        });
    }
}
