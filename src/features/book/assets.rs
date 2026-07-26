use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::Aseprite;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct BookAssets {
    #[asset(path = "models/book/book.ase")]
    pub book: Handle<Aseprite>,
    #[asset(path = "models/book/book_icon.ase")]
    pub book_icon: Handle<Aseprite>,
}
