use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::Aseprite;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct BackgroundAssets {
    #[asset(path = "textures/background/tavern_shelf.aseprite")]
    pub front_shelf: Handle<Aseprite>,
}
