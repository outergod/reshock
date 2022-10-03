use bevy::math::vec2;
use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;

use crate::component;
use crate::resource::ReshockFont;

#[derive(Bundle)]
pub struct Tile {
    position: component::Position,
    tile: component::Tile,

    #[bundle]
    text: Text2dBundle,
}

impl Tile {
    pub fn new(position: component::Position, font: &ReshockFont) -> Self {
        Self {
            position,
            tile: component::Tile,
            text: Text2dBundle {
                text: Text::from_section(
                    " ".to_string(),
                    TextStyle {
                        font: font.handle.clone_weak(),
                        font_size: font.size,
                        color: Color::WHITE,
                    },
                )
                .with_alignment(TextAlignment::CENTER),
                ..Default::default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct Effect {
    pub effect: component::Effect,
    pub position: component::Position,
    pub renderable: component::Renderable,
}

#[derive(Bundle, Default)]
pub struct Projectile {
    pub projectile: component::Projectile,
    pub velocity: component::Velocity,

    #[bundle]
    pub shape: ShapeBundle,
}

impl Projectile {
    pub fn new(start: Vec2, end: Vec2, speed: f32, size: f32) -> Self {
        let projectile = component::Projectile { start, end, size };
        let velocity = component::Velocity((end - start).normalize() * speed);

        let shape = shapes::Line(vec2(0.0, 0.0), (end - start).normalize() * size);
        let shape = GeometryBuilder::build_as(
            &shape,
            DrawMode::Stroke(StrokeMode::new(Color::GRAY, 2.0)),
            Transform::from_xyz(start.x, start.y, 1.0),
        );

        Self {
            projectile,
            velocity,
            shape,
        }
    }
}

#[derive(Bundle)]
pub struct Marker {
    pub position: component::Position,
    pub marker: component::Marker,
    pub visibility: Visibility,
    pub transform: Transform,
}
