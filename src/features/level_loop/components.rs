use crate::features::{personna::components::Persona, recipes::components::ResultTypes};
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct LevelDay {
    pub day: u32,
}

#[derive(Resource)]
pub struct CurrentLevel {
    pub customer_list: Vec<Persona>,
    pub customer_timer: Timer,
    pub level_timer: Timer,
    pub customer_order: String,
}

#[derive(Resource)]
pub struct CurrentPnjIndex(pub usize);

/// Marque une entité comme étant un pnj client actuellement en jeu.
#[derive(Component)]
pub struct Pnj;

#[derive(Component, Clone, Copy)]
pub struct Leaving {
    pub speed: f32,
}

impl Default for Leaving {
    fn default() -> Self {
        Self { speed: 150.0 } // pixels/seconde
    }
}

/// Envoyé quand un client arrive et doit être spawn.
#[derive(Message)]
pub struct CustomerArrived {
    pub index: usize,
}

#[derive(Message)]
pub struct DayEnded;
