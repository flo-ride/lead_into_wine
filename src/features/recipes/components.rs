use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct ResultTypes {
    pub name: String,
    pub texture: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Asset, TypePath)]
pub struct RecipesConfig {
    pub beverages: std::collections::HashMap<String, ResultTypes>,
    pub recipes: std::collections::HashMap<(String, String), String>,
}
