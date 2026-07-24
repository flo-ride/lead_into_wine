use crate::core::states::GameState;
use crate::features::personna::components::PersonnaConfig;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

pub struct PersonnaPlugin;

impl Plugin for PersonnaPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<PersonnaConfig>::new(&["ron"]))
            .add_loading_state(
                LoadingState::new(GameState::Loading)
                    .continue_to_state(GameState::initial())
                    .load_collection::<PersonnaAssets>(),
            );
    }
}

#[derive(AssetCollection, Resource)]
pub struct PersonnaAssets {
    #[asset(path = "personna.ron")]
    pub personna: Handle<PersonnaConfig>,
}
