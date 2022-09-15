use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs;
use std::path::Path;
use std::sync::Arc;

use bevy_ecs::prelude::*;
use bevy_hierarchy::BuildChildren;
use glam::{ivec2, IVec2};
use log::log_enabled;
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

use super::resource::Deltas;
use super::resource::SpatialHash;

mod cyberspace_cache;
mod floor_medical;
mod hibernation;
mod loader;
mod medical_bay;
mod storage;

const SEARCH_LIMIT: u8 = u8::MAX;

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

    pub fn random<P, R>(&self, rng: &mut R, predicate: Option<P>) -> Option<Room>
    where
        P: Fn(&Room) -> bool,
        R: Rng + ?Sized,
    {
        match predicate {
            Some(p) => self.0.values().filter(|room| p(*room)).choose(rng),
            None => self.0.values().choose(rng),
        }
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
    spawners: HashSet<RoomEntity>,
    walls: HashSet<RoomEntity>,
    bulkhead_doors: HashMap<RoomEntity, RoomEntity>,
    width: u32,
    height: u32,
    loader: Arc<dyn Fn(&Room, &mut Commands) -> () + Send + Sync>,
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
                    .and_then(|id| self.tiles.get(id))
                {
                    Some(&Tile::Floor) => '·',
                    Some(&Tile::Wall) => '#',
                    Some(&Tile::Door(Door::Open)) => 'O',
                    Some(&Tile::Door(Door::Closed)) => 'o',
                    Some(&Tile::Door(Door::Spawner)) => '+',
                    Some(&Tile::Player) => '@',
                    Some(&Tile::NPC(NPC::ServBot)) => 'b',
                    None => ' ',
                };

                s.push(c);
            }
            s.push('\n');
        }

        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub struct Unfit;

impl Room {
    pub fn is_dead_end(&self) -> bool {
        self.spawners.len() <= 1
    }

    pub fn erase_player(&mut self) {
        if let Some(id) = self.player {
            *self.tiles.get_mut(&id).unwrap() = Tile::Floor;
        }
        self.player = None;
    }

    pub fn spawn(&self, commands: &mut Commands) {
        (self.loader)(self, commands)
    }

    fn mutations(&self) -> Vec<Self> {
        [
            Rotate::Zero,
            Rotate::Ninety,
            Rotate::OneHundredEighty,
            Rotate::TwoHundredSeventy,
        ]
        .into_iter()
        .flat_map(|rotate| {
            [false, true]
                .into_iter()
                .map(move |mirror| self.mutate(&Mutation { rotate, mirror }))
        })
        .collect()
    }

    fn mutate(&self, mutation: &Mutation) -> Self {
        let Mutation { mirror, rotate } = mutation;

        let width = self.width as i32;
        let height = self.height as i32;

        let positions = self
            .positions
            .clone()
            .into_iter()
            .map(|(entity, pos)| {
                let (x, y) = match (rotate, mirror) {
                    (Rotate::Zero, false) => (pos.x, pos.y),
                    (Rotate::Zero, true) => (width - 1 - pos.x, pos.y),
                    (Rotate::Ninety, false) => (pos.y, width - 1 - pos.x),
                    (Rotate::Ninety, true) => (pos.y, pos.x),
                    (Rotate::OneHundredEighty, false) => (width - 1 - pos.x, height - 1 - pos.y),
                    (Rotate::OneHundredEighty, true) => (pos.x, height - 1 - pos.y),
                    (Rotate::TwoHundredSeventy, false) => (height - 1 - pos.y, pos.x),
                    (Rotate::TwoHundredSeventy, true) => (height - 1 - pos.y, width - 1 - pos.x),
                };

                (entity, ivec2(x, y))
            })
            .collect();

        let (width, height) = match rotate {
            Rotate::Zero | Rotate::OneHundredEighty => (self.width, self.height),
            Rotate::Ninety | Rotate::TwoHundredSeventy => (self.height, self.width),
        };

        Self {
            index: self.index,
            positions,
            tiles: self.tiles.clone(),
            chars: self.chars.clone(),
            player: self.player.clone(),
            spawners: self.spawners.clone(),
            walls: self.walls.clone(),
            bulkhead_doors: self.bulkhead_doors.clone(),
            width,
            height,
            loader: self.loader.clone(),
        }
    }

    // Used for debugging
    fn spatial_merge(&self, spatial: &mut SpatialHash) {
        for (id, pos) in self.positions.iter() {
            let pos = *pos;
            match self.tiles.get(&id).unwrap() {
                Tile::Wall => {
                    let mut cell = spatial.cells.entry(pos).or_default();
                    cell.wall = Some(Entity::from_raw((*id).into()));
                }
                Tile::Door(_) => {
                    let mut cell = spatial.cells.entry(pos).or_default();
                    cell.door = Some(Entity::from_raw((*id).into()));
                }
                _ => {
                    spatial.cells.entry(pos).or_default();
                }
            }
        }
    }

    fn attempt_build(
        &self,
        spatial: &SpatialHash,
        path: &Vec<IVec2>,
        deltas: &Deltas,
        straight_deltas: &Deltas,
    ) -> Option<Self> {
        let (position, path): (&IVec2, HashSet<_>) = match path.split_last() {
            Some((position, path)) => (position, path.iter().collect()),
            None => return None,
        };

        self.spawners
            .clone()
            .into_iter()
            .filter(|id| self.bulkhead_doors.get(&id).is_none())
            .filter_map(|id| self.positions.get(&id).map(|pos| (id, pos)))
            .find_map(move |(spawner, spawner_pos_room)| {
                let offset = *position - *spawner_pos_room;

                if log_enabled!(log::Level::Debug) {
                    let mut debug_room = self.clone();
                    let mut debug_spatial = spatial.clone();
                    debug_room
                        .positions
                        .iter_mut()
                        .for_each(|(_, pos)| *pos += offset);
                    debug_room.spatial_merge(&mut debug_spatial);
                    log::debug!("resulting map\n{}", debug_spatial);
                }

                let mut positions = HashMap::new();
                let mut tiles = HashMap::new();
                let mut player = None;
                let mut spawners = HashSet::new();
                let mut walls = HashSet::new();

                for (id, pos) in self.positions.iter() {
                    let pos = *pos + offset;

                    // No matter what it is, it must not block our path here and back
                    if &pos != position && path.contains(&pos) {
                        log::debug!("{} {} failing because path back is blocked", pos, position);
                        return None;
                    }

                    match self.tiles.get(&id).unwrap() {
                        Tile::Wall => match spatial.cells.get(&pos) {
                            // Wall on wall is allowed, preserving the existing wall
                            Some(cell) => {
                                if !cell.wall.is_some() {
                                    log::debug!("{} failing because of wall to not wall", pos);
                                    return None;
                                }
                            }
                            None => {
                                // Never build a wall next to an existing door (should only
                                // affect existing spawners)
                                if straight_deltas
                                    .0
                                    .iter()
                                    .any(|d| spatial.door_at(&(pos + *d)).is_some())
                                {
                                    log::debug!("{} failing because of door neighbor", pos);
                                    return None;
                                }

                                positions.insert(*id, pos);
                                tiles.insert(*id, Tile::Wall);
                                walls.insert(*id);
                            }
                        },

                        Tile::Door(Door::Spawner) => match spatial.cells.get(&pos) {
                            // Spawners can only be placed on the spawner being opened right now
                            Some(cell) => {
                                if cell.door.is_none() || id != &spawner {
                                    log::debug!("{} failing because of spawner blocked", pos);
                                    return None;
                                }
                            }
                            None => {
                                // Absolutely no straight neighbors next to spawners
                                if straight_deltas
                                    .0
                                    .iter()
                                    .any(|d| spatial.cells.contains_key(&(pos + *d)))
                                {
                                    log::debug!("{} failing because of spawner neighbor", pos);
                                    return None;
                                } else if id == &spawner {
                                    // Currently checked spawner? Become a piece of floor
                                    positions.insert(*id, pos);
                                    tiles.insert(*id, Tile::Floor);
                                } else {
                                    // Other spawner? Stays
                                    positions.insert(*id, pos);
                                    tiles.insert(*id, Tile::Door(Door::Spawner));
                                    spawners.insert(*id);
                                }
                            }
                        },

                        Tile::Player => match spatial.cells.get(&pos) {
                            Some(_) => {
                                log::debug!("{} failing because of player blocked", pos);
                                return None;
                            }
                            None => {
                                // Check whether a player exists already later
                                positions.insert(*id, pos);
                                tiles.insert(*id, Tile::Player);
                                player = Some(*id);
                            }
                        },

                        tile => match spatial.cells.get(&pos) {
                            Some(_) => {
                                log::debug!("{} failing because of {:?} blocked", pos, tile);
                                return None;
                            }
                            None => {
                                positions.insert(*id, pos);
                                tiles.insert(*id, *tile);
                            }
                        },
                    }
                }

                let mut index = self.index;

                // Create a pathway from door to door
                for pos in path.clone().into_iter() {
                    for pos in deltas.0.iter().filter_map(|d| {
                        let pos = *pos + *d;
                        if spatial.entities_at(&pos).is_empty()
                            && !path.contains(&pos)
                            && pos != *position
                        {
                            Some(pos)
                        } else {
                            None
                        }
                    }) {
                        // Building a wall next to a spawner? Abort!
                        if straight_deltas
                            .0
                            .iter()
                            .any(|d| spatial.door_at(&(pos + *d)).is_some())
                        {
                            log::debug!(
                                "failing because wall {} would be built next to spawner",
                                pos
                            );
                            return None;
                        }

                        index += 1;
                        positions.insert(index, pos);
                        tiles.insert(index, Tile::Wall);
                    }

                    index += 1;
                    positions.insert(index, *pos);
                    tiles.insert(index, Tile::Floor);
                }

                Some(Room {
                    index,
                    positions,
                    tiles,
                    chars: self.chars.clone(),
                    player,
                    spawners,
                    walls,
                    bulkhead_doors: self.bulkhead_doors.clone(),
                    width: self.width,
                    height: self.height,
                    loader: self.loader.clone(),
                })
            })
    }
}

pub struct FindSite {
    spatial: SpatialHash,
    moves: HashMap<IVec2, f32>,
}

impl FindSite {
    pub fn new(spatial: SpatialHash) -> Self {
        Self {
            spatial,
            moves: Self::moves(),
        }
    }

    fn moves() -> HashMap<IVec2, f32> {
        Deltas::cross().0.into_iter().map(|d| (d, 1.0)).collect()
    }

    fn recreate_path(parents: &HashMap<IVec2, Node>, goal: &IVec2) -> Vec<IVec2> {
        let mut path = Vec::new();
        let mut index = goal;

        path.push(*index);

        while let Some(parent) = parents.get(&index) {
            path.push(parent.index);
            index = &parent.index;
        }
        path.reverse();

        path
    }

    fn neighbors(&self, closed: &HashSet<IVec2>, node: &Node) -> Vec<Node> {
        self.moves
            .iter()
            .filter_map(|(delta, weight)| {
                let index = node.index + *delta;
                let f = node.f + *weight;

                if closed.contains(&index) || !self.spatial.entities_at(&index).is_empty() {
                    None
                } else {
                    Some(Node { index, f })
                }
            })
            .collect()
    }

    pub fn find_site(&self, room: &Room, start: (&IVec2, &IVec2)) -> Option<Room> {
        let mut fringe: BinaryHeap<Node> = BinaryHeap::new();
        let mut closed: HashSet<IVec2> = HashSet::new();
        let mut parents: HashMap<IVec2, Node> = HashMap::new();
        let straight_deltas = Deltas::cross();
        let deltas = Deltas::neighbors();
        let rooms = room.mutations();

        fringe.push(Node {
            index: *start.0,
            f: 0.0,
        });
        fringe.push(Node {
            index: *start.1,
            f: 1.0,
        });
        closed.insert(*start.0);
        closed.insert(*start.1);

        let mut steps = 0;

        while steps < SEARCH_LIMIT && let Some(node) = fringe.pop() {
            steps += 1;
            log::debug!("checking {:?}", node.index);

            for room in &rooms {
                let path = Self::recreate_path(&parents, &node.index);
                if let Some(room) = room.attempt_build(&self.spatial, &path, &deltas, &straight_deltas) {
                    return Some(room);
                }
            }

            for neighbor in self.neighbors(&closed, &node) {
                fringe.push(neighbor.clone());
                closed.insert(neighbor.index);
                parents.insert(neighbor.index, node.clone());
            }
        }

        None
    }
}

#[derive(Debug, Clone)]
struct Node {
    index: IVec2,
    f: f32,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.index.x == other.index.x && self.index.y == other.index.y
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.f.partial_cmp(&self.f)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.partial_cmp(&self.f).unwrap()
    }
}

#[cfg(test)]
mod test {
    use test_log::test;

    use super::*;

    #[test]
    fn test_room_rotation() {
        let room: Room = "#@######
########
########
"
        .to_string()
        .into();

        assert_eq!(
            room.to_string(),
            "#@######
########
########
"
            .to_string(),
        );

        let mut mutation = Mutation {
            mirror: false,
            rotate: Rotate::Ninety,
        };

        let ninety = room.mutate(&mutation);

        assert_eq!(
            ninety.to_string(),
            "###
##@
###
###
###
###
###
###
"
            .to_string(),
        );

        mutation.rotate = Rotate::OneHundredEighty;

        let oneeighty = room.mutate(&mutation);

        assert_eq!(
            oneeighty.to_string(),
            "########
########
######@#
"
            .to_string(),
        );

        mutation.rotate = Rotate::TwoHundredSeventy;

        let twoseventy = room.mutate(&mutation);

        assert_eq!(
            twoseventy.to_string(),
            "###
###
###
###
###
###
@##
###
"
            .to_string(),
        );
    }

    #[test]
    fn test_room_mirroring() {
        let room: Room = "#@######
########
########
"
        .to_string()
        .into();

        let mut mutation = Mutation {
            mirror: true,
            rotate: Rotate::Zero,
        };

        let mirror = room.mutate(&mutation);

        assert_eq!(
            mirror.to_string(),
            "######@#
########
########
"
            .to_string(),
        );

        mutation.rotate = Rotate::Ninety;

        let ninety = room.mutate(&mutation);

        assert_eq!(
            ninety.to_string(),
            "###
###
###
###
###
###
##@
###
"
            .to_string(),
        );

        mutation.rotate = Rotate::OneHundredEighty;

        let oneeighty = room.mutate(&mutation);

        assert_eq!(
            oneeighty.to_string(),
            "########
########
#@######
"
            .to_string(),
        );

        mutation.rotate = Rotate::TwoHundredSeventy;

        let twoseventy = room.mutate(&mutation);

        assert_eq!(
            twoseventy.to_string(),
            "###
@##
###
###
###
###
###
###
"
            .to_string(),
        );
    }

    #[test]
    fn test_building_trivial() {
        let base: Room = "
  ######
#-#··###
#······#
#·····@#
#o#··###
#·######
###
"
        .to_string()
        .into();

        let mut spatial: SpatialHash = Default::default();
        base.spatial_merge(&mut spatial);
        let find = FindSite::new(spatial.clone());

        let room: Room = "
#########
#··#····#
#·b·····#
#·····#·#
#·#·#·#-#
#·###·#
#··#··#
#····##
|b·#·##
#··#··##
#####··#
   #####
"
        .to_string()
        .into();

        let room = find.find_site(&room, (&ivec2(1, 5), &ivec2(1, 6))).unwrap();

        room.spatial_merge(&mut spatial);

        let expected = "#########     
#··#····#     
#·······#     
#·····#·######
#·#·#·#o#··###
#·###·#······#
#··#··#······#
#····##o#··###
o··#·##·######
#··#··###     
#####··#      
   #####      
";

        assert_eq!(spatial.to_string(), expected);
    }

    #[test]
    fn test_building_offset() {
        let base: Room = "
#########
#··#····#
#·b·····#
#·····#·#
#·#·#·#-#
#·###·#
#··#··#
#····##
|b·#·##
#··#··##
#####··#
   #####
"
        .to_string()
        .into();

        let mut spatial: SpatialHash = Default::default();
        base.spatial_merge(&mut spatial);
        let find = FindSite::new(spatial.clone());

        let room: Room = "
  ######
#-#··###
#······#
#·····@#
#o#··###
#·######
###
"
        .to_string()
        .into();

        // let room = find
        //     .find_site(&room, (&ivec2(0, 3), &ivec2(-1, 3)))
        //     .unwrap();

        // let doors = [ivec2(-1, 8)].into_iter().collect();
        // assert_eq!(room.doors, doors);
    }

    #[test]
    fn test_building_greater_offset() {
        let base: Room = "
#########
#··#····#
#·b·····#
#·····#·#
#·#·#·#-#
#·###·#
#··#··#
#····##
|b·#·##
#··#··##
#####··#
   #####
"
        .to_string()
        .into();

        let mut spatial: SpatialHash = Default::default();
        base.spatial_merge(&mut spatial);
        let find = FindSite::new(spatial.clone());

        // println!("{:?}", spatial.cells.get(&ivec2(-1, 3)));

        let room: Room = "
  ######
  #··###
#-#···##
#······#
#·····@#
#o#··###
#·######
###
"
        .to_string()
        .into();

        let room = find
            .find_site(&room, (&ivec2(0, 3), &ivec2(-1, 3)))
            .unwrap();

        // let doors = [ivec2(-1, 8)].into_iter().collect();
        // assert_eq!(room.doors, doors);
    }
}
