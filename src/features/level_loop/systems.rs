use crate::features::personna::components::PersonnaConfig;
use bevy::prelude::*;
use rand::seq::IndexedRandom;
use std::time::Duration;

#[derive(Resource)]
pub struct LevelTimer(pub Timer);

#[derive(Resource, Clone)]
pub struct LevelDay {
    pub day: u32,
    pub customer_count: usize,
    pub day_duration: f32,
    pub customer_delay: Duration,
}

#[derive(Resource)]
pub struct CurrentLevel {
    pub customer_list: Vec<String>,
    pub pnj_index: usize,
    pub customer_timer: Timer,
}

fn get_day_config(day: u32) -> LevelDay {
    LevelDay {
        day,
        customer_count: 3 + (day as usize * 2),
        day_duration: 120.0 + (day as f32 * 20.0),
        customer_delay: Duration::from_secs(5u64.saturating_sub(day as u64 / 2).max(2)),
    }
}
pub fn init_level_loop(mut commands: Commands, assets: Res<Assets<PersonnaConfig>>) {
    let config = assets
        .iter()
        .next()
        .map(|(_, c)| c)
        .expect("Config should be loaded");

    let names: Vec<String> = config.personas.keys().cloned().collect();

    let mut rng = rand::rng();

    let level_day = get_day_config(0);

    let customers = names
        .sample(&mut rng, level_day.customer_count)
        .cloned()
        .collect();

    commands.insert_resource(level_day.clone());

    commands.insert_resource(LevelTimer(Timer::from_seconds(
        level_day.day_duration,
        TimerMode::Once,
    )));

    commands.insert_resource(CurrentLevel {
        customer_list: customers,
        pnj_index: 0,
        customer_timer: Timer::new(level_day.customer_delay, TimerMode::Once),
    });
}

pub fn level_loop_system(
    time: Res<Time>,
    mut level_state: ResMut<CurrentLevel>,
    mut level_timer: ResMut<LevelTimer>,
    _assets: Res<Assets<PersonnaConfig>>,
) {
    level_timer.0.tick(time.delta());

    if level_timer.0.is_finished() {
        info!("Day is over!");
        return;
    }

    if level_state.pnj_index < level_state.customer_list.len() {
        level_state.customer_timer.tick(time.delta());

        if level_state.customer_timer.just_finished() {
            let pnj_name = &level_state.customer_list[level_state.pnj_index];
            info!("Customer {} arrived!", pnj_name);
            // Dummy drink choice
            info!("Customer {} wants a gin_tonic", pnj_name);

            level_state.pnj_index += 1;
            if level_state.pnj_index < level_state.customer_list.len() {
                level_state.customer_timer.reset();
            }
        }
    }
}
