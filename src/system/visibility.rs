use bevy::{prelude::*, utils::HashSet};

use crate::{component::*, resource::RadialLines};

pub fn system(
    mut set: ParamSet<(
        Query<(&mut Visible, &Position), Without<Player>>,
        Query<(&Opaque, &Position)>,
        Query<(&Position, &Sight), With<Player>>,
    )>,
    lines: Res<RadialLines>,
) {
    let (player, sight) = match set.p2().get_single() {
        Ok((position, sight)) => (position.0, sight.clone()),
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

    match sight {
        Sight::Blind => {
            for (mut visible, _) in set.p0().iter_mut() {
                visible.0 = false;
            }
        }
        Sight::Omniscience => {
            for (mut visible, _) in set.p0().iter_mut() {
                visible.0 = true;
            }
        }
        Sight::Eyes => {
            let empty = HashSet::new();
            for (mut visible, position) in set.p0().iter_mut() {
                let position = position.0 - player;

                visible.0 = lines
                    .0
                    .get(&position)
                    .unwrap_or(&empty)
                    .iter()
                    .any(|path| !path.iter().any(|p| obstacles.contains(p)));
            }
        }
        Sight::Sensors => todo!(),
    }
}
