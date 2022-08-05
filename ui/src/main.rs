use anyhow::{Context, Result};
use api::reshock_client::ReshockClient;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioPlugin};
use bevy_pancam::{PanCam, PanCamPlugin};
use bevy_tweening::component_animator_system;
use bevy_tweening::TweeningPlugin;
use component::Renderable;
use plugin::door::DoorPlugin;
use plugin::reshock_events::ReshockEventsPlugin;
use plugin::tile::TilePlugin;
use system::*;
use tokio::runtime::Runtime;

use crate::config::Config;

mod component;
mod config;
mod system {
    pub mod client;
    pub mod input;
    pub mod r#move;
    pub mod wall;
}
mod plugin {
    pub mod door;
    pub mod reshock_events;
    pub mod tile;
}
mod bundle;
mod resource;

const LEVEL01_MUSIC: &'static str = "sshock/music/chicajo/Medical.ogg";

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(PanCam::default());
}

fn start_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play_looped(asset_server.load(LEVEL01_MUSIC));
}

fn main() -> Result<()> {
    let runtime = Runtime::new()?;
    let config = Config::new().context("Could not load configuration")?;
    let client = runtime
        .block_on(ReshockClient::connect(config.reshock_url))
        .context("Couldn't connect to Reshock server")?;

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PanCamPlugin::default())
        .add_plugin(AudioPlugin)
        .add_plugin(TweeningPlugin)
        .add_plugin(TilePlugin)
        .add_plugin(DoorPlugin)
        .add_plugin(ReshockEventsPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(runtime)
        .insert_resource(client)
        .add_startup_system(setup)
        .add_startup_system(start_background_audio)
        .add_startup_system(client::setup)
        .add_system(input::system)
        .add_system(wall::system)
        .add_system(r#move::system)
        .add_system(bevy::window::close_on_esc)
        .add_system(component_animator_system::<Renderable>)
        .run();

    Ok(())
}
