use crate::component::{Obstacle, Player, Position};
use bevy::log;
use bevy::{math::ivec2, prelude::*, utils::HashMap};

pub fn system(
    keys: Res<Input<KeyCode>>,
    mut player: Query<&mut Position, With<Player>>,
    obstacles: Query<(&Obstacle, &Position), Without<Player>>,
) {
    if player.is_empty() {
        return;
    }

    let mut player = player.single_mut();
    let position = player.0;

    let mut diff = None;

    if keys.just_pressed(KeyCode::A) {
        diff = Some(ivec2(-1, 0));
    }
    if keys.just_pressed(KeyCode::E) {
        diff = Some(ivec2(1, 0));
    }
    if keys.just_pressed(KeyCode::Comma) {
        diff = Some(ivec2(0, 1));
    }
    if keys.just_pressed(KeyCode::Q) {
        diff = Some(ivec2(0, -1));
    }
    if keys.just_pressed(KeyCode::Apostrophe) {
        diff = Some(ivec2(-1, 1));
    }
    if keys.just_pressed(KeyCode::Period) {
        diff = Some(ivec2(1, 1));
    }
    if keys.just_pressed(KeyCode::Semicolon) {
        diff = Some(ivec2(-1, -1));
    }
    if keys.just_pressed(KeyCode::J) {
        diff = Some(ivec2(1, -1));
    }

    if let Some(diff) = diff {
        let neighbors: HashMap<IVec2, &Obstacle> = obstacles
            .iter()
            .filter_map(|(o, p)| {
                let diff = p.0 - position;
                if diff.x.abs() <= 1 && diff.y.abs() <= 1 {
                    Some((diff, o))
                } else {
                    None
                }
            })
            .collect();

        if neighbors.contains_key(&diff) {
            // TODO In-game logging
            log::info!("Player can't move to {:?}", position + diff);
        } else {
            player.0 += diff;
        }
    }
}
