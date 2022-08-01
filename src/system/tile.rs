use ab_glyph::ScaleFont;
use bevy::prelude::*;
use itertools::Itertools;

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
    player: Query<(&Sight, &Memory), With<Player>>,
    renderables: Query<(Entity, &Position, &Renderable, &Ordering)>,
    mut tiles: Query<(&Position, &mut Text)>,
) {
    let (seeing, memory, color) = match player.get_single() {
        Ok((Sight { seeing, .. }, Memory { entities, color })) => (seeing, entities, color),
        Err(_) => return,
    };

    let view = renderables
        .iter()
        .filter_map(|(entity, position, renderable, ordering)| {
            if seeing.contains(&entity) {
                Some((position, (renderable, ordering.0)))
            } else {
                None
            }
        })
        .into_grouping_map()
        .fold_first(|current, _, next| -> (&Renderable, u8) {
            let (_, l_ordering) = current;
            let (_, r_ordering) = next;
            if r_ordering < l_ordering {
                next
            } else {
                current
            }
        });

    let memory = memory
        .iter()
        .map(|(_, components)| {
            (
                &components.position,
                (&components.renderable, components.ordering.0),
            )
        })
        .into_grouping_map()
        .fold_first(|current, _, next| -> (&Renderable, u8) {
            let (_, l_ordering) = current;
            let (_, r_ordering) = next;
            if r_ordering < l_ordering {
                next
            } else {
                current
            }
        });

    for (position, mut text) in tiles.iter_mut() {
        if let Some(mut section) = text.sections.get_mut(0) {
            if let Some((renderable, _)) = view.get(position) {
                section.value = renderable.char.to_string();
                section.style.color = renderable.color;
            } else if let Some((renderable, _)) = memory.get(position) {
                section.value = renderable.char.to_string();
                section.style.color = color.clone();
            } else {
                section.value = " ".to_string();
            }
        }
    }
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
