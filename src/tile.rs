use crate::component::*;
use ab_glyph::ScaleFont;
use bevy::prelude::*;
use itertools::Itertools;
use std::collections::HashMap;

const FONT_SIZE: f32 = 30.0;
const BOUNDING_GLYPH: char = '@';

#[derive(Bundle)]
pub struct Tile {
    position: Position,

    #[bundle]
    text: Text2dBundle,
}

impl Tile {
    pub fn new(position: Position, font: Handle<Font>) -> Self {
        Self {
            position,
            text: Text2dBundle {
                text: Text::with_section(
                    " ".to_string(),
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

#[derive(Default)]
pub struct TileDimensions(Option<Size>);

pub struct ReshockFont(pub Handle<Font>);

pub fn adapt_glyph_dimensions(
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

pub fn render(
    renderables: Query<(&Position, &Renderable, &Ordering)>,
    mut tiles: Query<(&Position, &mut Text)>,
) {
    let mut tiles_map: HashMap<_, _> = tiles.iter_mut().collect();
    renderables
        .iter()
        .map(|(position, renderable, ordering)| (position, (renderable, ordering)))
        .into_grouping_map()
        .fold_first(|current, _, next| -> (&Renderable, &Ordering) {
            let (_, l_ordering) = current;
            let (_, r_ordering) = next;
            if r_ordering.0 < l_ordering.0 {
                next
            } else {
                current
            }
        })
        .into_iter()
        .for_each(|(position, (renderable, _))| {
            if let Some(text) = tiles_map.get_mut(position) {
                if let Some(mut section) = text.sections.get_mut(0) {
                    section.value = renderable.char.to_string();
                    section.style.color = renderable.color;
                }
            }
        });
}

pub fn position(
    mut query: Query<(&mut Transform, &Position), With<Text>>,
    dimensions: Res<TileDimensions>,
) {
    if let Some(Size { width, height }) = dimensions.0 {
        for (mut transform, Position(pos)) in query.iter_mut() {
            transform.translation.x = pos.x as f32 * width;
            transform.translation.y = pos.y as f32 * height;
            transform.translation.z = 1.0;
        }
    }
}
