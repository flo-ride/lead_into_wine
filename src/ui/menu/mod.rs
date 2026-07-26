mod screen;

use crate::core::states::GameState;
use crate::ui::UiFont;
use crate::ui::menu::screen::background::{
    animate_abstract_background, despawn_abstract_background, spawn_abstract_background,
};
use crate::ui::menu::screen::credits::{
    animate_credits_background, despawn_credits_background, despawn_credits_screen,
    spawn_credits_background, spawn_credits_screen, update_credits_button,
};
use crate::ui::menu::screen::game_over::{
    animate_game_over_background, cleanup_level, despawn_game_over_background,
    despawn_game_over_screen, spawn_game_over_background, spawn_game_over_screen,
    update_game_over_buttons,
};
use crate::ui::menu::screen::playing::{despawn_playing_hud, spawn_playing_hud};
use crate::ui::menu::screen::start::{
    despawn_start_screen, spawn_start_screen, update_start_menu_buttons,
};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<UiFont>()
            .add_systems(
                OnEnter(GameState::StartMenu),
                (spawn_abstract_background, spawn_start_screen),
            )
            .add_systems(
                OnExit(GameState::StartMenu),
                (despawn_start_screen, despawn_abstract_background),
            )
            .add_systems(OnEnter(GameState::Playing), spawn_playing_hud)
            .add_systems(OnExit(GameState::Playing), despawn_playing_hud)
            .add_systems(
                OnEnter(GameState::Credits),
                (spawn_credits_background, spawn_credits_screen),
            )
            .add_systems(
                OnExit(GameState::Credits),
                (despawn_credits_background, despawn_credits_screen),
            )
            .add_systems(
                Update,
                (animate_credits_background, update_credits_button)
                    .run_if(in_state(GameState::Credits)),
            )
            .add_systems(
                Update,
                (animate_abstract_background, update_start_menu_buttons)
                    .run_if(in_state(GameState::StartMenu)),
            )
            .add_systems(
                OnEnter(GameState::GameOver),
                (spawn_game_over_background, spawn_game_over_screen),
            )
            .add_systems(
                Update,
                (animate_game_over_background, update_game_over_buttons)
                    .run_if(in_state(GameState::GameOver)),
            )
            .add_systems(
                OnExit(GameState::GameOver),
                (
                    despawn_game_over_background,
                    despawn_game_over_screen,
                    cleanup_level,
                ),
            );
    }
}
