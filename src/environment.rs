use crate::alchemy::{LiquidContainer, LiquidVisual};
use crate::core::states::GameState;
use crate::interaction::Draggable;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_environment);
    }
}

fn setup_environment(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // ==========================================
    // === CUSTOMERS VIEW (X = 0.0) ===
    // ==========================================

    // Background Tavern Image
    commands.spawn((
        Sprite::from_image(asset_server.load("textures/background/tavern.png")),
        Transform::from_translation(Vec3::new(0.0, 0.0, -10.0)),
    ));

    // Foreground Tavern Image (assuming it should be spawned)
    commands.spawn((
        Sprite::from_image(asset_server.load("textures/foreground/tavern.png")),
        Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)), // Higher Z to appear in front
    ));

    // Invisible Counter (Tavern)
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, -450.0, -1.0)),
        RigidBody::Static,
        Collider::rectangle(3000.0, 400.0), // Top edge is at Y = -250
    ));

    // Invisible Walls to prevent items from falling off-screen (Tavern)
    commands.spawn((
        // Left wall
        Transform::from_translation(Vec3::new(-1000.0, 0.0, -1.0)),
        RigidBody::Static,
        Collider::rectangle(100.0, 2000.0),
    ));
    commands.spawn((
        // Right wall
        Transform::from_translation(Vec3::new(1000.0, 0.0, -1.0)),
        RigidBody::Static,
        Collider::rectangle(100.0, 2000.0),
    ));

    // A Glass on the counter
    let bottle_entity = commands
        .spawn((
            AseAnimation {
                aseprite: asset_server.load("textures/bottle/bottle_glouglou.aseprite"),
                animation: Animation::default(),
            },
            Transform {
                translation: Vec3::new(0.0, -150.0, 2.0),
                scale: Vec3::splat(2.5),
                ..default()
            },
            Sprite::default(),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Collider::rectangle(100.0, 150.0),
            Draggable,
            LiquidContainer {
                doses: 0,
                max_doses: 4,
                color: Color::NONE,
                is_glass: true,
            },
        ))
        .id();

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
    commands.entity(bottle_entity).add_child(glass_liquid);

    // ==========================================
    // === ALCHEMY VIEW (X = 5000.0) ===
    // ==========================================

    // Background Alchemy Image
    commands.spawn((
        Sprite::from_image(asset_server.load("textures/background/alchemy.png")),
        Transform::from_translation(Vec3::new(5000.0, 0.0, -10.0)),
    ));

    // Invisible Alchemy Table
    commands.spawn((
        Transform::from_translation(Vec3::new(5000.0, -450.0, -1.0)),
        RigidBody::Static,
        Collider::rectangle(3000.0, 400.0), // Top edge is at Y = -250
    ));

    // Invisible Walls (Alchemy)
    commands.spawn((
        // Left wall
        Transform::from_translation(Vec3::new(4000.0, 0.0, -1.0)),
        RigidBody::Static,
        Collider::rectangle(100.0, 2000.0),
    ));
    commands.spawn((
        // Right wall
        Transform::from_translation(Vec3::new(6000.0, 0.0, -1.0)),
        RigidBody::Static,
        Collider::rectangle(100.0, 2000.0),
    ));

    // Bottle 1 (Rouge)
    let bottle_1 = commands
        .spawn((
            Sprite {
                image: asset_server.load("textures/bottle/bottle_1.png"),
                custom_size: Some(Vec2::new(80.0, 180.0)),
                ..default()
            },
            Transform::from_translation(Vec3::new(4800.0, 100.0, 2.0)), // Bien espacées
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED, // Empêche de basculer
            Collider::rectangle(80.0, 180.0),
            Draggable,
            LiquidContainer {
                doses: 5,
                max_doses: 5,
                color: Color::srgb(0.9, 0.1, 0.1),
                is_glass: false,
            },
        ))
        .id();

    let liquid_1 = commands
        .spawn((
            Mesh2d(meshes.add(Rectangle::new(60.0, 160.0))),
            MeshMaterial2d(materials.add(Color::srgb(0.9, 0.1, 0.1))),
            Transform::from_translation(Vec3::new(0.0, 0.0, -0.1)),
            LiquidVisual {
                container_height: 160.0,
                max_width: 60.0,
            },
        ))
        .id();
    commands.entity(bottle_1).add_child(liquid_1);

    // Bottle 2 (Bleue)
    let bottle_2 = commands
        .spawn((
            Sprite {
                image: asset_server.load("textures/bottle/bottle_2.png"),
                custom_size: Some(Vec2::new(80.0, 180.0)),
                ..default()
            },
            Transform::from_translation(Vec3::new(5000.0, 100.0, 2.0)),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Collider::rectangle(80.0, 180.0),
            Draggable,
            LiquidContainer {
                doses: 5,
                max_doses: 5,
                color: Color::srgb(0.1, 0.1, 0.9),
                is_glass: false,
            },
        ))
        .id();

    let liquid_2 = commands
        .spawn((
            Mesh2d(meshes.add(Rectangle::new(60.0, 160.0))),
            MeshMaterial2d(materials.add(Color::srgb(0.1, 0.1, 0.9))),
            Transform::from_translation(Vec3::new(0.0, 0.0, -0.1)),
            LiquidVisual {
                container_height: 160.0,
                max_width: 60.0,
            },
        ))
        .id();
    commands.entity(bottle_2).add_child(liquid_2);

    // Bottle 3 (Verte)
    let bottle_3 = commands
        .spawn((
            Sprite {
                image: asset_server.load("textures/bottle/bottle_3.png"),
                custom_size: Some(Vec2::new(80.0, 180.0)),
                ..default()
            },
            Transform::from_translation(Vec3::new(5200.0, 100.0, 2.0)),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Collider::rectangle(80.0, 180.0),
            Draggable,
            LiquidContainer {
                doses: 5,
                max_doses: 5,
                color: Color::srgb(0.1, 0.9, 0.1),
                is_glass: false,
            },
        ))
        .id();

    let liquid_3 = commands
        .spawn((
            Mesh2d(meshes.add(Rectangle::new(60.0, 160.0))),
            MeshMaterial2d(materials.add(Color::srgb(0.1, 0.9, 0.1))),
            Transform::from_translation(Vec3::new(0.0, 0.0, -0.1)),
            LiquidVisual {
                container_height: 160.0,
                max_width: 60.0,
            },
        ))
        .id();
    commands.entity(bottle_3).add_child(liquid_3);
}
