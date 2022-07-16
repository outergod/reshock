use crate::component::{Player, Position};
use bevy::prelude::*;

pub fn system(keys: Res<Input<KeyCode>>, mut query: Query<&mut Position, With<Player>>) {
    if query.is_empty() {
        return;
    }

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
