use std::collections::HashMap;
use std::collections::HashSet;

use bevy_ecs::prelude::*;
use glam::ivec2;

use crate::game::component::Direction;
use crate::game::resource::Deltas;

use super::*;

const ROOM_ASSET_PREFIX: &'static str = "assets/rooms/";

pub fn load_asset(name: &str) -> String {
    let path = Path::new(ROOM_ASSET_PREFIX).join(name);
    fs::read_to_string(path)
        .expect("asset can be loaded as string")
        .into()
}

pub trait RoomLoader: 'static {
    fn source() -> String;

    fn char_tile(c: char) -> Option<Tile> {
        match c {
            '@' => Some(Tile::Player),
            'b' => Some(Tile::NPC(NPC::ServBot)),
            'c' => Some(Tile::Object(Object::Server)),
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

    fn load() -> Room {
        let mut index = 0;
        let mut positions = HashMap::new();
        let mut tiles = HashMap::new();
        let mut chars = HashMap::new();
        let mut player = None;
        let mut spawners = HashSet::new();
        let mut walls = HashSet::new();
        let mut doors = HashMap::new();
        let mut bulkhead_doors = HashMap::new();
        let mut width = 0;
        let mut height = 0;

        for (y, line) in Self::source().lines().rev().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let tile = match Self::char_tile(c) {
                    Some(tile) => tile,
                    None => {
                        continue;
                    }
                };

                let pos = ivec2(x as i32, y as i32);
                positions.insert(index, pos);

                match tile {
                    Tile::Wall => {
                        walls.insert(index);
                    }
                    Tile::Door(Door::Spawner) => {
                        doors.insert(pos, index);
                        spawners.insert(index);
                    }
                    Tile::Door(_) => {
                        doors.insert(pos, index);
                    }
                    Tile::Player => {
                        player = Some(index);
                    }
                    _ => {}
                }

                tiles.insert(index, tile);
                chars.insert(index, c);

                width = width.max(x + 1);
                height = height.max(y + 1);
                index += 1;
            }
        }

        let deltas = Deltas::cross();

        let spawners: HashMap<RoomEntity, Direction> = {
            let occupied: HashSet<_> = positions.values().collect();

            spawners
                .into_iter()
                .filter_map(|id| {
                    let pos = positions.get(&id).unwrap();
                    deltas
                        .0
                        .iter()
                        .find(|d| occupied.get(&(*pos + **d)).is_none())
                        .and_then(|d| match (d.x, d.y) {
                            (0, 1) => Some((id, Direction::North)),
                            (1, 0) => Some((id, Direction::East)),
                            (0, -1) => Some((id, Direction::South)),
                            (-1, 0) => Some((id, Direction::West)),
                            _ => {
                                log::warn!("Spawner door without empty adjacent space");
                                None
                            }
                        })
                })
                .collect()
        };

        for (pos, id) in doors.clone() {
            if let Some(other_id) = deltas.0.iter().find_map(|d| doors.get(&(pos + *d))) {
                bulkhead_doors.insert(id, *other_id);
            }
        }

        Room {
            index,
            positions,
            tiles,
            chars,
            player,
            spawners,
            walls,
            bulkhead_doors,
            width: width as u32,
            height: height as u32,
            loader: Arc::new(Self::spawn),
        }
    }

    fn spawn(room: &Room, room_id: RoomId, commands: &mut Commands);
}
