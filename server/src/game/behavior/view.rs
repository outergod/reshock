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

    let (mut seeing, mask) = match kind {
        SightKind::Blind => (HashMap::new(), HashSet::new()),
        SightKind::Eyes => {
            let viewer = positions.get(actor).unwrap();

            let mut queue = VecDeque::new();
            let mut entities = HashMap::new();
            let mut mask = HashSet::new();

            // This covers a special case: The viewer is standing right in a gateway
            if let Some((position, direction)) = gateways.iter().find_map(|(pos, gateway)| {
                (viewer == pos).then_some((positions.get(gateway.twin).unwrap(), gateway.direction))
            }) {
                let (x, y): (i32, i32) = direction.into();
                let direction = ivec2(x, y);

                let (here, there) = lines.0.clone().into_iter().fold(
                    (HashSet::new(), HashSet::new()),
                    |(mut here, mut there), path| {
                        let v = path[0] * direction;
                        if v.x + v.y > 0 {
                            there.insert(path);
                        } else {
                            here.insert(path);
                        }

                        (here, there)
                    },
                );

                queue.push_back((viewer.clone(), here));
                queue.push_back((position.clone(), there));
            } else {
                queue.push_back((viewer.clone(), lines.0.clone()));
            }

            while let Some((viewer, lines)) = queue.pop_front() {
                let mut followups = HashMap::new();

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

                let obstacles: HashSet<_> = obstacles
                    .iter()
                    .filter_map(|pos| {
                        (pos.room == viewer.room).then_some(pos.coordinates - viewer.coordinates)
                    })
                    .collect();

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

                for path in lines {
                    let mut iter = path.clone().into_iter();
                    while let Some(cell) = iter.next() {
                        if let Some(v) = sights.get(&cell) {
                            for entity in v {
                                entities
                                    .entry(*entity)
                                    .or_insert_with(HashSet::new)
                                    .insert(cell);
                                mask.insert(positions.get(*entity).unwrap().clone());
                            }
                        }

                        if obstacles.contains(&cell) {
                            break;
                        }

                        if let Some(twin) = gateways.get(&cell) {
                            // Position relative to the open gateway's twin
                            let viewer = **twin - cell;

                            followups
                                .entry(viewer)
                                .or_insert_with(HashSet::new)
                                .insert(iter.collect());
                            break;
                        };
                    }
                }

                for (viewer, lines) in followups {
                    queue.push_back((viewer, lines));
                }
            }

            (entities, mask)
        }
    };

    seeing.insert(actor, [ivec2(0, 0)].into());

    let sight = Sight { kind, seeing, mask };

    reactions
        .0
        .push(Action::View(ViewAction::Update { actor, sight }));

    let duration = Instant::now() - now;
    log::debug!("Time taken: {}µs", duration.as_micros());

    Status::Continue
}
