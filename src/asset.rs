use bevy::asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset};
use bevy::reflect::TypeUuid;

#[derive(TypeUuid)]
#[uuid = "4ca168a0-9d19-4479-a1e1-b74049ade2ee"]
pub struct Room(pub String);

#[derive(Default)]
pub struct RoomLoader;

impl AssetLoader for RoomLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let room = Room(String::from_utf8(bytes.to_owned())?);
            load_context.set_default_asset(LoadedAsset::new(room));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["room"]
    }
}
