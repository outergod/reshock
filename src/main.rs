use bevy::prelude::*;
use bevy_pancam::{PanCam, PanCamPlugin};
use component::*;
use system::input;
use tile::{ReshockFont, Tile, TileDimensions};

mod component;
mod system {
    pub mod input;
}
mod tile;

const FONT_PATH: &'static str = "fonts/FiraCode-Regular.otf";

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(PanCam::default());

    let asset = asset_server.load(FONT_PATH);

    let font = ReshockFont(asset.clone_weak());
    commands.insert_resource(font);

    commands
        .spawn()
        .insert(Player)
        .insert(Renderable::Human)
        .insert(Position((0, 0).into()))
        .insert(Ordering(u8::MIN));

    for x in -10..10 {
        for y in -10..10 {
            commands.spawn_bundle(Tile::new(Position((x, y).into()), asset.clone()));
            commands
                .spawn()
                .insert(Floor)
                .insert(Renderable::Floor)
                .insert(Position((x, y).into()))
                .insert(Ordering(u8::MAX));
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PanCamPlugin::default())
        .insert_resource(ClearColor(Color::BLACK))
        .init_resource::<TileDimensions>()
        .add_startup_system(setup)
        .add_system(tile::render)
        .add_system(tile::adapt_glyph_dimensions)
        .add_system(tile::position)
        .add_system(input::system)
        .run();
}
