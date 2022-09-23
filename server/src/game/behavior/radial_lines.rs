use std::collections::HashSet;

use bevy_ecs::prelude::*;
use glam::IVec2;
use itertools::Itertools;

use crate::game::resource::{Path, RadialLines};

fn radial_lines_origin(r: u8) -> HashSet<Path> {
    let turn = std::f32::consts::PI * 2.0;
    let segments = (r.pow(2) as f32 * turn).ceil() + 1.0;

    (0..=segments as u16)
        .map(|i| {
            let rad = i as f32 / segments * turn;
            (1..=r)
                .map(move |r| {
                    IVec2::new(
                        (r as f32 * rad.cos()).round() as i32,
                        (r as f32 * rad.sin()).round() as i32,
                    )
                })
                .dedup()
                .collect()
        })
        .collect()
}

pub fn setup(world: &mut World) {
    let lines = radial_lines_origin(10);

    world.insert_resource(RadialLines(lines));
}
