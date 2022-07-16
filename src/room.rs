use bevy::asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset};
use bevy::log;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::utils::HashMap;

use crate::bundle::{Floor, Player, Wall};
use crate::component::Position;
use crate::tile::{ReshockFont, Tile};

#[derive(TypeUuid, Debug)]
#[uuid = "4ca168a0-9d19-4479-a1e1-b74049ade2ee"]
pub struct Room(pub HashMap<IVec2, char>);

impl From<String> for Room {
    fn from(s: String) -> Self {
        let room = s
            .lines()
            .rev()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, c)| ((x as i32, y as i32).into(), c))
            })
            .collect();

        Self(room)
    }
}

#[derive(Default)]
pub struct RoomLoader;

impl AssetLoader for RoomLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let room: Room = String::from_utf8(bytes.to_owned())?.into();
            load_context.set_default_asset(LoadedAsset::new(room));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["room"]
    }
}

pub fn loaded(
    mut commands: Commands,
    mut events: EventReader<AssetEvent<Room>>,
    font: Res<ReshockFont>,
    rooms: ResMut<Assets<Room>>,
) {
    for event in events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                let room = rooms.get(handle).unwrap();
                log::info!("{:?}", room);

                for (pos, c) in room.0.iter() {
                    if *c != ' ' {
                        commands.spawn_bundle(Tile::new(Position(*pos), font.0.clone_weak()));
                        commands.spawn_bundle(Floor::new(Position(*pos)));
                    }

                    match c {
                        '@' => {
                            commands.spawn_bundle(Player::new(Position(*pos)));
                        }
                        'X' => {
                            commands.spawn_bundle(Wall::new(Position(*pos)));
                        }
                        '·' => {}
                        _ => {
                            log::error!("Unknown room char {}", c);
                        }
                    }
                }
            }
            AssetEvent::Modified { .. } | AssetEvent::Removed { .. } => (),
        }
    }
}
