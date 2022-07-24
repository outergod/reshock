use ab_glyph::ScaleFont;
use bevy::prelude::*;
use itertools::Itertools;
use std::collections::HashMap;

use crate::resource::ReshockFont;
use crate::{component::*, resource::TileDimensions};

pub fn adapt_glyph_dimensions(
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

pub fn render(
    player: Query<&Sight, With<Player>>,
    renderables: Query<(Entity, &Position, &Renderable, &Ordering)>,
    mut tiles: Query<(&Position, &mut Text)>,
) {
    let seeing = match player.get_single() {
        Ok(Sight { seeing, .. }) => seeing,
        Err(_) => return,
    };

    let mut tiles_map: HashMap<_, _> = tiles.iter_mut().collect();
    renderables
        .iter()
        .map(|(entity, position, renderable, ordering)| {
            (position, (renderable, ordering.0, seeing.contains(&entity)))
        })
        .into_grouping_map()
        .fold_first(|current, _, next| -> (&Renderable, u8, bool) {
            let (_, l_ordering, _) = current;
            let (_, r_ordering, visible) = next;
            if visible && r_ordering < l_ordering {
                next
            } else {
                current
            }
        })
        .into_iter()
        .for_each(|(position, (renderable, _, visible))| {
            if let Some(text) = tiles_map.get_mut(position) {
                if let Some(mut section) = text.sections.get_mut(0) {
                    if visible {
                        section.value = renderable.char.to_string();
                        section.style.color = renderable.color;
                    } else {
                        section.value = " ".to_string();
                    }
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
