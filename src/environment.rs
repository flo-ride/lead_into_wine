use std::f32::consts::PI;

use crate::alchemy::{LiquidContainer, LiquidVisual};
use crate::core::states::GameState;
use crate::features::level_loop::components::DayEnded;
use crate::features::level_loop::systems::LevelEntity;
use crate::interaction::Draggable;
use crate::physics::GameLayer;
use crate::ui::UiFont;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_asset_loader::prelude::*;
use rand::RngExt;

pub struct EnvironmentPlugin;

/// Ressource pour suivre le nombre de vies restantes
#[derive(Resource)]
pub struct PlayerLives {
    pub count: u8,
}

impl Default for PlayerLives {
    fn default() -> Self {
        Self { count: 3 }
    }
}

/// Marqueur pour le composant UI qui affiche le texte/icônes des vies
#[derive(Component)]
pub struct LivesText;
impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_environment)
            .init_resource::<PlayerLives>()
            .add_systems(
                Update,
                spawn_additional_bottles_on_new_day.run_if(in_state(GameState::Playing)),
            )
            .add_loading_state(
                LoadingState::new(GameState::Loading)
                    .continue_to_state(GameState::initial())
                    .load_collection::<UiAssets>(),
            );
    }
}

/// Marqueur pour identifier les bouteilles
#[derive(Component)]
pub struct ShelfBottle;

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

    #[asset(path = "models/bottles/cider.ase")]
    pub cider_bottle: Handle<Aseprite>,

    #[asset(path = "models/bottles/beer.ase")]
    pub beer_bottle: Handle<Aseprite>,
}

/// Fonction utilitaire pour instancier une nouvelle bouteille
fn spawn_bottle(
    commands: &mut Commands,
    aseprite: Handle<Aseprite>,
    content_name: &str,
    x_pos: f32,
    level: usize,
) {
    commands.spawn((
        ShelfBottle,
        LevelEntity,
        AseAnimation {
            aseprite,
            animation: Animation::tag("Full"),
        },
        Transform {
            translation: Vec3::new(x_pos, 160.0, 3.0),
            scale: Vec3::splat(1.7),
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
            content: Some(content_name.to_string()),
            level,
            max_doses: 10,
            is_glass: false,
        },
    ));
}

/// Trouve une position X sur l'étagère en évitant les bouteilles déjà présentes
fn find_free_x_position(existing_x_positions: &[f32]) -> f32 {
    let mut rng = rand::rng();
    let min_x = 700.0;
    let max_x = 1180.0;
    let min_distance = 80.0; // Espacement minimal entre 2 bouteilles (largeur du collider + marge)

    for _ in 0..50 {
        let candidate_x = rng.random_range(min_x..=max_x);
        let is_valid = existing_x_positions
            .iter()
            .all(|&x| (x - candidate_x).abs() >= min_distance);

        if is_valid {
            return candidate_x;
        }
    }

    // Si l'étagère est trop pleine, fallback sur une position aléatoire simple
    rng.random_range(min_x..=max_x)
}

/// Génère 3 nouvelles bouteilles sans toucher aux anciennes
pub fn spawn_additional_bottles_on_new_day(
    mut commands: Commands,
    mut day_ended_events: MessageReader<DayEnded>,
    existing_bottles: Query<&Transform, With<ShelfBottle>>,
    ui_assets: Res<UiAssets>,
) {
    if day_ended_events.read().next().is_none() {
        return;
    }

    let mut rng = rand::rng();

    // On récupère la position X de toutes les bouteilles actuellement sur l'étagère (Y proche de 160.0)
    let mut shelf_x_positions: Vec<f32> = existing_bottles
        .iter()
        .filter(|transform| (transform.translation.y - 160.0).abs() < 50.0)
        .map(|transform| transform.translation.x)
        .collect();

    let bottle_types = [
        (ui_assets.milk_bottle.clone(), "milk"),
        (ui_assets.wine_bottle.clone(), "wine"),
        (ui_assets.unicorn_tears_bottle.clone(), "unicorn_tear"),
        (ui_assets.cider_bottle.clone(), "cider"),
        (ui_assets.beer_bottle.clone(), "beer"),
    ];

    for (aseprite, content) in bottle_types {
        let x_pos = find_free_x_position(&shelf_x_positions);
        shelf_x_positions.push(x_pos); // On ajoute la nouvelle position pour que la bouteille suivante en tienne compte

        let random_level = rng.random_range(5..=10);
        spawn_bottle(&mut commands, aseprite, content, x_pos, random_level);
    }
}

fn setup_environment(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut lives: ResMut<PlayerLives>,
    ui_font: Res<UiFont>,
) {
    // ==========================================
    // === CUSTOMERS VIEW (X = 0.0) ===
    // ==========================================
    lives.count = 3;
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
        Collider::rectangle(3000.0, 400.0),
        CollisionLayers::new(
            [GameLayer::Environment],
            [GameLayer::TavernItem, GameLayer::ShelfItem],
        ),
    ));

    // Invisible Walls
    commands.spawn((
        Transform::from_translation(Vec3::new(-650.0, 0.0, -1.0)),
        RigidBody::Static,
        Collider::rectangle(100.0, 2000.0),
        CollisionLayers::new([GameLayer::Environment], [GameLayer::TavernItem]),
    ));
    commands.spawn((
        Transform::from_translation(Vec3::new(650.0, 0.0, -1.0)),
        RigidBody::Static,
        Collider::rectangle(100.0, 2000.0),
        CollisionLayers::new([GameLayer::Environment], [GameLayer::TavernItem]),
    ));

    let mug = commands
        .spawn((
            LevelEntity,
            AseAnimation {
                aseprite: ui_assets.mug.clone(),
                animation: Animation::default(),
            },
            Transform {
                translation: Vec3::new(0.0, -175.0, 10.0),
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

    // Spawn de l'UI des vies (en bas au milieu)
    //
    let full = " ".repeat(lives.count as usize);
    let empty = "|".repeat((3 - lives.count) as usize);

    commands.spawn((
        LevelEntity,
        LivesText,
        Text::new(format!("{}{}", full, empty)),
        ui_font.text(56.0),
        // Gris pierre sombre / ardoise
        TextColor(Color::srgb_u8(74, 77, 80)),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(30.0),    // À 20px du bas de l'écran
            left: Val::Percent(80.0), // Positionné au centre horizontal (50%)
            ..default()
        },
    ));

    // Trash bin
    commands.spawn((
        Transform {
            translation: Vec3::new(570.0, -280.0, 10.0),
            ..default()
        },
        #[cfg(feature = "dev")]
        Sprite {
            color: Color::srgba(1.0, 0.2, 0.2, 0.5),
            custom_size: Some(Vec2::new(150.0, 180.0)),
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

    // Spawn initial des 5 bouteilles au démarrage du jeu
    let initial_positions = [765.0, 865.0, 965.0, 1065.0, 1165.0];
    let initial_bottles = [
        (ui_assets.milk_bottle.clone(), "milk"),
        (ui_assets.wine_bottle.clone(), "wine"),
        (ui_assets.unicorn_tears_bottle.clone(), "unicorn_tear"),
        (ui_assets.cider_bottle.clone(), "cider"),
        (ui_assets.beer_bottle.clone(), "beer"),
    ];

    for (i, (aseprite, content)) in initial_bottles.into_iter().enumerate() {
        spawn_bottle(&mut commands, aseprite, content, initial_positions[i], 10);
    }
}
