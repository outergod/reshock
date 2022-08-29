use api::action_request::*;
use api::reshock_client::ReshockClient;
use bevy::log;
use bevy::prelude::*;
use tokio::runtime::Runtime;
use tonic::transport::Channel;

use crate::plugin::RestartEvent;
use crate::resource::ReshockEvents;

pub fn system(
    keys: Res<Input<KeyCode>>,
    runtime: Res<Runtime>,
    mut client: ResMut<ReshockClient<Channel>>,
    mut events: ResMut<ReshockEvents>,
    mut writer: EventWriter<RestartEvent>,
) {
    if !events.queue.is_empty() {
        return;
    }

    let shift = keys.pressed(KeyCode::RShift) || keys.pressed(KeyCode::LShift);

    if shift && keys.just_pressed(KeyCode::R) {
        writer.send(RestartEvent);
        return;
    }

    if let Some(action) = if keys.just_pressed(KeyCode::A) {
        Some(Action::Dwim(DwimAction::Left as i32))
    } else if keys.just_pressed(KeyCode::E) {
        Some(Action::Dwim(DwimAction::Right as i32))
    } else if keys.just_pressed(KeyCode::Comma) {
        Some(Action::Dwim(DwimAction::Up as i32))
    } else if keys.just_pressed(KeyCode::Q) {
        Some(Action::Dwim(DwimAction::Down as i32))
    } else if keys.just_pressed(KeyCode::Apostrophe) {
        Some(Action::Dwim(DwimAction::UpLeft as i32))
    } else if keys.just_pressed(KeyCode::Period) {
        Some(Action::Dwim(DwimAction::UpRight as i32))
    } else if keys.just_pressed(KeyCode::Semicolon) {
        Some(Action::Dwim(DwimAction::DownLeft as i32))
    } else if keys.just_pressed(KeyCode::J) {
        Some(Action::Dwim(DwimAction::DownRight as i32))
    } else if keys.just_pressed(KeyCode::C) {
        Some(Action::Dwim(DwimAction::Close as i32))
    } else if keys.just_pressed(KeyCode::G) {
        Some(Action::God(GodModeAction {}))
    } else {
        None
    } {
        runtime.block_on(async move {
            match client
                .process_action(api::ActionRequest {
                    action: Some(action),
                })
                .await
            {
                Ok(response) => {
                    events.queue = response.into_inner().events.into();
                    log::debug!("Received event queue {}", *events);
                }
                Err(e) => {
                    log::warn!("Couldn't process action {}", e);
                }
            }
        });
    }
}
