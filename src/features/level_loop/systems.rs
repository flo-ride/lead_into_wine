use crate::core::components::Scroll;
use crate::features::personna::components::*;
use crate::features::recipes::components::RecipesConfig;
use crate::{features::level_loop::components::*, ui::UiFont};
use avian2d::{collision::collider::Collider, dynamics::rigid_body::LinearVelocity};
use bevy::{prelude::*, text::TextBounds};
use bevy_aseprite_ultra::prelude::{Animation, AseAnimation};
use rand::seq::{IndexedRandom, IteratorRandom};
use std::time::Duration;

const WAIT_INDICATOR_RADIUS: f32 = 6.0;

fn wait_indicator_offset(texture: &str) -> (f32, f32) {
    match texture {
        "orc.aseprite" => (0.0, 70.0),
        "goblin.aseprite" => (0.0, 70.0),
        "wizard.aseprite" => (0.0, 60.0),
        "warrior.aseprite" => (-5.0, 60.0),
        _ => (0.0, 50.0),
    }
}

const PNJ_ARRIVAL_COOLDOWN_SECS: f32 = 0.6;

/// Marqueur temporaire : un PNJ "en attente" avant son apparition réelle.
#[derive(Component)]
pub struct PendingPnjSpawn {
    pub index: usize,
    pub timer: Timer,
}

fn get_day_config(assets: Res<Assets<PersonnaConfig>>, day: u32) -> CurrentLevel {
    const CUSTOMER_DELAY_START_SECS: u64 = 35; // jour 1 : large marge, le temps d'apprendre
    const CUSTOMER_DELAY_FLOOR_SECS: u64 = 12; // plancher : légèrement sous le temps d'un joueur rapide (20s), pour forcer la pression sans devenir impossible
    const CUSTOMER_DELAY_DECAY_PER_DAY: u64 = 3; // le délai baisse de 2s par jour
    const DAY_BUFFER_SECS: u64 = 10; // marge en fin de journée pour finir le dernier client

    let customer_count = 3 + (day as usize);
    let customer_delay = CUSTOMER_DELAY_START_SECS
        .saturating_sub(day as u64 * CUSTOMER_DELAY_DECAY_PER_DAY)
        .max(CUSTOMER_DELAY_FLOOR_SECS);
    let day_duration =
        Duration::from_secs(customer_delay * customer_count as u64 + DAY_BUFFER_SECS);

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

pub fn animate_pnj_wait_indicator(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut PnjWaitIndicator)>,
) {
    for (mut transform, mut indicator) in &mut query {
        indicator.timer.tick(time.delta());
        let remaining_ratio = 1.0 - indicator.timer.fraction();
        transform.scale = Vec3::splat(remaining_ratio.max(0.0));
    }
}

pub fn hide_wait_indicator_on_leaving(
    mut commands: Commands,
    leaving_pnj: Query<Entity, Added<Leaving>>,
    children_query: Query<&Children>,
    indicator_query: Query<Entity, With<PnjWaitIndicator>>,
) {
    for pnj_entity in &leaving_pnj {
        let Ok(children) = children_query.get(pnj_entity) else {
            continue;
        };
        for &child in children {
            if indicator_query.contains(child) {
                commands.entity(child).despawn();
            }
        }
    }
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

pub fn queue_pnj_spawn(mut commands: Commands, mut arrived_events: MessageReader<CustomerArrived>) {
    for event in arrived_events.read() {
        commands.spawn(PendingPnjSpawn {
            index: event.index,
            timer: Timer::from_seconds(PNJ_ARRIVAL_COOLDOWN_SECS, TimerMode::Once),
        });
    }
}

/// Fait avancer le cooldown, et spawn réellement le PNJ une fois le délai écoulé.
pub fn spawn_pnj(
    mut commands: Commands,
    time: Res<Time>,
    game_assets: Res<AssetServer>,
    current_level: Res<CurrentLevel>,
    assets: Res<Assets<PersonnaConfig>>,
    mut pending: Query<(Entity, &mut PendingPnjSpawn)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (pending_entity, mut pending_spawn) in &mut pending {
        pending_spawn.timer.tick(time.delta());

        if !pending_spawn.timer.is_finished() {
            continue;
        }

        commands.entity(pending_entity).despawn();

        let Some(current_pnj) = current_level.customer_list.get(pending_spawn.index) else {
            continue;
        };

        let config = assets
            .iter()
            .next()
            .map(|(_, c)| c)
            .expect("Config should be loaded");

        let texture = config.texture_for(current_pnj);
        let (offset_x, offset_y) = wait_indicator_offset(texture);

        let pnj_model = commands
            .spawn((
                Pnj,
                AseAnimation {
                    aseprite: game_assets.load(format!("models/personna/{}", texture)),
                    animation: Animation::default(),
                },
                Sprite::default(),
                Transform {
                    translation: Vec3::new(100.0, -70.0, -2.0),
                    scale: Vec3::splat(2.5),
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

        let wait_indicator = commands
            .spawn((
                PnjWaitIndicator {
                    timer: Timer::new(current_level.customer_timer.duration(), TimerMode::Once),
                },
                Mesh2d(meshes.add(Circle::new(WAIT_INDICATOR_RADIUS))),
                MeshMaterial2d(materials.add(Color::srgba(0.95, 0.85, 0.65, 0.9))),
                Transform::from_translation(Vec3::new(offset_x, offset_y, 0.5)),
            ))
            .id();

        commands.entity(pnj_model).add_child(pnj_hitbox);
        commands.entity(pnj_model).add_child(wait_indicator);
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
        let recipe = config.beverages.get(recipe_id).unwrap().name.clone();
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
