use bevy::prelude::*;
mod alchemy;
mod core;
mod environment;
mod features;
mod interaction;
mod physics;
mod ui;

use bevy_aseprite_ultra::AsepriteUltraPlugin;
use features::level_loop::plugin::LevelLoopPlugin;
use features::personna::plugin::PersonnaPlugin;
use features::recipes::plugin::RecipesPlugin;
use features::shelf::plugin::ShelfPlugin;

pub fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: "assets".into(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (1280, 720).into(),
                        title: "Lead Into Wine".into(),
                        // Tells Wasm to resize the window according to the available canvas
                        fit_canvas_to_parent: true,

                        name: Some("lead_into_wine.app".into()),
                        // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                        prevent_default_event_handling: false,
                        window_theme: Some(bevy::window::WindowTheme::Dark),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(core::states::StatePlugin)
        .add_plugins(core::assets::AssetPlugin)
        .add_plugins(physics::PhysicsPlugin)
        .add_plugins(interaction::InteractionPlugin)
        .add_plugins(alchemy::AlchemyPlugin)
        .add_systems(Startup, setup_camera)
        .add_plugins(LevelLoopPlugin)
        .add_plugins(PersonnaPlugin)
        .add_plugins(RecipesPlugin)
        .add_plugins(ui::MenuPlugin)
        .add_plugins(ShelfPlugin)
        .add_plugins(environment::EnvironmentPlugin)
        .add_plugins(AsepriteUltraPlugin)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        core::components::MainCamera,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::camera::ScalingMode::Fixed {
                width: 1280.0,
                height: 720.0,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}
