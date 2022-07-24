use bevy::prelude::*;
use bevy_pancam::{PanCam, PanCamPlugin};
use resource::ReshockFont;
use resource::Room;
use resource::TileDimensions;
use system::*;

mod asset;
mod component;
mod system {
    pub mod input;
    pub mod radial_lines;
    pub mod room;
    pub mod sight;
    pub mod tile;
    pub mod wall;
}
mod bundle;
mod resource;

const FONT_PATH: &'static str = "fonts/FiraCode-Regular.otf";
const FONT_SIZE: f32 = 30.0;
const FONT_BOUNDING_GLYPH: char = '@';
const LEVEL01_PATH: &'static str = "rooms/level01.room";

// const START_LAB_PATH: &'static str = "rooms/starting-lab.room";
// const TEST_PATH: &'static str = "rooms/test.room";

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PanCamPlugin::default())
        .insert_resource(ClearColor(Color::BLACK))
        .init_resource::<TileDimensions>()
        .add_asset::<asset::Room>()
        .init_asset_loader::<asset::RoomLoader>()
        .add_startup_system(setup)
        .add_startup_system(radial_lines::setup)
        .add_system(tile::render)
        .add_system(tile::adapt_glyph_dimensions)
        .add_system(tile::position)
        .add_system(input::system)
        .add_system(room::loaded)
        .add_system(wall::system)
        .add_system(sight::system)
        .run();
}
