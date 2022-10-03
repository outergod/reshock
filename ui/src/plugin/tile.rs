use ab_glyph::ScaleFont;
use bevy::prelude::*;
use itertools::Itertools;

use crate::resource::ReshockFont;
use crate::{component::*, resource::TileDimensions};

// const FONT_PATH: &'static str = "fonts/Hack-Regular.ttf";
// const FONT_PATH: &'static str = "fonts/DejaVuSansMono.ttf";
const FONT_PATH: &'static str = "fonts/FiraCode-Regular.otf";
const SYMBOL_FONT_PATH: &'static str = "fonts/Symbola.ttf";
const FONT_SIZE: f32 = 30.0;
const FONT_BOUNDING_GLYPH: char = '@';

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(render.before("camera").after("view"))
            .add_system(adapt_glyph_dimensions)
            .add_system(position)
            .init_resource::<TileDimensions>()
            .init_resource::<ReshockFont>();
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_asset = asset_server.load(FONT_PATH);
    let symbol_font_asset = asset_server.load(SYMBOL_FONT_PATH);

    let font = ReshockFont {
        handle: font_asset,
        symbol_handle: symbol_font_asset,
        size: FONT_SIZE,
        bounding_glyph: FONT_BOUNDING_GLYPH,
    };
    commands.insert_resource(font.clone());
}

fn adapt_glyph_dimensions(
    mut event_asset: EventReader<AssetEvent<Font>>,
    assets: Res<Assets<Font>>,
    font_resource: Res<ReshockFont>,
    mut dimensions: ResMut<TileDimensions>,
) {
    for event in event_asset.iter() {
        match event {
            AssetEvent::Created { handle } | AssetEvent::Modified { handle } => {
                if *handle == font_resource.handle {
                    let font = &assets.get(handle).unwrap().font;
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
    renderables: Query<(&Position, &Renderable)>,
    mut tiles: Query<(&Position, &mut Text, &mut Transform), With<Tile>>,
    font_resource: Res<ReshockFont>,
) {
    let view = renderables
        .iter()
        .into_grouping_map()
        .fold_first(|current, _, next| {
            if next.ordering as u8 > current.ordering as u8 {
                next
            } else {
                current
            }
        });

    for (position, mut text, mut transform) in tiles.iter_mut() {
        if let Some(mut section) = text.sections.get_mut(0) {
            section.style.font_size = font_resource.size;

            if let Some(renderable) = view.get(position) {
                section.value = renderable.char.to_string();
                section.style.font = font_resource.handle_for(renderable.char);
                section.style.color = renderable.color;
                transform.translation.z = renderable.ordering as u8 as f32;
            } else {
                section.value = " ".to_string();
                transform.translation.z = 1.0;
            }
        }
    }
}

fn position(
    mut tiles: Query<(&mut Transform, &Position), With<Tile>>,
    dimensions: Res<TileDimensions>,
) {
    if let Some(Size { width, height }) = dimensions.0 {
        for (mut transform, Position(pos)) in tiles.iter_mut() {
            transform.translation.x = pos.x as f32 * width;
            transform.translation.y = pos.y as f32 * height;
        }
    }
}

#[cfg(test)]
mod test {
    use ab_glyph::{Font, FontArc, FontVec};

    #[test]
    fn test_glyph_ids() {
        // let font_data = include_bytes!("../../assets/fonts/FiraCode-Regular.otf").to_vec();
        let font_data = include_bytes!("../../assets/fonts/Symbola.ttf").to_vec();
        let font_vec = FontVec::try_from_vec(font_data).unwrap();
        let font_arc = FontArc::new(font_vec);
        // let scaled = Font::as_scaled(&font_arc, font_resource.size);
        let glyph = Font::glyph_id(&font_arc, '⏻');

        println!("{:?}", glyph);
    }
}
