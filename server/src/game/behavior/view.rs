use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::*;

use crate::game::{component::*, pathfinding::euclidian_distance, resource::*, *};

pub fn view_all(
    action: Res<Action>,
    viewers: Query<Entity, With<Sight>>,
    mut reactions: ResMut<Reactions>,
) -> Status {
    match action.as_ref() {
        Action::View(ViewAction::All) => {}
        _ => return Status::Continue,
    };

    for actor in viewers.iter() {
        reactions.0.push(Action::View(ViewAction::Intent { actor }));
    }

    Status::Continue
}

pub fn behavior(
    action: Res<Action>,
    mut reactions: ResMut<Reactions>,
    viewers: Query<&Sight>,
    positions: Query<&Position>,
    gateways: Query<(&Position, &Gateway)>,
    sights: Query<(Entity, &Position), With<Renderable>>,
    obstacles: Query<&Position, With<Opaque>>,
    lines: Res<RadialLines>,
) -> Status {
    let actor = match action.as_ref() {
        Action::View(ViewAction::Intent { actor }) => *actor,
        Action::EndTurn(_) => {
            reactions.0.push(Action::View(ViewAction::All));
            return Status::Continue;
        }
        _ => return Status::Continue,
    };

    let now = Instant::now();

    let kind = viewers.get(actor).unwrap().kind;

    // `seeing` is a mapping of relative coordinates on a single, 2-dimensional
    // plane relative to the viewer at (0, 0). It is completely independent of
    // room dimensions but has to be generated by ray tracing from the viewer's
    // point of view.
    let mut seeing = HashMap::new();
    // `mask`, on the other hand, is a set of all `Position`s seen by the
    // viewer, which includes room dimension information.
    let mut mask = HashSet::new();

    match kind {
        SightKind::Blind => {
            let viewer = positions.get(actor).unwrap();

            // Even if the viewer is blind, they still "see" themselves.
            seeing.insert(actor, [ivec2(0, 0)].into());
            mask.insert(*viewer);
        }
        SightKind::Eyes => {
            let viewer = positions.get(actor).unwrap();

            // Since we need to track every ray as they might hit one or more
            // gateways, we use a queue in place where a functional language
            // would use recursion. We'll have to store the position of the
            // viewer relative to the room being observed, the (remaining) ray
            // segments leading into the room, and the ray's cell leading to the
            // segments separately to prevent infinite recursion when checking
            // gateways. In the current room, the starting cell is always (0, 0).
            let mut queue = VecDeque::new();

            // This covers a special case: The viewer is standing right in a
            // gateway. Rays are grouped into facing inside the gateway, and
            // not. If all rays were used for both gateways, tiles would become
            // visible that should be blocked in sight.
            if let Some((position, direction)) = gateways.iter().find_map(|(pos, gateway)| {
                (viewer == pos).then_some((positions.get(gateway.twin).unwrap(), gateway.direction))
            }) {
                let (x, y): (i32, i32) = direction.into();
                let direction = ivec2(x, y);

                let (here, there) = lines.0.clone().into_iter().fold(
                    (HashSet::new(), HashSet::new()),
                    |(mut here, mut there), path| {
                        // A mathematical trick: To know whether this ray leads
                        // through the gateway, we multiply the first ray cell
                        // with the direction vector; one coordinate in the
                        // vector is always zero, and the other one results in a
                        // positive integer if both signs are identical, so the
                        // sum of the resulting multiple and 0 will also be
                        // positive.
                        let v = path[0] * direction;
                        if v.x + v.y > 0 {
                            there.insert(path);
                        } else {
                            here.insert(path);
                        }

                        (here, there)
                    },
                );

                queue.push_back((viewer.clone(), here, ivec2(0, 0)));
                queue.push_back((position.clone(), there, ivec2(0, 0)));
            } else {
                queue.push_back((viewer.clone(), lines.0.clone(), ivec2(0, 0)));
            }

            while let Some((viewer, lines, start)) = queue.pop_front() {
                // We have to store all the rays that go through a gateway
                // together, for each ray affected.
                let mut followups = HashMap::new();

                // Normalize all sights relative to the viewer's position and
                // index them.
                let sights: HashMap<_, _> =
                    sights
                        .iter()
                        .fold(HashMap::new(), |mut acc, (entity, pos)| {
                            if pos.room == viewer.room
                                && euclidian_distance(&pos.coordinates, &viewer.coordinates) <= 10.0
                            {
                                acc.entry(pos.coordinates - viewer.coordinates)
                                    .or_insert_with(HashSet::new)
                                    .insert(entity);
                            }
                            acc
                        });

                // Same but for visual obstacles, only storing their coordinates within the room.
                let obstacles: HashSet<_> = obstacles
                    .iter()
                    .filter_map(|pos| {
                        (pos.room == viewer.room).then_some(pos.coordinates - viewer.coordinates)
                    })
                    .collect();

                // Same but for gateways, storing the twin's full `Position`.
                let gateways = gateways
                    .iter()
                    .fold(HashMap::new(), |mut acc, (pos, gateway)| {
                        if pos.room == viewer.room {
                            acc.insert(
                                pos.coordinates - viewer.coordinates,
                                positions.get(gateway.twin).unwrap(),
                            );
                        }

                        acc
                    });

                // Collect entities at the starting cell as explained above, to
                // prevent infinite recursion when starting at a gateway twin.
                if let Some(v) = sights.get(&start) {
                    for entity in v {
                        seeing
                            .entry(*entity)
                            .or_insert_with(HashSet::new)
                            .insert(start);
                        mask.insert(positions.get(*entity).unwrap().clone());
                    }
                }

                for path in lines {
                    // We use an iterator so we can split and collect remaining
                    // segments whenever a gateway is hit.
                    let mut iter = path.clone().into_iter();

                    while let Some(cell) = iter.next() {
                        // Collect all sights at `cell`.
                        if let Some(v) = sights.get(&cell) {
                            for entity in v {
                                seeing
                                    .entry(*entity)
                                    .or_insert_with(HashSet::new)
                                    .insert(cell);
                                mask.insert(positions.get(*entity).unwrap().clone());
                            }
                        }

                        // Hit a visual obstacle? Next ray.
                        if obstacles.contains(&cell) {
                            break;
                        }

                        // Hit a gateway?
                        if let Some(twin) = gateways.get(&cell) {
                            // Position relative to the open gateway's twin.
                            // Imagine looking at the other room from a voidlike
                            // space outside the room to understand what is happening.
                            let viewer = **twin - cell;

                            // Collect the remaining segment and the starting cell.
                            followups
                                .entry(viewer)
                                .or_insert_with(|| (HashSet::new(), cell))
                                .0
                                .insert(iter.collect());
                            break;
                        };
                    }
                }

                // Queue up all followup rays per gateway.
                for (viewer, (lines, start)) in followups {
                    queue.push_back((viewer, lines, start));
                }
            }
        }
    };

    let sight = Sight { kind, seeing, mask };

    reactions
        .0
        .push(Action::View(ViewAction::Update { actor, sight }));

    let duration = Instant::now() - now;
    log::debug!("Time taken: {}µs", duration.as_micros());

    Status::Continue
}
