use bevy::asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset};
use bevy::math::IVec2;
use bevy::reflect::TypeUuid;
use bevy::utils::HashMap;

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

// #[derive(TypeUuid, Debug)]
// #[uuid = "06b1db8d-a19c-4a19-a2c6-becc2a01f816"]
// pub struct RadialLines(pub Vec<Vec<Vec<IVec2>>>);
