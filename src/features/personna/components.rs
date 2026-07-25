use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct RaceProfile {
    pub tolerance: f32,
    pub mood_bias: f32,
    pub texture: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct Preference {
    pub recipe: String,
    pub tolerance_bonus: f32,
    pub add_effect: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct Persona {
    pub race: String,
    pub name: String,
    pub texture: Option<String>,
    pub greetings: Vec<String>,
    pub preferences: Vec<Preference>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Asset, TypePath)]
pub struct PersonnaConfig {
    pub races: Vec<String>,
    pub race_profiles: std::collections::HashMap<String, RaceProfile>,
    pub personas: Vec<Persona>,
}

impl PersonnaConfig {
    pub fn texture_for<'a>(&'a self, persona: &'a Persona) -> &'a str {
        persona
            .texture
            .as_deref()
            .or_else(|| {
                self.race_profiles
                    .get(&persona.race)
                    .and_then(|race| race.texture.as_deref())
            })
            .unwrap_or("knight.png")
    }
}
