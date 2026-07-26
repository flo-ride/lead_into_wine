use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::Aseprite;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct BackgroundAssets {
    #[asset(path = "textures/background/front_shelf.ase")]
    pub front_shelf: Handle<Aseprite>,
}
