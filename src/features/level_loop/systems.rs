use crate::features::personna::components::PersonnaConfig;
use avian2d::{
    collision::collider::Collider,
    dynamics::rigid_body::{LockedAxes, RigidBody},
};
use bevy::prelude::*;
use rand::seq::IndexedRandom;
use std::time::Duration;

#[derive(Resource, Default)]
pub struct LevelDay {
    pub day: u32,
}

#[derive(Resource)]
pub struct CurrentLevel {
    pub customer_list: Vec<String>,
    pub pnj_index: usize,
    pub customer_timer: Timer,
    pub level_timer: Timer,
}

fn get_day_config(assets: Res<Assets<PersonnaConfig>>, day: u32) -> CurrentLevel {
    let customer_count = 3 + (day as usize);
    let customer_delay = 5u64.saturating_sub(day as u64 / 2).max(2);
    let day_duration = Duration::from_secs(customer_delay * customer_count as u64);

    let config = assets
        .iter()
        .next()
        .map(|(_, c)| c)
        .expect("Config should be loaded");
    let names: Vec<String> = config.personas.keys().cloned().collect();
    let mut rng = rand::rng();

    let customers = names.sample(&mut rng, customer_count).cloned().collect();
    CurrentLevel {
        customer_list: customers,
        pnj_index: 0,
        customer_timer: Timer::new(Duration::from_secs(customer_delay), TimerMode::Once),
        level_timer: Timer::new(day_duration, TimerMode::Once),
    }
}

pub fn init_level_loop(
    mut commands: Commands,
    assets: Res<Assets<PersonnaConfig>>,
    level_day: Option<Res<LevelDay>>,
) {
    let day = level_day.map(|l| l.day).unwrap_or(1); // Start at level 1
    let current_level = get_day_config(assets, day);

    commands.insert_resource(LevelDay { day });
    commands.insert_resource(current_level);
}

pub fn level_loop_system(
    time: Res<Time>,
    mut current_level: ResMut<CurrentLevel>,
    assets: Res<Assets<PersonnaConfig>>,
    mut level_day: ResMut<LevelDay>,
) {
    current_level.level_timer.tick(time.delta());

    if current_level.level_timer.just_finished() {
        info!("Day {} finished!", level_day.day);
        level_day.day += 1;
        *current_level = get_day_config(assets, level_day.day);
        return;
    }

    if current_level.level_timer.is_finished() {
        return;
    }

    if current_level.pnj_index < current_level.customer_list.len() {
        current_level.customer_timer.tick(time.delta());

        if current_level.customer_timer.just_finished() {
            let pnj_name = &current_level.customer_list[current_level.pnj_index];
            info!("Customer {} arrived!", pnj_name);

            current_level.pnj_index += 1;
            if current_level.pnj_index < current_level.customer_list.len() {
                current_level.customer_timer.reset();
            }
        }
    }
}

pub fn spawn_pnj(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let pnj_model = commands
        .spawn((
            Sprite {
                image: asset_server.load("models/personna/knight.png"),
                custom_size: Some(Vec2::new(80.0, 180.0)),
                ..default()
            },
            Transform {
                translation: Vec3::new(100.0, 100.0, 0.0),
                scale: Vec3::splat(2.5),
                ..default()
            },
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Collider::rectangle(80.0, 180.0),
        ))
        .id();

    let pnj_hitbox = commands
        .spawn((
            Mesh2d(meshes.add(Rectangle::new(60.0, 160.0))),
            MeshMaterial2d(materials.add(Color::NONE)),
            Transform::from_translation(Vec3::new(0.0, 0.0, -0.1)),
        ))
        .id();

    commands.entity(pnj_model).add_child(pnj_hitbox);
}
