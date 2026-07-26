use crate::{
    core::states::GameState,
    features::shelf::{
        assets::BackgroundAssets,
        systems::{
            animate_shelf_transition, handle_player_keyboard_shelf, handle_shelf_button,
            setup_shelf,
        },
    },
};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct ShelfPlugin;

impl Plugin for ShelfPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::initial()) // Corrige la transition
                .load_collection::<BackgroundAssets>(),
        )
        .add_systems(OnEnter(GameState::Playing), setup_shelf)
        .add_systems(
            Update,
            (
                handle_shelf_button,
                handle_player_keyboard_shelf,
                animate_shelf_transition,
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}
