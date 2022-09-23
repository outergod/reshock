use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs;
use std::path::Path;
use std::sync::Arc;

use bevy_ecs::prelude::*;
use bevy_hierarchy::BuildChildren;
use glam::{ivec2, IVec2};
use rand::prelude::*;
use strum::{EnumIter, IntoEnumIterator};

use crate::game::bundle;
use crate::game::component;

use self::cyberspace_cache::CyberspaceCacheRoom;
use self::floor_medical::FloorMedicalRoom;
use self::hibernation::HibernationRoom;
use self::loader::RoomLoader;
use self::medical_bay::MedicalBayRoom;
use self::storage::StorageRoom;

use super::component::Direction;

mod cyberspace_cache;
mod floor_medical;
mod hibernation;
mod loader;
mod medical_bay;
mod storage;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RoomId(pub u16);

impl RoomId {
    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

pub struct Rooms(HashMap<RoomAsset, Room>);

impl Default for Rooms {
    fn default() -> Self {
        let assets = RoomAsset::iter()
            .map(|asset| (asset, asset.load()))
            .collect();

        Self(assets)
    }
}

impl Rooms {
    #[allow(dead_code)]
    pub fn get<'a>(&'a self, asset: &RoomAsset) -> &'a Room {
        self.0.get(asset).unwrap()
    }

    pub fn random<P, R>(&self, rng: &mut R, predicate: P) -> Option<Room>
    where
        P: Fn(&Room) -> bool,
        R: Rng + ?Sized,
    {
        self.0
            .values()
            .filter(|room| predicate(*room))
            .choose(rng)
            .cloned()
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, EnumIter)]
pub enum RoomAsset {
    Hibernation,
    MedicalBay,
    FloorMedical,
    Storage,
    CyberspaceCache,
}

impl RoomAsset {
    pub fn load(&self) -> Room {
        match self {
            RoomAsset::Hibernation => HibernationRoom::load(),
            RoomAsset::MedicalBay => MedicalBayRoom::load(),
            RoomAsset::FloorMedical => FloorMedicalRoom::load(),
            RoomAsset::Storage => StorageRoom::load(),
            RoomAsset::CyberspaceCache => CyberspaceCacheRoom::load(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Rotate {
    Zero,
    Ninety,
    OneHundredEighty,
    TwoHundredSeventy,
}

#[derive(Debug, Clone, Copy)]
pub struct Mutation {
    rotate: Rotate,
    mirror: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Tile {
    Floor,
    Wall,
    Door(Door),
    Player,
    NPC(NPC),
}

type RoomEntity = u16;

#[derive(Debug, Clone, Copy)]
pub enum Door {
    Open,
    Closed,
    Spawner,
}

#[derive(Debug, Clone, Copy)]
pub enum NPC {
    ServBot,
}

#[derive(Clone)]
pub struct Room {
    index: RoomEntity,
    positions: HashMap<RoomEntity, IVec2>,
    tiles: HashMap<RoomEntity, Tile>,
    chars: HashMap<RoomEntity, char>,
    player: Option<RoomEntity>,
    spawners: HashMap<RoomEntity, Direction>,
    walls: HashSet<RoomEntity>,
    bulkhead_doors: HashMap<RoomEntity, RoomEntity>,
    width: u32,
    height: u32,
    loader: Arc<dyn Fn(&Room, RoomId, &mut Commands) -> () + Send + Sync>,
}

impl core::fmt::Debug for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Room")
            .field("index", &self.index)
            .field("positions", &self.positions)
            .field("tiles", &self.tiles)
            .field("chars", &self.chars)
            .field("player", &self.player)
            .field("spawners", &self.spawners)
            .field("walls", &self.walls)
            .field("bulkhead_doors", &self.bulkhead_doors)
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}

impl Display for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let positions: HashMap<_, _> = self.positions.iter().map(|(k, v)| (v, k)).collect();
        let mut s = String::new();

        for y in (0..self.height as i32).rev() {
            for x in 0..self.width as i32 {
                let c = match positions
                    .get(&ivec2(x, y))
                    .and_then(|id| self.tiles.get(id).map(|tile| (tile, id)))
                {
                    Some((&Tile::Floor, _)) => '·',
                    Some((&Tile::Wall, _)) => '#',
                    Some((&Tile::Door(Door::Open), _)) => 'O',
                    Some((&Tile::Door(Door::Closed), _)) => 'o',
                    Some((&Tile::Door(Door::Spawner), id)) => {
                        match self.spawners.get(id).unwrap() {
                            Direction::North | Direction::South => '-',
                            Direction::East | Direction::West => '|',
                        }
                    }
                    Some((&Tile::Player, _)) => '@',
                    Some((&Tile::NPC(NPC::ServBot), _)) => 'b',
                    None => ' ',
                };

                s.push(c);
            }
            s.push('\n');
        }

        write!(f, "{}", s)
    }
}

impl Room {
    pub fn position_of<'a>(&'a self, entity: &'a RoomEntity) -> Option<&'a IVec2> {
        self.positions.get(entity)
    }

    pub fn is_dead_end(&self) -> bool {
        self.spawners.len() <= 1
    }

    pub fn erase_player(&mut self) {
        if let Some(id) = self.player {
            *self.tiles.get_mut(&id).unwrap() = Tile::Floor;
        }
        self.player = None;
    }

    pub fn release_spawner(&mut self, entity: &RoomEntity) {
        *self.tiles.get_mut(entity).unwrap() = Tile::Door(Door::Open);
    }

    pub fn random_spawner<'a, R>(&'a self, rng: &mut R) -> Option<&'a RoomEntity>
    where
        R: Rng + ?Sized,
    {
        self.spawners.keys().choose(rng)
    }

    pub fn turn_towards(&mut self, spawner: &RoomEntity, direction: &Direction) -> Result<(), ()> {
        let from = self.spawners.get(spawner).ok_or(())?;
        let diff = *direction as i8 - *from as i8;

        let rotate = match diff {
            0 => Rotate::Zero,
            1 | -3 => Rotate::Ninety,
            2 | -2 => Rotate::OneHundredEighty,
            3 | -1 => Rotate::TwoHundredSeventy,
            _ => panic!("Impossible"),
        };

        self.mutate(&Mutation {
            rotate,
            mirror: false,
        });

        Ok(())
    }

    pub fn spawn(&self, room_id: RoomId, commands: &mut Commands) {
        (self.loader)(self, room_id, commands)
    }

    fn mutate(&mut self, mutation: &Mutation) {
        let Mutation { mirror, rotate } = mutation;

        let width = self.width as i32;
        let height = self.height as i32;

        for (_, pos) in self.positions.iter_mut() {
            match (rotate, mirror) {
                (Rotate::Zero, false) => {}
                (Rotate::Zero, true) => *pos = ivec2(width - 1 - pos.x, pos.y),
                (Rotate::Ninety, false) => *pos = ivec2(pos.y, width - 1 - pos.x),
                (Rotate::Ninety, true) => *pos = ivec2(pos.y, pos.x),
                (Rotate::OneHundredEighty, false) => {
                    *pos = ivec2(width - 1 - pos.x, height - 1 - pos.y)
                }
                (Rotate::OneHundredEighty, true) => *pos = ivec2(pos.x, height - 1 - pos.y),
                (Rotate::TwoHundredSeventy, false) => *pos = ivec2(height - 1 - pos.y, pos.x),
                (Rotate::TwoHundredSeventy, true) => {
                    *pos = ivec2(height - 1 - pos.y, width - 1 - pos.x)
                }
            };
        }

        for (_, direction) in self.spawners.iter_mut() {
            match (*direction as usize + *rotate as usize) % 4 {
                0 => *direction = Direction::North,
                1 => *direction = Direction::East,
                2 => *direction = Direction::South,
                3 => *direction = Direction::West,
                _ => panic!("Impossible"),
            }
        }

        (self.width, self.height) = match rotate {
            Rotate::Zero | Rotate::OneHundredEighty => (self.width, self.height),
            Rotate::Ninety | Rotate::TwoHundredSeventy => (self.height, self.width),
        };
    }
}

// #[cfg(test)]
// mod test {
//     use test_log::test;

//     use super::*;

//     #[test]
//     fn test_room_rotation() {
//         let room: Room = "#@######
// ########
// ########
// "
//         .to_string()
//         .into();

//         assert_eq!(
//             room.to_string(),
//             "#@######
// ########
// ########
// "
//             .to_string(),
//         );

//         let mut mutation = Mutation {
//             mirror: false,
//             rotate: Rotate::Ninety,
//         };

//         let ninety = room.mutate(&mutation);

//         assert_eq!(
//             ninety.to_string(),
//             "###
// ##@
// ###
// ###
// ###
// ###
// ###
// ###
// "
//             .to_string(),
//         );

//         mutation.rotate = Rotate::OneHundredEighty;

//         let oneeighty = room.mutate(&mutation);

//         assert_eq!(
//             oneeighty.to_string(),
//             "########
// ########
// ######@#
// "
//             .to_string(),
//         );

//         mutation.rotate = Rotate::TwoHundredSeventy;

//         let twoseventy = room.mutate(&mutation);

//         assert_eq!(
//             twoseventy.to_string(),
//             "###
// ###
// ###
// ###
// ###
// ###
// @##
// ###
// "
//             .to_string(),
//         );
//     }

//     #[test]
//     fn test_room_mirroring() {
//         let room: Room = "#@######
// ########
// ########
// "
//         .to_string()
//         .into();

//         let mut mutation = Mutation {
//             mirror: true,
//             rotate: Rotate::Zero,
//         };

//         let mirror = room.mutate(&mutation);

//         assert_eq!(
//             mirror.to_string(),
//             "######@#
// ########
// ########
// "
//             .to_string(),
//         );

//         mutation.rotate = Rotate::Ninety;

//         let ninety = room.mutate(&mutation);

//         assert_eq!(
//             ninety.to_string(),
//             "###
// ###
// ###
// ###
// ###
// ###
// ##@
// ###
// "
//             .to_string(),
//         );

//         mutation.rotate = Rotate::OneHundredEighty;

//         let oneeighty = room.mutate(&mutation);

//         assert_eq!(
//             oneeighty.to_string(),
//             "########
// ########
// #@######
// "
//             .to_string(),
//         );

//         mutation.rotate = Rotate::TwoHundredSeventy;

//         let twoseventy = room.mutate(&mutation);

//         assert_eq!(
//             twoseventy.to_string(),
//             "###
// @##
// ###
// ###
// ###
// ###
// ###
// ###
// "
//             .to_string(),
//         );
//     }

//     #[test]
//     fn test_building_trivial() {
//         let base: Room = "
//   ######
// #-#··###
// #······#
// #·····@#
// #o#··###
// #·######
// ###
// "
//         .to_string()
//         .into();

//         let mut spatial: SpatialHash = Default::default();
//         base.spatial_merge(&mut spatial);
//         let find = FindSite::new(spatial.clone());

//         let room: Room = "
// #########
// #··#····#
// #·b·····#
// #·····#·#
// #·#·#·#-#
// #·###·#
// #··#··#
// #····##
// |b·#·##
// #··#··##
// #####··#
//    #####
// "
//         .to_string()
//         .into();

//         let room = find.find_site(&room, (&ivec2(1, 5), &ivec2(1, 6))).unwrap();

//         room.spatial_merge(&mut spatial);

//         let expected = "#########
// #··#····#
// #·······#
// #·····#·######
// #·#·#·#o#··###
// #·###·#······#
// #··#··#······#
// #····##o#··###
// o··#·##·######
// #··#··###
// #####··#
//    #####
// ";

//         assert_eq!(spatial.to_string(), expected);
//     }

//     #[test]
//     fn test_building_offset() {
//         let base: Room = "
// #########
// #··#····#
// #·b·····#
// #·····#·#
// #·#·#·#-#
// #·###·#
// #··#··#
// #····##
// |b·#·##
// #··#··##
// #####··#
//    #####
// "
//         .to_string()
//         .into();

//         let mut spatial: SpatialHash = Default::default();
//         base.spatial_merge(&mut spatial);
//         let find = FindSite::new(spatial.clone());

//         let room: Room = "
//   ######
// #-#··###
// #······#
// #·····@#
// #o#··###
// #·######
// ###
// "
//         .to_string()
//         .into();

//         // let room = find
//         //     .find_site(&room, (&ivec2(0, 3), &ivec2(-1, 3)))
//         //     .unwrap();

//         // let doors = [ivec2(-1, 8)].into_iter().collect();
//         // assert_eq!(room.doors, doors);
//     }

//     #[test]
//     fn test_building_greater_offset() {
//         let base: Room = "
// #########
// #··#····#
// #·b·····#
// #·····#·#
// #·#·#·#-#
// #·###·#
// #··#··#
// #····##
// |b·#·##
// #··#··##
// #####··#
//    #####
// "
//         .to_string()
//         .into();

//         let mut spatial: SpatialHash = Default::default();
//         base.spatial_merge(&mut spatial);
//         let find = FindSite::new(spatial.clone());

//         // println!("{:?}", spatial.cells.get(&ivec2(-1, 3)));

//         let room: Room = "
//   ######
//   #··###
// #-#···##
// #······#
// #·····@#
// #o#··###
// #·######
// ###
// "
//         .to_string()
//         .into();

//         let room = find
//             .find_site(&room, (&ivec2(0, 3), &ivec2(-1, 3)))
//             .unwrap();

//         // let doors = [ivec2(-1, 8)].into_iter().collect();
//         // assert_eq!(room.doors, doors);
//     }
// }
