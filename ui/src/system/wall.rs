use bevy::{math::ivec2, prelude::*, utils::HashSet};

use crate::{component::*, resource::Deltas};

const EMPTY: char = ' ';
const CROSS: char = '╋';
const HDCROSS: char = '┳';
const HUCROSS: char = '┻';
const VRCROSS: char = '┣';
const VLCROSS: char = '┫';
const TLCORNER: char = '┏';
const TRCORNER: char = '┓';
const BLCORNER: char = '┗';
const BRCORNER: char = '┛';
const HWALL: char = '━';
const VWALL: char = '┃';
const SWALL: char = '░';

pub fn system(
    changed: Query<(), Or<(Changed<Door>, Changed<Wall>)>>,
    mut set: ParamSet<(
        Query<(&mut Renderable, &Position), With<Wall>>,
        Query<&Position, Or<(With<Wall>, With<Door>)>>,
    )>,
    deltas: Res<Deltas>,
) {
    if changed.is_empty() {
        return;
    }

    let index: HashSet<_> = set.p1().iter().map(|p| p.0).collect();
    let deltas = &deltas.0;

    let cross: HashSet<_> = [ivec2(1, 0), ivec2(-1, 0), ivec2(0, 1), ivec2(0, -1)]
        .into_iter()
        .collect();
    let hdcross: HashSet<_> = [ivec2(1, 0), ivec2(-1, 0), ivec2(0, -1)]
        .into_iter()
        .collect();
    let hucross: HashSet<_> = [ivec2(1, 0), ivec2(-1, 0), ivec2(0, 1)]
        .into_iter()
        .collect();
    let vrcross: HashSet<_> = [ivec2(0, -1), ivec2(0, 1), ivec2(1, 0)]
        .into_iter()
        .collect();
    let vlcross: HashSet<_> = [ivec2(0, -1), ivec2(0, 1), ivec2(-1, 0)]
        .into_iter()
        .collect();

    let tlcorner_1: HashSet<_> = deltas
        .difference(&[ivec2(1, -1)].into_iter().collect())
        .cloned()
        .collect();
    let tlcorner_2: HashSet<_> = [ivec2(1, 0), ivec2(0, -1)].into_iter().collect();
    let trcorner_1: HashSet<_> = deltas
        .difference(&[ivec2(-1, -1)].into_iter().collect())
        .cloned()
        .collect();
    let trcorner_2: HashSet<_> = [ivec2(-1, 0), ivec2(0, -1)].into_iter().collect();
    let blcorner_1: HashSet<_> = deltas
        .difference(&[ivec2(1, 1)].into_iter().collect())
        .cloned()
        .collect();
    let blcorner_2: HashSet<_> = [ivec2(0, 1), ivec2(1, 0)].into_iter().collect();
    let brcorner_1: HashSet<_> = deltas
        .difference(&[ivec2(-1, 1)].into_iter().collect())
        .cloned()
        .collect();
    let brcorner_2: HashSet<_> = [ivec2(-1, 0), ivec2(0, 1)].into_iter().collect();

    let hwall_1: HashSet<_> = deltas
        .difference(
            &[ivec2(-1, 1), ivec2(1, 1), ivec2(0, 1)]
                .into_iter()
                .collect(),
        )
        .cloned()
        .collect();
    let hwall_2: HashSet<_> = deltas
        .difference(
            &[ivec2(1, -1), ivec2(-1, -1), ivec2(0, -1)]
                .into_iter()
                .collect(),
        )
        .cloned()
        .collect();
    let hwall_3: HashSet<_> = [ivec2(1, 0)].into_iter().collect();
    let hwall_4: HashSet<_> = [ivec2(-1, 0)].into_iter().collect();
    let vwall_1: HashSet<_> = deltas
        .difference(
            &[ivec2(-1, 0), ivec2(-1, -1), ivec2(-1, 1)]
                .into_iter()
                .collect(),
        )
        .cloned()
        .collect();
    let vwall_2: HashSet<_> = deltas
        .difference(
            &[ivec2(1, 0), ivec2(1, 1), ivec2(1, -1)]
                .into_iter()
                .collect(),
        )
        .cloned()
        .collect();
    let vwall_3: HashSet<_> = [ivec2(0, 1)].into_iter().collect();
    let vwall_4: HashSet<_> = [ivec2(0, -1)].into_iter().collect();

    for (mut renderable, position) in set.p0().iter_mut() {
        let neighbors: HashSet<_> = deltas
            .iter()
            .cloned()
            .filter(|n| index.contains(&(position.0 + *n)))
            .collect();

        if neighbors == *deltas {
            renderable.char = EMPTY;
        } else if blcorner_1.is_subset(&neighbors) {
            renderable.char = BLCORNER;
        } else if brcorner_1.is_subset(&neighbors) {
            renderable.char = BRCORNER;
        } else if tlcorner_1.is_subset(&neighbors) {
            renderable.char = TLCORNER;
        } else if trcorner_1.is_subset(&neighbors) {
            renderable.char = TRCORNER;
        } else if hwall_1.is_subset(&neighbors) || hwall_2.is_subset(&neighbors) {
            renderable.char = HWALL;
        } else if vwall_1.is_subset(&neighbors) || vwall_2.is_subset(&neighbors) {
            renderable.char = VWALL;
        } else if cross.is_subset(&neighbors) {
            renderable.char = CROSS;
        } else if hdcross.is_subset(&neighbors) {
            renderable.char = HDCROSS;
        } else if hucross.is_subset(&neighbors) {
            renderable.char = HUCROSS;
        } else if vrcross.is_subset(&neighbors) {
            renderable.char = VRCROSS;
        } else if vlcross.is_subset(&neighbors) {
            renderable.char = VLCROSS;
        } else if tlcorner_2.is_subset(&neighbors) {
            renderable.char = TLCORNER;
        } else if trcorner_2.is_subset(&neighbors) {
            renderable.char = TRCORNER;
        } else if blcorner_2.is_subset(&neighbors) {
            renderable.char = BLCORNER;
        } else if brcorner_2.is_subset(&neighbors) {
            renderable.char = BRCORNER;
        } else if hwall_3.is_subset(&neighbors) || hwall_4.is_subset(&neighbors) {
            renderable.char = HWALL;
        } else if vwall_3.is_subset(&neighbors) || vwall_4.is_subset(&neighbors) {
            renderable.char = VWALL;
        } else {
            renderable.char = SWALL;
        }
    }
}
