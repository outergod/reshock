use bevy::{math::ivec2, prelude::*, utils::HashSet};

use crate::component::*;

const VDOOR: char = '║';
const HDOOR: char = '═';
const DOOR: char = '+';

pub fn system(
    mut set: ParamSet<(
        Query<(
            &mut Renderable,
            &mut Opaque,
            &mut Obstacle,
            &Door,
            &Position,
        )>,
        Query<&Position, With<Room>>,
    )>,
) {
    let index: HashSet<_> = set.p1().iter().map(|p| p.0).collect();

    let deltas: HashSet<_> = (-1..=1)
        .flat_map(|x| {
            (-1..=1).filter_map(move |y| {
                if x == 0 && y == 0 {
                    None
                } else {
                    Some(ivec2(x, y))
                }
            })
        })
        .collect();

    let vdoor: HashSet<_> = [ivec2(0, -1), ivec2(0, 1)].into_iter().collect();
    let hdoor: HashSet<_> = [ivec2(1, 0), ivec2(1, 0)].into_iter().collect();

    for (mut renderable, mut opaque, mut obstacle, door, position) in set.p0().iter_mut() {
        let neighbors: HashSet<_> = deltas
            .iter()
            .cloned()
            .filter(|n| index.contains(&(position.0 + *n)))
            .collect();

        if vdoor.is_subset(&neighbors) {
            renderable.char = VDOOR;
        } else if hdoor.is_subset(&neighbors) {
            renderable.char = HDOOR;
        } else {
            renderable.char = DOOR;
        }

        if door.open {
            renderable.color = Color::DARK_GRAY;
            opaque.0 = false;
            obstacle.0 = false;
        } else {
            renderable.color = Color::WHITE;
            opaque.0 = true;
            obstacle.0 = true;
        }
    }
}
