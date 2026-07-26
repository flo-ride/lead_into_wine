use crate::core::states::GameState;
use crate::features::level_loop::components::*;
use crate::features::level_loop::scroll_animation::{
    animate_scroll_entering, animate_scroll_exiting, spawn_scroll, start_scroll_exiting,
};
use crate::features::level_loop::systems::*;
use bevy::prelude::*;

pub struct LevelLoopPlugin;

impl Plugin for LevelLoopPlugin {
    fn build(&self, app: &mut App) {
        // Ajout de la ressource initiale pour éviter que le système `level_loop_system` ne panique
        // si init_level_loop n'a pas encore été appelé.
        // Ou mieux, on ajoute une condition sur le système `level_loop_system`
        // pour ne s'exécuter que si la ressource LevelState existe.
        app.add_message::<CustomerArrived>()
            .add_message::<DayEnded>()
            .add_systems(
                Update,
                init_level_loop
                    .run_if(in_state(GameState::Playing))
                    .run_if(not(level_initialized)),
            )
            .add_systems(
                Update,
                (
                    level_loop_system,
                    queue_pnj_spawn,
                    spawn_pnj,
                    animate_pnj_wait_indicator,
                    select_recipe,
                    write_customer_text,
                    hide_wait_indicator_on_leaving,
                    despawn_pnj,

                    pnj_departure_system,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing))
                    .run_if(level_initialized),
            )
            .add_systems(
                Update,
                (
                    animate_scroll_entering,
                    spawn_scroll,
                    start_scroll_exiting,
                    animate_scroll_exiting,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing))
                    .run_if(level_initialized),
            );
    }
}

fn level_initialized(level: Option<Res<CurrentLevel>>) -> bool {
    level.is_some()
}
