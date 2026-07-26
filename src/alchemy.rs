use crate::features::recipes::components::RecipesConfig;
use crate::features::recipes::plugin::RecipesAssets;
use crate::interaction::{CursorWorldPos, Held};
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

pub struct AlchemyPlugin;

impl Plugin for AlchemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_pouring, update_liquid_visuals));
    }
}

#[derive(Component, Clone)]
pub struct LiquidContainer {
    pub content: Option<String>,
    pub level: usize,
    pub max_doses: usize,
    #[allow(dead_code)]
    pub is_glass: bool,
}

#[derive(Component)]
pub struct LiquidVisual {
    pub container_height: f32,
    #[allow(dead_code)]
    pub max_width: f32,
}

fn handle_pouring(
    buttons: Res<ButtonInput<MouseButton>>,
    cursor_pos: Res<CursorWorldPos>,
    spatial_query: SpatialQuery,
    mut container_query: Query<(Entity, &mut LiquidContainer)>,
    held_query: Query<Entity, With<Held>>,
) {
    if buttons.just_pressed(MouseButton::Right) {
        let Some(held_entity) = held_query.iter().next() else {
            return;
        };

        let intersections =
            spatial_query.point_intersections(cursor_pos.0, &SpatialQueryFilter::default());

        let target_entity = intersections.iter().find(|compared| {
            return **compared != held_entity && container_query.contains(**compared);
        });

        if let Some(target) = target_entity {
            let combinations = container_query.get_many_mut([held_entity, *target]);
            if let Ok([mut held_container, mut target_container]) = combinations {
                if target_container.1.level < target_container.1.max_doses {
                    // Add mixing here
                }
            }
        }
    }
}

fn update_liquid_visuals(
    recipes_assets: Option<Res<RecipesAssets>>,
    recipes_configs: Res<Assets<RecipesConfig>>,
    mut container_query: Query<(&LiquidContainer, &mut AseAnimation), Changed<LiquidContainer>>,
    mut visual_query: Query<(
        &mut Transform,
        &mut MeshMaterial2d<ColorMaterial>,
        &LiquidVisual,
    )>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let recipes_config = recipes_assets.and_then(|ra| recipes_configs.get(&ra.recipes));

    for (container, mut animation) in container_query.iter_mut() {
        if let Some(config) = recipes_config
            && let Some(content) = &container.content
        {
            let base_texture = config.beverages.get(content).unwrap().texture.clone();
            let level = container.level.clamp(1, 4);
            let tag_name = format!("{base_texture}{level}");

            animation.animation = Animation::tag(&tag_name);
        } else if container.content.is_none() {
            animation.animation = Animation::tag("Empty");
        }
    }
}
