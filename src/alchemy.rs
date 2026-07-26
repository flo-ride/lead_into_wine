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
    pub contents: Vec<String>, // List of ingredients currently inside
    pub max_doses: usize,
    pub base_color: Color, // Used mainly for raw bottles
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

        let filter = SpatialQueryFilter::default();
        let intersections = spatial_query.point_intersections(cursor_pos.0, &filter);

        let mut target_entity = None;
        for entity in intersections {
            if entity != held_entity && container_query.contains(entity) {
                target_entity = Some(entity);
                break;
            }
        }

        if let Some(target) = target_entity {
            let combinations = container_query.get_many_mut([held_entity, target]);
            if let Ok([mut held_container, mut target_container]) = combinations {
                if !held_container.1.contents.is_empty()
                    && target_container.1.contents.len() < target_container.1.max_doses
                {
                    // Transfer 1 dose (ingredient)
                    if let Some(ingredient) = held_container.1.contents.pop() {
                        target_container.1.contents.push(ingredient);
                    }
                }
            }
        }
    }
}

fn evaluate_mixture(contents: &[String], config: &RecipesConfig) -> String {
    let mut unique = contents.to_vec();
    unique.sort();
    unique.dedup();

    if unique.is_empty() {
        return "Empty".to_string();
    }

    if unique.len() == 1 {
        // Fallback for single raw ingredients to specific color bases
        match unique[0].as_str() {
            "Wine" => "Red".to_string(),
            "Beer" => "Yellow".to_string(),
            "Cider" => "Orange".to_string(),
            "Brandy" => "Purple".to_string(),
            "Unicorn Tear" => "Blue".to_string(),
            "Mandrake Root" => "Green".to_string(),
            _ => "Red".to_string(),
        }
    } else if unique.len() == 2 {
        let mut pair = (unique[0].clone(), unique[1].clone());
        if !config.recipes.contains_key(&pair) {
            pair = (unique[1].clone(), unique[0].clone()); // try reverse
        }

        if let Some(result_id) = config.recipes.get(&pair) {
            if let Some(result_type) = config.result_types.get(result_id) {
                return result_type.texture.clone();
            }
        }
        "Green".to_string() // Stagnant water fallback
    } else {
        "Green".to_string() // Too many ingredients -> stagnant water
    }
}

fn update_liquid_visuals(
    recipes_assets: Option<Res<RecipesAssets>>,
    recipes_configs: Res<Assets<RecipesConfig>>,
    mut container_query: Query<
        (&LiquidContainer, &Children, Option<&mut AseAnimation>),
        Changed<LiquidContainer>,
    >,
    mut visual_query: Query<(
        &mut Transform,
        &mut MeshMaterial2d<ColorMaterial>,
        &LiquidVisual,
    )>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let recipes_config = recipes_assets.and_then(|ra| recipes_configs.get(&ra.recipes));

    for (container, children, mut ase_anim) in container_query.iter_mut() {
        let fill_ratio = container.contents.len() as f32 / container.max_doses as f32;

        // 1. Update Aseprite Animation for Glass
        if let Some(ref mut anim) = ase_anim {
            if container.contents.is_empty() {
                anim.animation = Animation::tag("Empty");
            } else if let Some(config) = recipes_config {
                let base_texture = evaluate_mixture(&container.contents, config);
                let dose_num = container.contents.len().clamp(1, 4);
                let tag_name = format!("{}{}", base_texture, dose_num);

                anim.animation = Animation::tag(&tag_name);
            }
        }

        // 2. Update Mesh2d for Bottles (or fallback glass visual)
        for child in children.iter() {
            if let Ok((mut transform, mut material_handle, visual)) = visual_query.get_mut(child) {
                transform.scale.y = fill_ratio.max(0.001);

                let base_y = -visual.container_height / 2.0;
                let current_height = visual.container_height * fill_ratio;
                transform.translation.y = base_y + (current_height / 2.0);

                if !container.is_glass {
                    // Bottles keep their raw base color
                    material_handle.0 = materials.add(container.base_color);
                } else {
                    // If the glass still has a mesh visual, hide it entirely since we use Aseprite
                    material_handle.0 = materials.add(Color::NONE);
                }
            }
        }
    }
}
