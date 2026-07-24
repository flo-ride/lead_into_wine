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
                    .continue_to_state(GameState::initial()) // Corrige la transition
                    .load_collection::<PersonnaAssets>(),
            )
            // On exécute le print seulement pendant le jeu
            .add_systems(Update, print_pnj_name.run_if(in_state(GameState::Playing)));
    }
}

#[derive(AssetCollection, Resource)]
pub struct PersonnaAssets {
    #[asset(path = "personna.ron")]
    pub personna: Handle<PersonnaConfig>,
}

fn print_pnj_name(
    personna_assets: Option<Res<PersonnaAssets>>,
    configs: Res<Assets<PersonnaConfig>>,
) {
    let Some(personna_assets) = personna_assets else {
        println!("Pas d'assets");
        return; // resource pas encore là, on skip cette frame
    };
    if let Some(config) = configs.get(&personna_assets.personna) {
        for name in config.personas.keys() {
            println!("Persona: {}", name);
        }
    }
}
