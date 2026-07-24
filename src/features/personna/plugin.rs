use crate::features::personna::components::PersonnaConfig;
use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

pub struct PersonnaPlugin;

impl Plugin for PersonnaPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<PersonnaConfig>::new(&["personna.ron"]))
            .init_resource::<PersonnaHandle>();
    }
}

#[allow(dead_code)]
#[derive(Resource)]
pub struct PersonnaHandle(pub Handle<PersonnaConfig>);

impl FromWorld for PersonnaHandle {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        info!("The personna file has been loaded");
        PersonnaHandle(asset_server.load("personna.ron"))
    }
}
