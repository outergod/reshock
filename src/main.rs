use ab_glyph::ScaleFont;
use bevy::prelude::*;
use bevy_pancam::{PanCam, PanCamPlugin};
// use bevy_prototype_lyon::entity::ShapeBundle;
// use bevy_prototype_lyon::prelude::*;
// use bevy_prototype_lyon::shapes::RectangleOrigin;
// use lyon_tessellation::path::Path;

const FONT_PATH: &'static str = "fonts/FiraCode-Regular.otf";
const FONT_SIZE: f32 = 30.0;
const BOUNDING_GLYPH: char = '@';

#[derive(Component)]
struct Player;

#[derive(Component)]
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

    fn ordering(&self) -> u8 {
        match self {
            Renderable::Human => 2,
            Renderable::Floor => 0,
        }
    }
}

#[derive(Component)]
struct Position(IVec2);

impl Default for Position {
    fn default() -> Self {
        Self((0, 0).into())
    }
}

#[derive(Bundle)]
struct Object {
    position: Position,
    renderable: Renderable,

    #[bundle]
    text: Text2dBundle,
    // background: Entity,
}

impl Object {
    fn new(
        position: Position,
        renderable: Renderable,
        font: Handle<Font>,
        // background: Entity,
    ) -> Self {
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
            // background,
        }
    }
}

// #[derive(Bundle)]
// struct BlackSquare {
//     #[bundle]
//     shape: ShapeBundle,
// }

// impl BlackSquare {
//     fn new() -> Self {
//         let shape = shapes::Rectangle {
//             // TODO adaptive
//             width: 15.0,
//             height: 30.0,
//             origin: RectangleOrigin::BottomRight,
//         };

//         BlackSquare {
//             shape: GeometryBuilder::build_as(
//                 &shape,
//                 // ShapeColors::new(Color::BLACK),
//                 DrawMode::Fill(FillOptions::default()),
//                 Transform::default(),
//             ),
//         }
//     }
// }

// struct Tile(Text2dBundle);

struct ReshockFont(Handle<Font>);

#[derive(Default)]
struct TileDimensions(Option<Size>);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(PanCam::default());

    let asset = asset_server.load(FONT_PATH);

    let font = ReshockFont(asset.clone_weak());
    commands.insert_resource(font);

    // let square = commands.spawn_bundle(BlackSquare::new()).id();
    commands
        .spawn_bundle(Object::new(
            Position::default(),
            Renderable::Human,
            asset.clone(),
            // square,
        ))
        .insert(Player);

    for x in -10..10 {
        for y in -10..10 {
            // let square = commands.spawn_bundle(BlackSquare::new()).id();
            commands.spawn_bundle(Object::new(
                Position((x, y).into()),
                Renderable::Floor,
                asset.clone(),
                // square,
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

fn position(
    mut query: Query<(&mut Transform, &Position, &Renderable)>,
    dimensions: Res<TileDimensions>,
) {
    if let Some(Size { width, height }) = dimensions.0 {
        for (mut transform, Position(pos), renderable) in query.iter_mut() {
            transform.translation.x = pos.x as f32 * width;
            transform.translation.y = pos.y as f32 * height;
            transform.translation.z = 2.0 * renderable.ordering() as f32;
        }
    }
}

// fn adapt_backgrounds(
//     query_parents: Query<(&Position, &Renderable, &Entity)>,
//     mut query: Query<(&mut Transform, &mut Path)>,
//     dimensions: Res<TileDimensions>,
// ) {
//     if let Some(Size { width, height }) = dimensions.0 {
//         for (Position(pos), renderable, background) in query_parents.iter() {
//             if let Ok((mut transform, mut path)) = query.get_mut(*background) {
//                 transform.translation.x = pos.x as f32 * width;
//                 transform.translation.y = pos.y as f32 * height;
//                 transform.translation.z = 2.0 * renderable.ordering() as f32 - 1.0;

//                 // let shape = shapes::Rectangle {
//                 //     width,
//                 //     height,
//                 //     origin: RectangleOrigin::TopLeft,
//                 // };

//                 // *path = GeometryBuilder::build_as(
//                 //     &shape,
//                 //     ShapeColors::new(Color::BLACK),
//                 //     DrawMode::Fill(FillOptions::default()),
//                 //     Transform::default(),
//                 // );
//             }
//         }
//     }
// }

fn input(keys: Res<Input<KeyCode>>, mut query: Query<&mut Position, With<Player>>) {
    let mut position = query.single_mut();

    if keys.just_pressed(KeyCode::A) {
        position.0.x -= 1;
    }
    if keys.just_pressed(KeyCode::E) {
        position.0.x += 1;
    }
    if keys.just_pressed(KeyCode::Comma) {
        position.0.y += 1;
    }
    if keys.just_pressed(KeyCode::Q) {
        position.0.y -= 1;
    }
    if keys.just_pressed(KeyCode::Apostrophe) {
        position.0.x -= 1;
        position.0.y += 1;
    }
    if keys.just_pressed(KeyCode::Period) {
        position.0.x += 1;
        position.0.y += 1;
    }
    if keys.just_pressed(KeyCode::Semicolon) {
        position.0.x -= 1;
        position.0.y -= 1;
    }
    if keys.just_pressed(KeyCode::J) {
        position.0.x += 1;
        position.0.y -= 1;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PanCamPlugin::default())
        // .add_plugin(ShapePlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .init_resource::<TileDimensions>()
        .add_startup_system(setup)
        .add_system(render_tile)
        .add_system(adapt_glyph_dimensions)
        .add_system(position)
        // .add_system(adapt_backgrounds.system())
        .add_system(input)
        .run();
}
