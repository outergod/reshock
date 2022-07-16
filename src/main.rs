use bevy::prelude::*;
use bevy_pancam::{PanCam, PanCamPlugin};
use system::input;
use tile::{ReshockFont, TileDimensions};

mod component;
mod system {
    pub mod input;
}
mod bundle;
mod room;
mod tile;

const FONT_PATH: &'static str = "fonts/FiraCode-Regular.otf";
const START_LAB_PATH: &'static str = "rooms/starting-lab.room";

pub struct Room(pub Handle<room::Room>);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(PanCam::default());

    let asset = asset_server.load(FONT_PATH);

    let font = ReshockFont(asset);
    commands.insert_resource(font);

    // let room = Room {
    //     handle: asset_server.load(START_LAB_PATH),
    //     loaded: false,
    // };

    let room = Room(asset_server.load(START_LAB_PATH));
    commands.insert_resource(room);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PanCamPlugin::default())
        .insert_resource(ClearColor(Color::BLACK))
        .init_resource::<TileDimensions>()
        .add_asset::<room::Room>()
        .init_asset_loader::<room::RoomLoader>()
        .add_startup_system(setup)
        .add_system(tile::render)
        .add_system(tile::adapt_glyph_dimensions)
        .add_system(tile::position)
        .add_system(input::system)
        .add_system(room::loaded)
        .run();
}
