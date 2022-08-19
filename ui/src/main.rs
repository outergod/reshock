use anyhow::{Context, Result};
use api::reshock_client::ReshockClient;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioPlugin};
use bevy_pancam::{PanCam, PanCamPlugin};
use bevy_tweening::component_animator_system;
use bevy_tweening::TweeningPlugin;
use component::Renderable;
use tokio::runtime::Runtime;

use crate::config::Config;
use crate::plugin::*;

mod bundle;
mod component;
mod config;
mod plugin;
mod resource;
mod system;

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
        .add_plugin(ReshockEventsPlugin)
        .add_plugin(DoorPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(runtime)
        .insert_resource(client)
        .init_resource::<resource::Deltas>()
        .add_startup_system(setup)
        .add_startup_system(start_background_audio)
        .add_startup_system(system::client_setup)
        .add_system(system::input)
        .add_system(system::view.label("view"))
        .add_system(system::wall)
        .add_system(system::r#move)
        .add_system(bevy::window::close_on_esc)
        .add_system(component_animator_system::<Renderable>)
        .run();

    Ok(())
}
