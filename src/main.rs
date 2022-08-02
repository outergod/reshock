use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioPlugin};
use bevy_pancam::{PanCam, PanCamPlugin};
use bevy_tweening::component_animator_system;
use bevy_tweening::TweeningPlugin;
use component::Renderable;
use plugin::door::DoorPlugin;
use plugin::room::RoomPlugin;
use plugin::tile::TilePlugin;
use resource::ReshockFont;
use resource::Room;
use system::*;

mod asset;
mod component;
mod system {
    pub mod input;
    pub mod radial_lines;
    pub mod sight;
    pub mod wall;
}
mod plugin {
    pub mod door;
    pub mod room;
    pub mod tile;
}
mod bundle;
mod resource;

const FONT_PATH: &'static str = "fonts/Hack-Regular.ttf";
// const FONT_PATH: &'static str = "fonts/DejaVuSansMono.ttf";
const FONT_SIZE: f32 = 30.0;
const FONT_BOUNDING_GLYPH: char = '@';
const LEVEL01_PATH: &'static str = "rooms/level01.room";
const LEVEL01_MUSIC: &'static str = "sshock/music/chicajo/Medical.ogg";

// const START_LAB_PATH: &'static str = "rooms/starting-lab.room";
// const TEST_PATH: &'static str = "rooms/test.room";

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(PanCam::default());

    let asset = asset_server.load(FONT_PATH);

    let font = ReshockFont {
        handle: asset,
        size: FONT_SIZE,
        bounding_glyph: FONT_BOUNDING_GLYPH,
    };
    commands.insert_resource(font);

    let room = Room(asset_server.load(LEVEL01_PATH));
    // let room = Room(asset_server.load(TEST_PATH));
    commands.insert_resource(room);
}

fn start_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play_looped(asset_server.load(LEVEL01_MUSIC));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PanCamPlugin::default())
        .add_plugin(AudioPlugin)
        .add_plugin(TweeningPlugin)
        .add_plugin(TilePlugin)
        .add_plugin(RoomPlugin)
        .add_plugin(DoorPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(setup)
        .add_startup_system(start_background_audio)
        .add_startup_system(radial_lines::setup)
        .add_system(input::system)
        .add_system(wall::system)
        .add_system(sight::system)
        .add_system(bevy::window::close_on_esc)
        .add_system(component_animator_system::<Renderable>)
        .run();
}
