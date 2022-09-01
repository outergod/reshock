// use std::time::Duration;

use bevy::prelude::*;
// use bevy_tweening::{lens::TransformPositionLens, Animator, EaseMethod, Tween, TweeningType};

use crate::component::*;
use crate::resource::*;

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
    entity: Query<&Position, With<Focus>>,
    mut camera: Query<&mut Transform, With<Camera>>,
    dimensions: Res<TileDimensions>,
) {
    let IVec2 { x, y } = match entity.get_single() {
        Ok(Position(it)) => *it,
        Err(_) => return,
    };

    let size = match dimensions.0 {
        Some(it) => it,
        None => return,
    };

    for mut transform in camera.iter_mut() {
        transform.translation.x = (x as f32 + 0.5) * size.width;
        transform.translation.y = (y as f32 - 0.5) * size.height;
    }
}

// fn follow(
//     mut reader: EventReader<api::MoveEvent>,
//     camera: Query<(Entity, &Transform), With<Camera>>,
//     dimensions: Res<TileDimensions>,
//     mut commands: Commands,
// ) {
//     let (camera, transform) = match camera.get_single() {
//         Ok(it) => it,
//         Err(_) => return,
//     };

//     let size = match dimensions.0 {
//         Some(it) => it,
//         None => return,
//     };

//     for api::MoveEvent { entity: _, x, y } in reader.iter() {
//         let lens = TransformPositionLens {
//             start: transform.translation,
//             end: (
//                 (*x as f32 + 0.5) * size.width,
//                 (*y as f32 - 0.5) * size.height,
//                 transform.translation.z,
//             )
//                 .into(),
//         };

//         let tween = Tween::new(
//             EaseMethod::Linear,
//             TweeningType::Once,
//             Duration::from_secs_f32(0.1),
//             lens,
//         );

//         commands.entity(camera).insert(Animator::new(tween));
//     }
// }
