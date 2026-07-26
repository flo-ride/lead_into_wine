use crate::alchemy::{LiquidContainer, LiquidVisual};

use crate::core::components::Scroll;
use crate::core::states::GameState;
use crate::interaction::Draggable;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_aseprite_ultra::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_environment)
            .add_loading_state(
                LoadingState::new(GameState::Loading)
                    .continue_to_state(GameState::initial())
                    .load_collection::<UiAssets>(),
            );
    }
}

#[derive(AssetCollection, Resource)]
pub struct UiAssets {
    #[asset(path = "textures/background/front.ase")]
    pub background: Handle<Aseprite>,

    #[asset(path = "models/scroll.aseprite")]
    pub scroll: Handle<Aseprite>,

    #[asset(path = "models/mug/cut_mug.ase")]
    pub mug: Handle<Aseprite>,
}

fn setup_environment(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ui_assets: Res<UiAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // ==========================================
    // === CUSTOMERS VIEW (X = 0.0) ===
    // ==========================================

    let window = window_query.single().unwrap();

    // Background Tavern Image
    commands.spawn((
        AseAnimation {
            aseprite: ui_assets.background.clone(),
            animation: Animation::default(),
        },
        Sprite {
            custom_size: Some(Vec2::new(1280., 720.)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -10.0),
    ));

    commands.spawn((
        Scroll,
        AseAnimation {
            aseprite: ui_assets.scroll.clone(),
            animation: Animation::default(),
        },
        Sprite::default(),
        Transform {
            translation: Vec3::new(-510.0, 190.0, -10.0),
            scale: Vec3::splat(0.7),
            ..default()
        },
    ));

    // Invisible Counter (Tavern)
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, -450.0, -1.0)),
        RigidBody::Static,
        Collider::rectangle(3000.0, 400.0), // Top edge is at Y = -250
    ));

    // Invisible Walls to prevent items from falling off-screen (Tavern)
    commands.spawn((
        Transform::from_translation(Vec3::new(-1000.0, 0.0, -1.0)),
        RigidBody::Static,
        Collider::rectangle(100.0, 2000.0),
    ));
    commands.spawn((
        Transform::from_translation(Vec3::new(1000.0, 0.0, -1.0)),
        RigidBody::Static,
        Collider::rectangle(100.0, 2000.0),
    ));

    let mug = commands
        .spawn((
            AseAnimation {
                aseprite: ui_assets.mug.clone(),
                animation: Animation::default(),
            },
            Transform {
                translation: Vec3::new(0.0, 0.0, 2.0),
                scale: Vec3::splat(2.0),
                ..default()
            },
            Sprite::default(),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Collider::rectangle(100.0, 150.0),
            Draggable,
            LiquidContainer {
                content: None,
                level: 0,
                max_doses: 4,
                is_glass: true,
            },
        ))
        .id();

    // On garde un LiquidVisual si jamais, mais on le rend invisible (ou on l'enlève)
    let glass_liquid = commands
        .spawn((
            Mesh2d(meshes.add(Rectangle::new(80.0, 130.0))),
            MeshMaterial2d(materials.add(Color::NONE)),
            Transform::from_translation(Vec3::new(0.0, 0.0, -0.1)),
            LiquidVisual {
                container_height: 130.0,
                max_width: 80.0,
            },
        ))
        .id();
    commands.entity(mug).add_child(glass_liquid);
}
