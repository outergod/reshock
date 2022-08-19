use ab_glyph::ScaleFont;
use bevy::prelude::*;
use itertools::Itertools;

use crate::resource::ReshockFont;
use crate::{component::*, resource::TileDimensions};

const FONT_PATH: &'static str = "fonts/Hack-Regular.ttf";
// const FONT_PATH: &'static str = "fonts/DejaVuSansMono.ttf";
// const FONT_PATH: &'static str = "fonts/FiraCode-Regular.otf";
const FONT_SIZE: f32 = 30.0;
const FONT_BOUNDING_GLYPH: char = '@';

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(render.after("view"))
            .add_system(adapt_glyph_dimensions)
            .add_system(position)
            .init_resource::<TileDimensions>()
            .init_resource::<ReshockFont>();
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let asset = asset_server.load(FONT_PATH);

    let font = ReshockFont {
        handle: asset,
        size: FONT_SIZE,
        bounding_glyph: FONT_BOUNDING_GLYPH,
    };
    commands.insert_resource(font.clone());
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
                if *handle == font_resource.handle {
                    let font = &assets.get_mut(handle).unwrap().font;
                    let scaled = ab_glyph::Font::as_scaled(font, font_resource.size);
                    let glyph = ab_glyph::Font::glyph_id(&font, font_resource.bounding_glyph);

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

fn render(
    renderables: Query<(&Position, &Renderable, &Ordering)>,
    mut tiles: Query<(&Position, &mut Text, &mut Transform)>,
    font_resource: Res<ReshockFont>,
) {
    let view = renderables
        .iter()
        .map(|(position, renderable, ordering)| (position, (renderable, *ordering as u8)))
        .into_grouping_map()
        .fold_first(|current, _, next| {
            let (_, l_ordering) = current;
            let (_, r_ordering) = next;
            if r_ordering > l_ordering {
                next
            } else {
                current
            }
        });

    for (position, mut text, mut transform) in tiles.iter_mut() {
        if let Some(mut section) = text.sections.get_mut(0) {
            section.style.font_size = font_resource.size;
            section.style.font = font_resource.handle.clone_weak();

            if let Some((renderable, ordering)) = view.get(position) {
                section.value = renderable.char.to_string();
                section.style.color = renderable.color;
                transform.translation.z = *ordering as f32;
            } else {
                section.value = " ".to_string();
                transform.translation.z = 1.0;
            }
        }
    }
}

fn position(
    mut tiles: Query<(&mut Transform, &Position), With<Text>>,
    dimensions: Res<TileDimensions>,
) {
    if let Some(Size { width, height }) = dimensions.0 {
        for (mut transform, Position(pos)) in tiles.iter_mut() {
            transform.translation.x = pos.x as f32 * width;
            transform.translation.y = pos.y as f32 * height;
        }
    }
}
