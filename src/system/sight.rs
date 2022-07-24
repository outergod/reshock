use bevy::{prelude::*, utils::HashSet};

use crate::{component::*, resource::RadialLines};

pub fn system(
    mut set: ParamSet<(
        Query<(Entity, &Position), (With<Renderable>, Without<Player>)>,
        Query<(&Opaque, &Position)>,
        Query<(Entity, &Position, &mut Sight), With<Player>>,
    )>,
    lines: Res<RadialLines>,
) {
    let (entity, player, sight) = match set.p2().get_single() {
        Ok((entity, position, sight)) => (entity, position.0, sight.kind.clone()),
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

    let mut seeing = match sight {
        SightKind::Blind => HashSet::new(),
        SightKind::Omniscience => set.p0().iter().map(|(entity, _)| entity).collect(),
        SightKind::Eyes => {
            let empty = HashSet::new();

            set.p0()
                .iter()
                .filter_map(|(entity, position)| {
                    let position = position.0 - player;
                    if lines
                        .0
                        .get(&position)
                        .unwrap_or(&empty)
                        .iter()
                        .any(|path| !path.iter().any(|p| obstacles.contains(p)))
                    {
                        Some(entity)
                    } else {
                        None
                    }
                })
                .collect()
        }
        SightKind::Sensors => todo!(),
    };

    seeing.insert(entity);

    set.p2().get_single_mut().unwrap().2.seeing = seeing;
}
