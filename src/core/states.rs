use bevy::prelude::*;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};

#[allow(dead_code)]
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Loading,
    StartMenu,
    Playing,
    GameOver,
    Victory,
}

#[derive(SubStates, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[source(GameState = GameState::Playing)]
pub enum ShelfState {
    #[default]
    Closed, // à droite, position de repos
    Open, // tirée vers la gauche
}

impl GameState {
    pub fn initial() -> Self {
        #[cfg(feature = "dev")]
        {
            GameState::Playing
        }

        #[cfg(not(feature = "dev"))]
        {
            GameState::StartMenu
        }
    }
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_sub_state::<ShelfState>()
            .add_loading_state(
                LoadingState::new(GameState::Loading).continue_to_state(GameState::initial()),
            );
    }
}
