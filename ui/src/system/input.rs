use crate::component::{Door, Obstacle, Player, Position};
use api::reshock_client::ReshockClient;
use bevy::log;
use bevy::{math::ivec2, prelude::*, utils::HashMap};
use tokio::runtime::Runtime;
use tonic::transport::Channel;

// (def sound-resources
//   {:player-hurt "00265.wav"

//    :door "00206.wav"
//    :airlock-door "00204.wav"
//    :blast-door "00268.wav"

//    :lead-pipe-hit "00222.wav"
//    :lead-pipe-miss "00225.wav"
//    :ion-rifle "00296.wav"
//    :gauss-rifle "00230.wav"
//    :dart "00287.wav"
//    :pistol "00240.wav"
//    :flechette "00239.wav"
//    :magnum-2100 "00241.wav"
//    :mark-3 "00218.wav"
//    :skorpion "00266.wav"
//    :plasma "00298.wav"
//    :magpulse "00246.wav"

//    :bot-die "00211.wav"
//    :serv-bot-spot "00275.wav"
//    :vmail "00293.wav"
//    :radiation "00203.wav"

//    :appendage-attack "00256.wav"
//    :hopper-attack "00213.wav"

//    :unknown-assault-rifle-2 "00210.wav"
//    :unknown-assault-rifle-1 "00292.wav"})

pub fn system(
    keys: Res<Input<KeyCode>>,
    // mut player: Query<&mut Position, With<Player>>,
    // mut obstacles: Query<(&Obstacle, &Position, Option<&mut Door>), Without<Player>>,
    runtime: Res<Runtime>,
    mut client: ResMut<ReshockClient<Channel>>,
) {
    // if player.is_empty() {
    //     return;
    // }

    // let mut player = player.single_mut();
    // let position = player.0;

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
                Ok(events) => {
                    log::debug!("events {:?}", events);
                }
                Err(e) => {
                    log::warn!("Couldn't process command {}", e);
                }
            }
        });
    }

    // if keys.just_pressed(KeyCode::A) {
    //     diff = Some(ivec2(-1, 0));
    // }
    // if keys.just_pressed(KeyCode::E) {
    //     diff = Some(ivec2(1, 0));
    // }
    // if keys.just_pressed(KeyCode::Comma) {
    //     diff = Some(ivec2(0, 1));
    // }
    // if keys.just_pressed(KeyCode::Q) {
    //     diff = Some(ivec2(0, -1));
    // }
    // if keys.just_pressed(KeyCode::Apostrophe) {
    //     diff = Some(ivec2(-1, 1));
    // }
    // if keys.just_pressed(KeyCode::Period) {
    //     diff = Some(ivec2(1, 1));
    // }
    // if keys.just_pressed(KeyCode::Semicolon) {
    //     diff = Some(ivec2(-1, -1));
    // }
    // if keys.just_pressed(KeyCode::J) {
    //     diff = Some(ivec2(1, -1));
    // }

    // if let Some(diff) = diff {
    //     let mut neighbors: HashMap<_, _> = obstacles
    //         .iter_mut()
    //         .filter_map(|(o, p, d)| {
    //             if !o.0 {
    //                 return None;
    //             }

    //             let diff = p.0 - position;
    //             if diff.x.abs() <= 1 && diff.y.abs() <= 1 {
    //                 Some((diff, d))
    //             } else {
    //                 None
    //             }
    //         })
    //         .collect();

    //     match neighbors.get_mut(&diff) {
    //         Some(Some(door)) => {
    //             door.toggle = true;
    //         }
    //         Some(None) => {
    //             // TODO In-game logging
    //             log::info!("Player can't move to {:?}", position + diff);
    //         }
    //         None => {
    //             player.0 += diff;
    //         }
    //     }
    // }
}
