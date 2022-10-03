use bevy::{math::vec2, prelude::*};
use bevy_prototype_lyon::prelude::*;

use crate::{bundle, component::*, resource::*};

pub struct MarkerPlugin;

fn setup(mut commands: Commands) {
    commands.spawn_bundle(bundle::Marker {
        position: Position(IVec2::ZERO),
        marker: Marker::Selection,
        visibility: Visibility { is_visible: false },
        transform: default(),
    });
}

impl Plugin for MarkerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(adapt)
            .add_system(select)
            .add_system(color)
            .add_system(activate);
    }
}

fn adapt(
    mut commands: Commands,
    marker: Query<Entity, With<Marker>>,
    dimensions: Res<TileDimensions>,
) {
    if dimensions.is_changed() && let Some(Size { width, height }) = dimensions.0  {
        let mut builder = PathBuilder::new();
        builder.move_to(vec2(0.0, 3.0 * height / 4.0));
        builder.line_to(vec2(0.0, height));
        builder.line_to(vec2(width / 4.0, height));
        builder.move_to(vec2(3.0 * width / 4.0, height));
        builder.line_to(vec2(width, height));
        builder.line_to(vec2(width, 3.0 * height / 4.0));
        builder.move_to(vec2(width, height / 4.0));
        builder.line_to(vec2(width, 0.0));
        builder.line_to(vec2(3.0 * width / 4.0, 0.0));
        builder.move_to(vec2(width / 4.0, 0.0));
        builder.line_to(Vec2::ZERO);
        builder.line_to(vec2(0.0, height / 4.0));
        let path = builder.build();
        let shape = GeometryBuilder::build_as(
            &path,
            DrawMode::Stroke(StrokeMode::new(
                Color::Rgba {
                    red: 0.716,
                    green: 0.733,
                    blue: 0.608,
                    alpha: 1.0,
                },
                2.5,
            )),
            Transform::from_xyz(0.0, 0.0, 2.0),
        );

        let marker = marker.single();
        commands.entity(marker).insert_bundle(shape);
    }
}

fn select(
    cursor: Res<Cursor>,
    dimensions: Res<TileDimensions>,
    mut marker: Query<(&mut Transform, &mut Visibility, &Marker)>,
) {
    let (mut transform, mut visibility) = match marker.single_mut() {
        (transform, visibility, Marker::Selection) => (transform, visibility),
        _ => return,
    };

    let dimensions = match dimensions.0 {
        Some(Size { width, height }) => vec2(width, height),
        None => return,
    };

    match cursor.0 {
        None => {
            visibility.is_visible = false;
        }
        Some(pos) => {
            visibility.is_visible = true;
            transform.translation = (pos.as_vec2() * dimensions - dimensions / 2.0, 2.0).into();
        }
    }
}

fn color(mut marker: Query<(&Marker, &mut DrawMode), Changed<Marker>>) {
    let (marker, mut mode) = match marker.get_single_mut() {
        Ok(it) => it,
        Err(_) => return,
    };

    match marker {
        Marker::Selection => {
            *mode = DrawMode::Stroke(StrokeMode::new(
                Color::Rgba {
                    red: 0.716,
                    green: 0.733,
                    blue: 0.608,
                    alpha: 1.0,
                },
                2.5,
            ));
        }
        Marker::Active(_) => {
            *mode = DrawMode::Stroke(StrokeMode::new(
                Color::Rgba {
                    red: 0.671,
                    green: 0.733,
                    blue: 0.29,
                    alpha: 1.0,
                },
                2.5,
            ));
        }
    }
}

fn activate(cursor: Res<Cursor>, buttons: Res<Input<MouseButton>>, mut marker: Query<&mut Marker>) {
    let mut marker = marker.single_mut();
    let pos = match cursor.0 {
        Some(it) => it,
        None => return,
    };

    if buttons.just_pressed(MouseButton::Left) {
        match marker.as_ref() {
            Marker::Selection => {
                *marker = Marker::Active(pos);
            }
            _ => {}
        }
    }

    if buttons.just_pressed(MouseButton::Right) {
        match marker.as_ref() {
            Marker::Active(_) => {
                *marker = Marker::Selection;
            }
            _ => {}
        }
    }
}
