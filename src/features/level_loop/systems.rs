use crate::core::components::Scroll;
use crate::features::personna::components::*;
use crate::features::recipes::components::RecipesConfig;
use crate::{features::level_loop::components::*, ui::UiFont};
use avian2d::{
    collision::collider::Collider,
    dynamics::rigid_body::{LinearVelocity, LockedAxes, RigidBody},
};
use bevy::{prelude::*, text::TextBounds};
use bevy_aseprite_ultra::prelude::{Animation, AseAnimation};
use rand::seq::{IndexedRandom, IteratorRandom};
use std::time::Duration;

fn get_day_config(assets: Res<Assets<PersonnaConfig>>, day: u32) -> CurrentLevel {
    let customer_count = 3 + (day as usize);
    let customer_delay = 5u64.saturating_sub(day as u64 / 4).max(2) / 2;
    let day_duration = Duration::from_secs(customer_delay * customer_count as u64 + 3);

    let config = assets
        .iter()
        .next()
        .map(|(_, c)| c)
        .expect("Config should be loaded");
    let mut rng = rand::rng();
    let customers = config
        .personas
        .sample(&mut rng, customer_count)
        .cloned()
        .collect();
    CurrentLevel {
        customer_list: customers,
        customer_timer: Timer::new(Duration::from_secs(customer_delay), TimerMode::Once),
        level_timer: Timer::new(day_duration, TimerMode::Once),
        customer_order: "".to_string(),
    }
}

fn spawn_first_customer(
    current_level: &CurrentLevel,
    pnj: &mut CurrentPnjIndex,
    arrived_events: &mut MessageWriter<CustomerArrived>,
) {
    if current_level.customer_list.is_empty() {
        pnj.0 = 0;
        return;
    }

    let customer = &current_level.customer_list[0];
    info!("Customer {} ({}) arrived!", customer.name, customer.race);
    arrived_events.write(CustomerArrived { index: 0 });
    pnj.0 = 1;
}

pub fn init_level_loop(
    mut commands: Commands,
    assets: Res<Assets<PersonnaConfig>>,
    level_day: Option<Res<LevelDay>>,
    mut arrived_events: MessageWriter<CustomerArrived>,
) {
    let day = level_day.map(|l| l.day).unwrap_or(1);
    let current_level = get_day_config(assets, day);
    let mut pnj = CurrentPnjIndex(0);

    spawn_first_customer(&current_level, &mut pnj, &mut arrived_events);

    commands.insert_resource(LevelDay { day });
    commands.insert_resource(current_level);
    commands.insert_resource(pnj);
}

pub fn level_loop_system(
    time: Res<Time>,
    mut current_level: ResMut<CurrentLevel>,
    assets: Res<Assets<PersonnaConfig>>,
    mut level_day: ResMut<LevelDay>,
    mut pnj: ResMut<CurrentPnjIndex>,
    mut arrived_events: MessageWriter<CustomerArrived>,
    mut day_ended_events: MessageWriter<DayEnded>,
) {
    current_level.level_timer.tick(time.delta());
    let pnj_index = pnj.0;

    if current_level.level_timer.just_finished() {
        info!("Day {} finished!", level_day.day);
        day_ended_events.write(DayEnded);

        level_day.day += 1;
        *current_level = get_day_config(assets, level_day.day);
        spawn_first_customer(&current_level, &mut pnj, &mut arrived_events);
        return;
    }

    if current_level.level_timer.is_finished() {
        return;
    }

    if pnj_index < current_level.customer_list.len() {
        current_level.customer_timer.tick(time.delta());

        if current_level.customer_timer.just_finished() {
            let customer = &current_level.customer_list[pnj_index];
            info!("Customer {} ({}) arrived!", customer.name, customer.race);

            arrived_events.write(CustomerArrived { index: pnj_index });

            pnj.0 += 1;
            if pnj.0 < current_level.customer_list.len() {
                current_level.customer_timer.reset();
            }
        }
    }
}

pub fn write_customer_text(
    mut commands: Commands,
    current_level: Res<CurrentLevel>,
    mut arrived_events: MessageReader<CustomerArrived>,
    mut scrool_query: Query<(Entity, &mut Scroll)>,
    ui_font: Res<UiFont>,
) {
    for event in arrived_events.read() {
        let Some(current_pnj) = current_level.customer_list.get(event.index) else {
            continue;
        };

        for (entity, _) in &mut scrool_query {
            commands.entity(entity).despawn_children();

            let customer_text = current_pnj.greetings[0]
                .replace("{order}", &current_level.customer_order)
                .replace("{name}", &current_pnj.name);
            commands.entity(entity).with_children(|parent| {
                parent.spawn((
                    Text2d::new(customer_text),
                    ui_font.text(32.0),
                    TextColor(Color::BLACK),
                    Transform::from_xyz(0.0, 0.0, 1.0),
                    TextBounds::new_horizontal(200.0), // Bounded width, unbounded height
                ));
            });
        }
    }
}

pub fn spawn_pnj(
    mut commands: Commands,
    game_assets: Res<AssetServer>,
    current_level: Res<CurrentLevel>,
    assets: Res<Assets<PersonnaConfig>>,
    mut arrived_events: MessageReader<CustomerArrived>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for event in arrived_events.read() {
        let Some(current_pnj) = current_level.customer_list.get(event.index) else {
            continue;
        };

        let config = assets
            .iter()
            .next()
            .map(|(_, c)| c)
            .expect("Config should be loaded");

        let texture = config.texture_for(current_pnj);

        let pnj_model = commands
            .spawn((
                Pnj,
                AseAnimation {
                    aseprite: game_assets.load(format!("models/personna/{}", texture)),
                    animation: Animation::default(),
                },
                Sprite::default(),
                Transform {
                    translation: Vec3::new(100.0, -70.0, 0.0),
                    scale: Vec3::splat(3.5),
                    ..default()
                },
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
}

pub fn select_recipe(
    mut current_level: ResMut<CurrentLevel>,
    mut arrived_events: MessageReader<CustomerArrived>,
    assets: Res<Assets<RecipesConfig>>,
) {
    for _ in arrived_events.read() {
        let config = assets
            .iter()
            .next()
            .map(|(_, c)| c)
            .expect("Config should be loaded");
        let mut rng = rand::rng();

        let recipe_id = config.recipes.values().choose(&mut rng).unwrap();
        let recipe = config.result_types.get(recipe_id).unwrap().name.clone();
        current_level.customer_order = recipe.to_string();
    }
}

pub fn despawn_pnj(
    mut commands: Commands,
    mut day_ended_events: MessageReader<CustomerArrived>,
    pnj_query: Query<Entity, With<Pnj>>,
) {
    if day_ended_events.read().next().is_none() {
        return;
    }

    for entity in &pnj_query {
        start_pnj_leaving(&mut commands, entity);
    }
}

pub fn start_pnj_leaving(commands: &mut Commands, entity: Entity) {
    let leave = Leaving::default();
    commands
        .entity(entity)
        .insert(leave)
        .insert(LinearVelocity(Vec2::new(-leave.speed, 0.0)));
}

pub fn pnj_departure_system(
    mut commands: Commands,
    time: Res<Time>,
    mut pnj_query: Query<(Entity, &mut Transform, &Leaving)>,
) {
    for (entity, mut transform, leaving) in &mut pnj_query {
        transform.translation.x -= leaving.speed * time.delta_secs();

        if transform.translation.x < -680.0 {
            commands.entity(entity).despawn();
        }
    }
}
