use ab_glyph::ScaleFont;
use bevy::prelude::*;

const FONT_PATH: &'static str = "fonts/FiraCode-Regular.otf";
const FONT_SIZE: f32 = 60.0;
const BOUNDING_GLYPH: char = '@';

struct Player;

enum Renderable {
    Human,
    Floor,
}

impl Renderable {
    fn tile(&self) -> char {
        match self {
            Renderable::Human => '@',
            Renderable::Floor => '·',
        }
    }

    fn color(&self) -> Color {
        match self {
            Renderable::Human => Color::WHITE,
            Renderable::Floor => Color::GRAY,
        }
    }
}

struct Position(IVec2);

#[derive(Bundle)]
struct Object {
    position: Position,
    renderable: Renderable,

    #[bundle]
    text: Text2dBundle,
}

impl Object {
    fn new(position: Position, renderable: Renderable, font: Handle<Font>) -> Self {
        let tile = renderable.tile().to_string();

        Object {
            position,
            renderable,
            text: Text2dBundle {
                text: Text::with_section(
                    tile,
                    TextStyle {
                        font,
                        font_size: FONT_SIZE,
                        color: Color::WHITE,
                    },
                    TextAlignment::default(),
                ),
                ..Default::default()
            },
        }
    }
}

// struct Tile(Text2dBundle);

struct ReshockFont(Handle<Font>);

#[derive(Default)]
struct TileDimensions(Option<Size>);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let asset = asset_server.load(FONT_PATH);

    let font = ReshockFont(asset.clone_weak());
    commands.insert_resource(font);
    commands
        .spawn_bundle(Object::new(
            Position((0, 0).into()),
            Renderable::Human,
            asset.clone(),
        ))
        .insert(Player);

    for x in -10..10 {
        for y in -10..10 {
            commands.spawn_bundle(Object::new(
                Position((x, y).into()),
                Renderable::Floor,
                asset.clone(),
            ));
        }
    }
}

fn adapt_glyph_dimensions(
    mut event_asset: EventReader<AssetEvent<Font>>,
    mut assets: ResMut<Assets<Font>>,
    font_resource: Res<ReshockFont>,
    mut dimensions: ResMut<TileDimensions>,
) {
    for event in event_asset.iter() {
        match event {
            AssetEvent::Created { handle } | AssetEvent::Modified { handle } => {
                if *handle == font_resource.0 {
                    let font = &assets.get_mut(handle).unwrap().font;
                    let scaled = ab_glyph::Font::as_scaled(font, FONT_SIZE);
                    let glyph = ab_glyph::Font::glyph_id(&font, BOUNDING_GLYPH);

                    dimensions.0 = Some(Size {
                        width: scaled.h_advance(glyph),
                        height: scaled.height(),
                    });
                }
            }
            AssetEvent::Removed { handle: _ } => {}
        }
    }
}

fn render_tile(mut query: Query<(&mut Text, &Renderable)>) {
    for (mut text, renderable) in query.iter_mut() {
        if let Some(mut section) = text.sections.get_mut(0) {
            section.value = renderable.tile().to_string();
            section.style.color = renderable.color();
        }
    }
}

fn position(mut query: Query<(&mut Transform, &Position)>, dimensions: Res<TileDimensions>) {
    if let Some(Size { width, height }) = dimensions.0 {
        for (mut transform, Position(pos)) in query.iter_mut() {
            transform.translation.x = pos.x as f32 * width;
            transform.translation.y = pos.y as f32 * height;
        }
    }
}

fn input(keys: Res<Input<KeyCode>>, mut query: Query<&mut Position, With<Player>>) {
    if let Ok(mut position) = query.single_mut() {
        if keys.just_pressed(KeyCode::A) {
            position.0.x -= 1;
        }
        if keys.just_pressed(KeyCode::E) {
            position.0.x += 1;
        }
        if keys.just_pressed(KeyCode::Comma) {
            position.0.y += 1;
        }
        if keys.just_pressed(KeyCode::O) {
            position.0.y -= 1;
        }
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK))
        .init_resource::<TileDimensions>()
        .add_startup_system(setup.system())
        .add_system(render_tile.system())
        .add_system(adapt_glyph_dimensions.system())
        .add_system(position.system())
        .add_system(input.system())
        .run();
}
