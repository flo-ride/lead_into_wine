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
#[allow(dead_code)]
pub struct LiquidVisual {
    pub container_height: f32,
    pub max_width: f32,
}

fn handle_pouring(
    buttons: Res<ButtonInput<MouseButton>>,
    cursor_pos: Res<CursorWorldPos>,
    spatial_query: SpatialQuery,
    mut container_query: Query<(Entity, &mut LiquidContainer)>,
    held_query: Query<Entity, With<Held>>,
    recipes_assets: Option<Res<RecipesAssets>>,
    recipes_configs: Res<Assets<RecipesConfig>>,
) {
    if buttons.just_pressed(MouseButton::Right) {
        let Some(held_entity) = held_query.iter().next() else {
            return;
        };

        let intersections =
            spatial_query.point_intersections(cursor_pos.0, &SpatialQueryFilter::default());

        let target_entity = intersections
            .iter()
            .find(|compared| **compared != held_entity && container_query.contains(**compared));

        if let Some(target) = target_entity {
            let combinations = container_query.get_many_mut([held_entity, *target]);
            if let Ok([mut held, mut target]) = combinations {
                let held_container = &mut held.1;
                let target_container = &mut target.1;

                if held_container.level > 0
                    && target_container.level < target_container.max_doses
                    && let Some(poured_content) = held_container.content.clone()
                {
                    // We pour one dose
                    held_container.level -= 1;
                    if held_container.level == 0 {
                        held_container.content = None;
                    }

                    target_container.level += 1;

                    if let Some(target_content) = target_container.content.clone() {
                        // Target already had liquid, attempt mixing
                        if let Some(ra) = &recipes_assets
                            && let Some(config) = recipes_configs.get(&ra.recipes)
                        {
                            let new_content = config
                                .recipes
                                .get(&(poured_content.clone(), target_content.clone()))
                                .or_else(|| config.recipes.get(&(target_content, poured_content)));

                            if let Some(c) = new_content {
                                target_container.content = Some(c.clone());
                            }
                        }
                    } else {
                        // Target was empty
                        target_container.content = Some(poured_content);
                    }
                }
            }
        }
    }
}

fn update_liquid_visuals(
    recipes_assets: Option<Res<RecipesAssets>>,
    recipes_configs: Res<Assets<RecipesConfig>>,
    mut container_query: Query<(&LiquidContainer, &mut AseAnimation), Changed<LiquidContainer>>,
) {
    let recipes_config = recipes_assets.and_then(|ra| recipes_configs.get(&ra.recipes));

    for (container, mut animation) in container_query.iter_mut() {
        if container.is_glass {
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
        } else {
            let tag_name = match container.level {
                0 => "Empty",
                1 => "1/5",
                2 => "2/5",
                3 => "3/5",
                4 => "4/5",
                _ => "Full",
            };
            animation.animation = Animation::tag(tag_name);
        }
    }
}
