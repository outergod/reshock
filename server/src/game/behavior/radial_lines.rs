use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::*;
use glam::IVec2;
use itertools::Itertools;

use crate::game::resource::{Path, RadialLines};

fn radial_lines_origin(r: u8) -> HashMap<IVec2, HashSet<Path>> {
    let turn = std::f32::consts::PI * 2.0;
    let segments = (r.pow(2) as f32 * turn).ceil() + 1.0;

    (0..=segments as u16)
        .flat_map(|i| {
            let rad = i as f32 / segments * turn;
            let path: Vec<_> = (1..=r)
                .map(move |r| {
                    IVec2::new(
                        (r as f32 * rad.cos()).round() as i32,
                        (r as f32 * rad.sin()).round() as i32,
                    )
                })
                .dedup()
                .collect();

            (1..=r).map(move |r| path.iter().take(r as usize).cloned().collect::<Vec<_>>())
        })
        .fold(HashMap::new(), |mut acc, path| {
            let (key, value) = path.split_last().unwrap();
            acc.entry(*key)
                .or_insert_with(HashSet::new)
                .insert(value.to_vec());
            acc
        })
}

pub fn setup(world: &mut World) {
    let lines = radial_lines_origin(10);
    world.insert_resource(RadialLines(lines));
}

#[cfg(test)]
mod test {
    // use bevy::math::IVec2;

    use super::radial_lines_origin;

    #[test]
    fn test_lines() {
        let lines = radial_lines_origin(2);
        println!("{:#?}", lines.keys());

        // let lines = radial_lines_origin(10);
        // println!("{:#?}", lines.get(&IVec2::new(1, -1)));
    }
}
