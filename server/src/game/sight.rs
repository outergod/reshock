use std::collections::{HashMap, HashSet};

use super::{Resources, World};

pub fn update(world: &World, resources: &Resources) {
    // let (entity, player, sight) = match set.p2().get_single() {
    //     Ok((entity, position, sight, _)) => (entity.clone(), position.0, sight.kind.clone()),
    //     Err(_) => return,
    // };

    // let obstacles: HashSet<_> = set
    //     .p1()
    //     .iter()
    //     .filter_map(|(opaque, position)| {
    //         if opaque.0 {
    //             Some(position.0 - player)
    //         } else {
    //             None
    //         }
    //     })
    //     .collect();

    // let seeing: HashMap<Entity, MemoryComponents> = match sight {
    //     SightKind::Blind => HashMap::new(),
    //     SightKind::Omniscience => set
    //         .p0()
    //         .iter()
    //         .map(
    //             |(entity, position, renderable, ordering, wall, room, door)| {
    //                 (
    //                     entity.clone(),
    //                     MemoryComponents {
    //                         player: None,
    //                         wall: wall.cloned(),
    //                         room: room.cloned(),
    //                         door: door.cloned(),
    //                         renderable: renderable.clone(),
    //                         position: position.clone(),
    //                         ordering: ordering.clone(),
    //                     },
    //                 )
    //             },
    //         )
    //         .collect(),
    //     SightKind::Eyes => {
    //         let empty = HashSet::new();

    //         set.p0()
    //             .iter()
    //             .filter_map(
    //                 |(entity, position, renderable, ordering, wall, room, door)| {
    //                     let pos = position.0 - player;
    //                     if lines
    //                         .0
    //                         .get(&pos)
    //                         .unwrap_or(&empty)
    //                         .iter()
    //                         .any(|path| !path.iter().any(|p| obstacles.contains(p)))
    //                     {
    //                         Some((
    //                             entity.clone(),
    //                             MemoryComponents {
    //                                 player: None,
    //                                 wall: wall.cloned(),
    //                                 room: room.cloned(),
    //                                 door: door.cloned(),
    //                                 renderable: renderable.clone(),
    //                                 position: position.clone(),
    //                                 ordering: ordering.clone(),
    //                             },
    //                         ))
    //                     } else {
    //                         None
    //                     }
    //                 },
    //             )
    //             .collect()
    //     }
    // };

    // match set.p2().get_single_mut() {
    //     Ok((_, _, mut sight, mut memory)) => {
    //         sight.seeing = seeing.keys().cloned().collect();
    //         sight.seeing.insert(entity);
    //         memory.entities.extend(seeing);
    //         events.0.push(api::Event {
    //             event: Some(api::event::Event::View(api::ViewUpdateEvent {
    //                 entity: entity.id(),
    //                 seeing: sight.seeing.iter().map(|e| e.id()).collect(),
    //             })),
    //         });
    //         events.0.push(api::Event {
    //             event: Some(api::event::Event::Memory(api::MemoryUpdateEvent {
    //                 entity: entity.id(),
    //                 memory: memory.clone().into(),
    //             })),
    //         });
    //     }
    //     Err(_) => return,
    // }
}
