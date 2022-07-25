use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};

use crate::{component::*, resource::RadialLines};

pub fn system(
    mut set: ParamSet<(
        Query<(Entity, &Position, &Renderable, &Ordering), Without<Player>>,
        Query<(&Opaque, &Position)>,
        Query<(Entity, &Position, &mut Sight, &mut Memory), With<Player>>,
    )>,
    lines: Res<RadialLines>,
) {
    let (entity, player, sight) = match set.p2().get_single() {
        Ok((entity, position, sight, _)) => (entity, position.0, sight.kind.clone()),
        Err(_) => return,
    };

    let obstacles: HashSet<_> = set
        .p1()
        .iter()
        .filter_map(|(opaque, position)| {
            if opaque.0 {
                Some(position.0 - player)
            } else {
                None
            }
        })
        .collect();

    let seeing: HashMap<Entity, MemoryComponents> = match sight {
        SightKind::Blind => HashMap::new(),
        SightKind::Omniscience => set
            .p0()
            .iter()
            .map(|(entity, position, renderable, ordering)| {
                (
                    entity,
                    MemoryComponents {
                        renderable: renderable.clone(),
                        position: position.clone(),
                        ordering: ordering.clone(),
                    },
                )
            })
            .collect(),
        SightKind::Eyes => {
            let empty = HashSet::new();

            set.p0()
                .iter()
                .filter_map(|(entity, position, renderable, ordering)| {
                    let pos = position.0 - player;
                    if lines
                        .0
                        .get(&pos)
                        .unwrap_or(&empty)
                        .iter()
                        .any(|path| !path.iter().any(|p| obstacles.contains(p)))
                    {
                        Some((
                            entity,
                            MemoryComponents {
                                renderable: renderable.clone(),
                                position: position.clone(),
                                ordering: ordering.clone(),
                            },
                        ))
                    } else {
                        None
                    }
                })
                .collect()
        }
        SightKind::Sensors => todo!(),
    };

    match set.p2().get_single_mut() {
        Ok((_, _, mut sight, mut memory)) => {
            sight.seeing = seeing.keys().cloned().collect();
            sight.seeing.insert(entity);
            memory.0.extend(seeing);
        }
        Err(_) => return,
    }
}
