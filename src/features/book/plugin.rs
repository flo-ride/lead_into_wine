use crate::{
    core::states::GameState,
    features::book::{
        assets::BookAssets,
        systems::{
            animate_book_transition, handle_book_button, handle_player_keyboard_book, setup_book,
        },
    },
};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct BookPlugin;

impl Plugin for BookPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::initial())
                .load_collection::<BookAssets>(),
        )
        .add_systems(OnEnter(GameState::Playing), setup_book)
        .add_systems(
            Update,
            (
                handle_book_button,
                handle_player_keyboard_book,
                animate_book_transition,
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}
