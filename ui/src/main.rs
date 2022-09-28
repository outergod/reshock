#![feature(iter_intersperse)]

use anyhow::{Context, Result};
use api::reshock_client::ReshockClient;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_kira_audio::{Audio, AudioPlugin};
use bevy_prototype_lyon::prelude::*;
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
        .add_plugin(AudioPlugin)
        .add_plugin(TweeningPlugin)
        .add_plugin(TilePlugin)
        .add_plugin(ReshockEventsPlugin)
        .add_plugin(DoorPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(EguiPlugin)
        .add_plugin(UiPlugin)
        .add_plugin(ClientPlugin)
        .add_plugin(ShapePlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(runtime)
        .insert_resource(client)
        .init_resource::<resource::Deltas>()
        .init_resource::<resource::Log>()
        .add_startup_system(start_background_audio)
        .add_system(system::input)
        .add_system(system::view.label("view"))
        .add_system(system::wall)
        .add_system(system::spot)
        .add_system(system::log)
        .add_system(system::hit)
        .add_system(system::effect)
        .add_system(system::death)
        .add_system(system::shoot)
        .add_system(system::projectile)
        .add_system(bevy::window::close_on_esc)
        .add_system(component_animator_system::<Renderable>)
        .run();

    Ok(())
}
