use bevy::prelude::*;

use crate::{component::*, resource::*};

pub struct CameraPlugin;

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(follow.label("camera"));
    }
}

fn follow(
    player: Query<&Position, With<Player>>,
    mut camera: Query<&mut Transform, With<Camera>>,
    dimensions: Res<TileDimensions>,
) {
    let (x, y) = match player.get_single() {
        Ok(Position(IVec2 { x, y })) => (*x as f32, *y as f32),
        Err(_) => return,
    };

    let size = match dimensions.0 {
        Some(it) => it,
        None => return,
    };

    for mut transform in camera.iter_mut() {
        transform.translation.x = (x + 0.5) * size.width;
        transform.translation.y = (y - 0.5) * size.height;
    }
}
