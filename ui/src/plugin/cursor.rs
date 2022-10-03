use bevy::math::vec2;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;

use crate::resource::*;

pub struct CursorPlugin;

fn setup(mut commands: Commands) {
    commands.init_resource::<Cursor>();
}

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(system);
    }
}

fn system(
    cameras: Query<(&Camera, &Transform)>,
    mut cursor: ResMut<Cursor>,
    windows: Res<Windows>,
    dimensions: Res<TileDimensions>,
) {
    let (camera, transform) = cameras.single();

    let window = if let RenderTarget::Window(id) = camera.target {
        match windows.get(id) {
            Some(it) => it,
            None => {
                cursor.0 = None;
                return;
            }
        }
    } else {
        match windows.get_primary() {
            Some(it) => it,
            None => {
                cursor.0 = None;
                return;
            }
        }
    };

    let Size { width, height } = match dimensions.0 {
        Some(it) => it,
        None => return,
    };

    if let Some(pos) = window.cursor_position() {
        let size = vec2(window.width() as f32, window.height() as f32);
        let ndc = (pos / size) * 2.0 - Vec2::ONE;
        let ndc_to_world = transform.compute_matrix() * camera.projection_matrix().inverse();
        let pos = (ndc_to_world.project_point3(ndc.extend(-1.0)).truncate()) / vec2(width, height);
        cursor.0 = Some(pos.round().as_ivec2());
    } else {
        cursor.0 = None;
    }
}
