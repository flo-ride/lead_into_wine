use crate::alchemy::{LiquidContainer, LiquidVisual};

use crate::core::states::GameState;
use crate::interaction::Draggable;
use crate::physics::GameLayer;
use avian2d::prelude::*;
use bevy::prelude::*;
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
    #[asset(path = "textures/background/tavern_background.aseprite")]
    pub background: Handle<Aseprite>,

    #[asset(path = "textures/background/tavern_foreground.aseprite")]
    pub foreground: Handle<Aseprite>,

    #[asset(path = "models/scroll.aseprite")]
    pub scroll: Handle<Aseprite>,

    #[asset(path = "models/mug/cut_mug.ase")]
    pub mug: Handle<Aseprite>,

    #[asset(path = "models/bottles/wine.ase")]
    pub wine_bottle: Handle<Aseprite>,

    #[asset(path = "models/bottles/milk.ase")]
    pub milk_bottle: Handle<Aseprite>,

    #[asset(path = "models/bottles/unicorn_tears.ase")]
    pub unicorn_tears_bottle: Handle<Aseprite>,
}

fn setup_environment(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // ==========================================
    // === CUSTOMERS VIEW (X = 0.0) ===
    // ==========================================

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
        AseAnimation {
            aseprite: ui_assets.foreground.clone(),
            animation: Animation::default(),
        },
        Sprite {
            custom_size: Some(Vec2::new(1280., 720.)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -1.0),
    ));

    // Invisible Counter (Tavern)
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, -450.0, -1.0)),
        RigidBody::Static,
        Collider::rectangle(3000.0, 400.0), // Top edge is at Y = -250
        CollisionLayers::new(
            [GameLayer::Environment],
            [GameLayer::TavernItem, GameLayer::ShelfItem],
        ),
    ));

    // Invisible Walls to prevent items from falling off-screen (Tavern)
    // Left wall
    commands.spawn((
        Transform::from_translation(Vec3::new(-650.0, 0.0, -1.0)),
        RigidBody::Static,
        Collider::rectangle(100.0, 2000.0),
        CollisionLayers::new([GameLayer::Environment], [GameLayer::TavernItem]),
    ));
    // Right wall
    commands.spawn((
        Transform::from_translation(Vec3::new(650.0, 0.0, -1.0)),
        RigidBody::Static,
        Collider::rectangle(100.0, 2000.0),
        CollisionLayers::new([GameLayer::Environment], [GameLayer::TavernItem]),
    ));

    let mug = commands
        .spawn((
            AseAnimation {
                aseprite: ui_assets.mug.clone(),
                animation: Animation::default(),
            },
            Transform {
                translation: Vec3::new(0.0, -175.0, 1.0),
                scale: Vec3::splat(2.0),
                ..default()
            },
            Sprite::default(),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Collider::rectangle(100.0, 150.0),
            CollisionLayers::new(
                [GameLayer::TavernItem],
                [GameLayer::Environment, GameLayer::TavernItem],
            ),
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

    // Wine Bottle
    commands.spawn((
        AseAnimation {
            aseprite: ui_assets.wine_bottle.clone(),
            animation: Animation::tag("Full"),
        },
        Transform {
            translation: Vec3::new(965.0, 160.0, 3.0),
            scale: Vec3::splat(2.0),
            ..default()
        },
        Sprite::default(),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Collider::rectangle(60.0, 180.0), // Approximated collider size for a bottle
        CollisionLayers::new(
            [GameLayer::ShelfItem],
            [
                GameLayer::Environment,
                GameLayer::ShelfItem,
                GameLayer::ShelfStructure,
            ],
        ),
        Draggable,
        LiquidContainer {
            content: Some("wine".to_string()),
            level: 5,
            max_doses: 5,
            is_glass: false,
        },
    ));

    commands.spawn((
        Transform {
            translation: Vec3::new(570.0, -280.0, 10.0),
            ..default()
        },
        #[cfg(feature = "dev")]
        Sprite {
            color: Color::srgba(1.0, 0.2, 0.2, 0.5),
            custom_size: Some(Vec2::new(150.0, 180.0)), // pour visualiser la vraie taille du collider
            ..default()
        },
        #[cfg(not(feature = "dev"))]
        Sprite {
            color: Color::NONE,
            custom_size: Some(Vec2::new(100.0, 180.0)),
            ..default()
        },
        Collider::rectangle(150.0, 180.0),
        LiquidContainer {
            content: Some("bin".to_string()),
            level: 0,
            max_doses: usize::MAX,
            is_glass: true,
        },
    ));
    // Milk Bottle
    commands.spawn((
        AseAnimation {
            aseprite: ui_assets.milk_bottle.clone(),
            animation: Animation::tag("Full"),
        },
        Transform {
            translation: Vec3::new(765.0, 160.0, 3.0), // À gauche du vin
            scale: Vec3::splat(2.0),
            ..default()
        },
        Sprite::default(),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Collider::rectangle(60.0, 180.0),
        CollisionLayers::new(
            [GameLayer::ShelfItem],
            [
                GameLayer::Environment,
                GameLayer::ShelfItem,
                GameLayer::ShelfStructure,
            ],
        ),
        Draggable,
        LiquidContainer {
            content: Some("milk".to_string()),
            level: 5,
            max_doses: 5,
            is_glass: false,
        },
    ));

    // Unicorn Tears Bottle
    commands.spawn((
        AseAnimation {
            aseprite: ui_assets.unicorn_tears_bottle.clone(),
            animation: Animation::tag("Full"),
        },
        Transform {
            translation: Vec3::new(1165.0, 160.0, 3.0), // À droite du vin
            scale: Vec3::splat(2.0),
            ..default()
        },
        Sprite::default(),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Collider::rectangle(60.0, 180.0),
        CollisionLayers::new(
            [GameLayer::ShelfItem],
            [
                GameLayer::Environment,
                GameLayer::ShelfItem,
                GameLayer::ShelfStructure,
            ],
        ),
        Draggable,
        LiquidContainer {
            content: Some("unicorn_tear".to_string()),
            level: 5,
            max_doses: 5,
            is_glass: false,
        },
    ));
}
