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

    if let Some(action) = if keys.just_pressed(KeyCode::A) {
        Some(api::action_request::Action::Left)
    } else if keys.just_pressed(KeyCode::E) {
        Some(api::action_request::Action::Right)
    } else if keys.just_pressed(KeyCode::Comma) {
        Some(api::action_request::Action::Up)
    } else if keys.just_pressed(KeyCode::Q) {
        Some(api::action_request::Action::Down)
    } else if keys.just_pressed(KeyCode::Apostrophe) {
        Some(api::action_request::Action::UpLeft)
    } else if keys.just_pressed(KeyCode::Period) {
        Some(api::action_request::Action::UpRight)
    } else if keys.just_pressed(KeyCode::Semicolon) {
        Some(api::action_request::Action::DownLeft)
    } else if keys.just_pressed(KeyCode::J) {
        Some(api::action_request::Action::DownRight)
    } else {
        None
    } {
        runtime.block_on(async move {
            match client
                .process_action(api::ActionRequest {
                    action: action as i32,
                })
                .await
            {
                Ok(response) => {
                    events.0 = response.into_inner().events.into();
                }
                Err(e) => {
                    log::warn!("Couldn't process action {}", e);
                }
            }
        });
    }
}
