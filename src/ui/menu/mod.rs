mod screen;

use crate::core::states::GameState;
use crate::ui::UiFont;
use crate::ui::menu::screen::background::{
    animate_abstract_background, despawn_abstract_background, spawn_abstract_background,
};
use crate::ui::menu::screen::playing::{despawn_playing_hud, spawn_playing_hud};
use crate::ui::menu::screen::splash::{
    animate_loading_screen, despawn_loading_screen, loading_to_starting, spawn_loading_screen,
};
use crate::ui::menu::screen::start::{
    despawn_start_screen, spawn_start_screen, update_start_menu_buttons,
};
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiFont>()
            .add_systems(OnEnter(GameState::Loading), spawn_loading_screen)
            .add_systems(OnExit(GameState::Loading), despawn_loading_screen)
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
                Update,
                animate_loading_screen.run_if(in_state(GameState::Loading)),
            )
            .add_systems(
                Update,
                loading_to_starting.run_if(in_state(GameState::Loading)),
            )
            .add_systems(
                Update,
                (animate_abstract_background, update_start_menu_buttons)
                    .run_if(in_state(GameState::StartMenu)),
            );
    }
}
