use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs;
use std::path::Path;

use bevy_ecs::prelude::*;
use glam::{ivec2, IVec2};
use rand::distributions::Standard;
use rand::prelude::*;

use crate::game::bundle;
use crate::game::component;

use super::resource::Deltas;
use super::resource::SpatialHash;

const SEARCH_LIMIT: u16 = u16::MAX;

const ROOM_ASSET_PREFIX: &'static str = "assets/rooms/";

pub enum RoomAsset {
    Hibernation,
    MedicalBay,
    Floor,
    Storage,
}

impl Distribution<Room> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Room {
        match rng.gen_range(0..=3) {
            0 => RoomAsset::Hibernation,
            1 => RoomAsset::MedicalBay,
            2 => RoomAsset::Floor,
            3 => RoomAsset::Storage,
            _ => panic!("Impossible"),
        }
        .load()
    }
}

impl RoomAsset {
    pub fn load(&self) -> Room {
        let file = match self {
            RoomAsset::Hibernation => "hibernation.room",
            RoomAsset::MedicalBay => "medical-bay.room",
            RoomAsset::Floor => "floor-1.room",
            RoomAsset::Storage => "storage.room",
        };

        let path = Path::new(ROOM_ASSET_PREFIX).join(file);
        Room::from_asset(path.to_str().unwrap())
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

#[derive(Debug, Clone)]
pub struct Room {
    index: RoomEntity,
    positions: HashMap<RoomEntity, IVec2>,
    tiles: HashMap<RoomEntity, Tile>,
    player: Option<RoomEntity>,
    spawners: HashSet<RoomEntity>,
    walls: HashSet<RoomEntity>,
    width: u32,
    height: u32,
}

impl From<String> for Room {
    fn from(s: String) -> Self {
        let mut index = 0;
        let mut positions = HashMap::new();
        let mut tiles = HashMap::new();
        let mut player = None;
        let mut spawners = HashSet::new();
        let mut walls = HashSet::new();
        let mut width = 0;
        let mut height = 0;

        for (y, line) in s.lines().rev().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let tile = match Self::char_tile(c) {
                    Some(tile) => tile,
                    None => {
                        continue;
                    }
                };

                positions.insert(index, ivec2(x as i32, y as i32));

                match tile {
                    Tile::Wall => {
                        walls.insert(index);
                    }
                    Tile::Door(Door::Spawner) => {
                        spawners.insert(index);
                    }
                    Tile::Player => {
                        player = Some(index);
                    }
                    _ => {}
                }

                tiles.insert(index, tile);

                width = width.max(x + 1);
                height = height.max(y + 1);
                index += 1;
            }
        }

        Self {
            index,
            positions,
            tiles,
            player,
            spawners,
            walls,
            width: width as u32,
            height: height as u32,
        }
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
    pub fn from_asset(asset: &str) -> Self {
        fs::read_to_string(asset)
            .expect("asset can be loaded as string")
            .into()
    }

    fn char_tile(c: char) -> Option<Tile> {
        match c {
            '@' => Some(Tile::Player),
            'b' => Some(Tile::NPC(NPC::ServBot)),
            '#' => Some(Tile::Wall),
            'O' => Some(Tile::Door(Door::Open)),
            'o' => Some(Tile::Door(Door::Closed)),
            '-' | '|' => Some(Tile::Door(Door::Spawner)),
            '·' => Some(Tile::Floor),
            ' ' => None,
            _ => {
                log::error!("Unknown room char {}", c);
                None
            }
        }
    }

    pub fn erase_player(&mut self) {
        if let Some(id) = self.player {
            *self.tiles.get_mut(&id).unwrap() = Tile::Floor;
        }
        self.player = None;
    }

    pub fn spawn(&self, commands: &mut Commands) {
        for (id, pos) in self.positions.iter() {
            let position = component::Position(*pos);

            commands.spawn().insert_bundle(bundle::Floor {
                position: position.clone(),
                ..Default::default()
            });

            match self.tiles.get(id).unwrap() {
                Tile::Floor => {}
                Tile::Wall => {
                    commands.spawn().insert_bundle(bundle::Wall {
                        position,
                        ..Default::default()
                    });
                }
                Tile::Door(Door::Open) => {
                    commands.spawn().insert_bundle(bundle::Door {
                        position,
                        door: component::Door {
                            open: true,
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                }
                Tile::Door(Door::Closed) => {
                    commands
                        .spawn()
                        .insert_bundle(bundle::Door {
                            position,
                            door: component::Door {
                                open: false,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(component::Solid)
                        .insert(component::Opaque);
                }
                Tile::Door(Door::Spawner) => {
                    commands
                        .spawn()
                        .insert_bundle(bundle::Door {
                            position,
                            door: component::Door {
                                open: false,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(component::Solid)
                        .insert(component::Opaque)
                        .insert(component::RoomSpawner);
                }
                Tile::Player => {
                    let player = commands
                        .spawn()
                        .insert_bundle(bundle::Player {
                            position,
                            ..Default::default()
                        })
                        .id();

                    commands
                        .spawn()
                        .insert_bundle(bundle::MeleeWeapon::laser_rapier())
                        .insert(component::Item {
                            owner: Some(player),
                        })
                        .insert(component::Equipped);

                    let rifle = commands
                        .spawn()
                        .insert_bundle(bundle::ProjectileGun::assault_rifle())
                        .insert(component::Item {
                            owner: Some(player),
                        })
                        .insert(component::Equipped)
                        .id();

                    let mut magazine = bundle::Magazine::magnesium_tips();
                    magazine.magazine.attached = Some(rifle);
                    commands.spawn().insert_bundle(magazine);
                }
                Tile::NPC(NPC::ServBot) => {
                    let npc = commands
                        .spawn()
                        .insert_bundle(bundle::NPC {
                            position,
                            ai: component::AI::ServBot,
                            renderable: component::Renderable::ServBot,
                            sight: component::Sight {
                                kind: component::SightKind::Eyes,
                                ..Default::default()
                            },
                            description: component::Description {
                                name: "Serv-Bot unit".into(),
                                article: component::Article::A,
                            },
                            vulnerable: component::Vulnerable {
                                kind: component::VulnerableKind::Robot,
                                hp: 20,
                                max: 20,
                                defense: 2,
                                armor: 20,
                            },
                            ..Default::default()
                        })
                        .insert(component::Alive::ServBot)
                        .id();

                    commands
                        .spawn()
                        .insert_bundle(bundle::NaturalMeleeWeapon::appendages())
                        .insert(component::Item { owner: Some(npc) })
                        .insert(component::Equipped);
                }
            }
        }
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
            player: self.player.clone(),
            spawners: self.spawners.clone(),
            walls: self.walls.clone(),
            width,
            height,
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
    ) -> Option<Self> {
        let (position, path): (&IVec2, HashSet<_>) = match path.split_last() {
            Some((position, path)) => (position, path.iter().collect()),
            None => return None,
        };

        self.spawners
            .clone()
            .into_iter()
            .filter_map(|id| self.positions.get(&id).map(|pos| (id, pos)))
            .find_map(move |(spawner, spawner_pos_room)| {
                let offset = *position - *spawner_pos_room;
                log::trace!(
                    "checking spawner {} at {:?} / {:?}",
                    spawner,
                    spawner_pos_room,
                    offset
                );

                let mut debug_room = self.clone();
                let mut debug_spatial = spatial.clone();
                debug_room
                    .positions
                    .iter_mut()
                    .for_each(|(_, pos)| *pos += offset);
                debug_room.spatial_merge(&mut debug_spatial);
                println!("resulting map\n{}", debug_spatial);

                let mut positions = HashMap::new();
                let mut tiles = HashMap::new();
                let mut player = None;
                let mut spawners = HashSet::new();
                let mut walls = HashSet::new();

                for (id, pos) in self.positions.iter() {
                    let pos = *pos + offset;

                    // No matter what it is, it must not block our path here and back
                    if path.contains(&pos) {
                        println!("{} failing because path back is blocked", pos);
                        return None;
                    }

                    match self.tiles.get(&id).unwrap() {
                        Tile::Wall => match spatial.cells.get(&pos) {
                            // Wall on wall is allowed, preserving the existing wall
                            Some(cell) => {
                                if !cell.wall.is_some() {
                                    println!("{} failing because of wall to not wall", pos);
                                    return None;
                                }
                            }
                            None => {
                                // Never build a wall next to an existing door (should only
                                // affect existing spawners)
                                if deltas
                                    .0
                                    .iter()
                                    .any(|d| spatial.door_at(&(pos + *d)).is_some())
                                {
                                    println!("{} failing because of door neighbor", pos);
                                    return None;
                                }

                                positions.insert(*id, pos);
                                tiles.insert(*id, Tile::Wall);
                                walls.insert(*id);
                            }
                        },

                        Tile::Door(Door::Spawner) => match spatial.cells.get(&pos) {
                            // Spawners can only be placed on the spawner being opened right now, or free space
                            Some(cell) => {
                                if cell.door.is_none() || *id != spawner {
                                    println!("{} failing because of spawner blocked", pos);
                                    return None;
                                }
                            }
                            None => {
                                // Absolutely no straight neighbors next to spawners
                                if deltas
                                    .0
                                    .iter()
                                    .any(|d| spatial.cells.contains_key(&(pos + *d)))
                                {
                                    println!("{} failing because of spawner neighbor", pos);
                                    return None;
                                } else if *id == spawner {
                                    // Currently checked spawner? Become a regular closed door
                                    positions.insert(*id, pos);
                                    tiles.insert(*id, Tile::Door(Door::Closed));
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
                                println!("{} failing because of player blocked", pos);
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
                                println!("{} failing because of {:?} blocked", pos, tile);
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
                    player,
                    spawners,
                    walls,
                    width: self.width,
                    height: self.height,
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
        let deltas = Deltas::cross();
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
            println!("checking {:?}", node.index);

            for room in &rooms {
                let path = Self::recreate_path(&parents, &node.index);
                if let Some(room) = room.attempt_build(&self.spatial, &path, &deltas) {
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
|b·#·###
#··#···#
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

        let room = find
            .find_site(&room, (&ivec2(0, 4), &ivec2(-1, 4)))
            .unwrap();

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
|b·#·###
#··#···#
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
            .find_site(&room, (&ivec2(0, 4), &ivec2(-1, 4)))
            .unwrap();

        // let doors = [ivec2(-1, 8)].into_iter().collect();
        // assert_eq!(room.doors, doors);
    }
}
