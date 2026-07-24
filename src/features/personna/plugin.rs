use crate::features::personna::components::PersonnaConfig;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

pub struct PersonnaPlugin;

impl Plugin for PersonnaPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<PersonnaConfig>::new(&["personna.ron"]))
            .init_collection::<PersonnaAssets>();
    }
}

#[allow(dead_code)]
#[derive(AssetCollection, Resource)]
struct PersonnaAssets {
    #[asset(path = "personna.ron")]
    personna: Handle<PersonnaConfig>,
}
