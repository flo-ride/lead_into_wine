use crate::core::states::GameState;
use crate::features::recipes::components::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

pub struct RecipesPlugin;

impl Plugin for RecipesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<RecipesConfig>::new(&["recipes.ron"]))
            .add_loading_state(
                LoadingState::new(GameState::Loading)
                    .continue_to_state(GameState::initial()) // Corrige la transition
                    .load_collection::<RecipesAssets>(),
            )
            .add_systems(OnEnter(GameState::Playing), print_ingredients);
    }
}

#[derive(AssetCollection, Resource)]
pub struct RecipesAssets {
    #[expect(dead_code)]
    #[asset(path = "config/mixing.recipes.ron")]
    pub recipes: Handle<RecipesConfig>,
}

pub fn print_ingredients(assets: Res<Assets<RecipesConfig>>) {
    let config = assets
        .iter()
        .next()
        .map(|(_, c)| c)
        .expect("Config should be loaded");
    for ingredient in config.ingredients.clone() {
        info!("New ingredients {}", ingredient);
    }
}
