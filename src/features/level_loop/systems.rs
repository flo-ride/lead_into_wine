use crate::core::components::Scroll;
use crate::core::states::GameState;
use crate::environment::{LivesText, PlayerLives};
use crate::features::personna::components::*;
use crate::features::recipes::components::RecipesConfig;
use crate::{features::level_loop::components::*, ui::UiFont};
use avian2d::{collision::collider::Collider, dynamics::rigid_body::LinearVelocity};
use bevy::{prelude::*, text::TextBounds};
use bevy_aseprite_ultra::prelude::{Animation, AseAnimation};
use rand::seq::{IndexedRandom, IteratorRandom};
use std::time::Duration;

const WAIT_INDICATOR_RADIUS: f32 = 6.0;

/// Marqueur apposé sur toutes les entités générées pendant le gameplay
#[derive(Component)]
pub struct LevelEntity;

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

#[derive(Component)]
pub struct DayTransitionText {
    pub timer: Timer,
}

pub fn handle_day_transition_cooldown(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DayTransitionText)>,
    current_level: Res<CurrentLevel>,
    mut pnj: ResMut<CurrentPnjIndex>,
    mut arrived_events: MessageWriter<CustomerArrived>,
) {
    for (entity, mut transition) in &mut query {
        transition.timer.tick(time.delta());

        if transition.timer.just_finished() {
            // Le cooldown de 5s est fini, on détruit le texte
            commands.entity(entity).despawn();

            // On fait spawner le premier client du nouveau jour
            spawn_first_customer(&current_level, &mut pnj, &mut arrived_events);
        }
    }
}

fn get_day_config(assets: Res<Assets<PersonnaConfig>>, day: u32) -> CurrentLevel {
    const CUSTOMER_DELAY_START_SECS: u64 = 3; // jour 1 : large marge, le temps d'apprendre
    const CUSTOMER_DELAY_FLOOR_SECS: u64 = 4; // plancher : légèrement sous le temps d'un joueur rapide (20s), pour forcer la pression sans devenir impossible
    const CUSTOMER_DELAY_DECAY_PER_DAY: u64 = 2; // le délai baisse de 2s par jour
    const DAY_BUFFER_SECS: u64 = 0; // marge en fin de journée pour finir le dernier client

    let customer_count = 1 + (day as usize);
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
    mut text_query: Query<(&mut Text2d, &mut PnjWaitText)>,
) {
    for (mut transform, mut indicator) in &mut query {
        indicator.timer.tick(time.delta());
        let remaining_ratio = 1.0 - indicator.timer.fraction();
        transform.scale = Vec3::splat(remaining_ratio.max(0.0));
    }
    for (mut text, mut wait_text) in &mut text_query {
        wait_text.timer.tick(time.delta());
        text.0 = format!("{:.0}", wait_text.timer.remaining_secs().ceil());
    }
}

pub fn hide_wait_indicator_on_leaving(
    mut commands: Commands,
    leaving_pnj: Query<Entity, Added<Leaving>>,
    children_query: Query<&Children>,
    indicator_query: Query<Entity, Or<(With<PnjWaitIndicator>, With<PnjWaitText>)>>,
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
    ui_font: Res<UiFont>,
) {
    let day = level_day.map(|l| l.day).unwrap_or(1);
    let current_level = get_day_config(assets, day);
    let mut pnj = CurrentPnjIndex(0);

    commands.spawn((
        Text2d::new(format!("Day {}", day)),
        ui_font.text(64.0),
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, 0.0, 15.0),
        DayTransitionText {
            timer: Timer::from_seconds(2.0, TimerMode::Once),
        },
    ));

    commands.insert_resource(LevelDay { day });
    commands.insert_resource(current_level);
    commands.insert_resource(pnj);
}

pub fn level_loop_system(
    mut commands: Commands,
    time: Res<Time>,
    mut current_level: ResMut<CurrentLevel>,
    assets: Res<Assets<PersonnaConfig>>,
    mut level_day: ResMut<LevelDay>,
    mut pnj: ResMut<CurrentPnjIndex>,
    ui_font: Res<UiFont>,
    mut arrived_events: MessageWriter<CustomerArrived>,
    mut day_ended_events: MessageWriter<DayEnded>,
    transition_query: Query<&DayTransitionText>,
    pnj_query: Query<Entity, With<Pnj>>,
) {
    // Si le texte "Day X" est affiché, la boucle du niveau est en pause
    if !transition_query.is_empty() {
        return;
    }

    current_level.level_timer.tick(time.delta());

    // 1. Le temps de la journée vient d'expirer : on signale la fin du jour
    if current_level.level_timer.just_finished() {
        info!("Day {} finished!", level_day.day);
        day_ended_events.write(DayEnded); // déclenche despawn_pnj (le PNJ commence à marcher vers la sortie)
        return;
    }

    // 2. Le temps est fini MAIS on attend que le PNJ ait totalement quitté la scène (despawn)
    if current_level.level_timer.is_finished() {
        // Tant qu'il y a au moins un PNJ à l'écran, on ne fait rien
        if !pnj_query.is_empty() {
            return;
        }

        // C'EST BON : Plus aucun PNJ à l'écran ! On peut lancer le Jour suivant.
        level_day.day += 1;
        *current_level = get_day_config(assets, level_day.day);
        pnj.0 = 0; // Réinitialise l'index des PNJ

        // Spawn du texte "Day X" (cooldown 5s)
        commands.spawn((
            Text2d::new(format!("Day {}", level_day.day)),
            ui_font.text(64.0),
            TextColor(Color::WHITE),
            Transform::from_xyz(0.0, 0.0, 15.0),
            DayTransitionText {
                timer: Timer::from_seconds(3.0, TimerMode::Once),
            },
        ));

        return;
    }

    // 3. Gestion du spawn des clients durant la journée (clients 2, 3, etc.)
    let pnj_index = pnj.0;
    if pnj_index > 0 && pnj_index < current_level.customer_list.len() {
        current_level.customer_timer.tick(time.delta());

        if current_level.customer_timer.is_finished() {
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
    ui_font: Res<UiFont>,
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

        let wait_text = commands
            .spawn((
                PnjWaitText {
                    timer: Timer::new(current_level.customer_timer.duration(), TimerMode::Once),
                },
                Text2d::new(format!(
                    "{:.0}",
                    current_level.customer_timer.duration().as_secs_f32()
                )),
                ui_font.text(60.0),
                TextColor(Color::WHITE),
                Transform {
                    translation: Vec3::new(offset_x, offset_y - WAIT_INDICATOR_RADIUS - 15.0, 0.6),
                    scale: Vec3::splat(0.4), // Counter-scale the parent's 2.5 scale (1.0 / 2.5) to keep it crisp
                    ..default()
                },
            ))
            .id();

        commands.entity(pnj_model).add_child(pnj_hitbox);
        commands.entity(pnj_model).add_child(wait_indicator);
        commands.entity(pnj_model).add_child(wait_text);
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
#[derive(Component)]
pub struct PnjLeaving;

pub fn despawn_pnj(
    mut commands: Commands,
    mut customer_ended_events: MessageReader<CustomerArrived>,
    mut day_ended_events: MessageReader<DayEnded>,
    // Exclut les PNJ qui ont DÉJÀ reçu l'ordre de partir
    pnj_query: Query<Entity, (With<Pnj>, Without<PnjLeaving>)>,
    mut lives: ResMut<PlayerLives>,
    mut lives_ui_query: Query<&mut Text, With<LivesText>>,
) {
    // Vider les lecteurs d'événements et vérifier si au moins un s'est déclenché
    let has_day_ended = day_ended_events.read().next().is_some();
    let has_customer_left = customer_ended_events.read().next().is_some();

    if !has_day_ended && !has_customer_left {
        return;
    }

    for entity in &pnj_query {
        // 1. Appliquer le marqueur pour éviter qu'il ne re-passe ici dans le même frame ou le suivant
        commands.entity(entity).insert(PnjLeaving);

        // 2. Décrémenter les vies
        lives.count = lives.count.saturating_sub(1);
        warn!(
            "Un client est parti sans être servi ! Vies restantes : {}",
            lives.count
        );

        // 3. Mettre à jour l'UI
        if let Ok(mut text) = lives_ui_query.single_mut() {
            let full = " ".repeat(lives.count as usize);
            let empty = "|".repeat((3 - lives.count) as usize);
            **text = format!("{}{}", full, empty)
        }

        // 4. Lancer l'animation/logique de départ
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

pub fn check_game_over(lives: Res<PlayerLives>, mut next_state: ResMut<NextState<GameState>>) {
    if lives.count == 0 {
        next_state.set(GameState::GameOver);
    }
}
