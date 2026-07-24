use bevy::prelude::*;

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
pub enum InGameView {
    #[default]
    Customers, // View with the clients
    Alchemy, // View with the bottles and potion making
}

impl GameState {
    pub fn initial() -> Self {
        #[cfg(feature = "dev")]
        {
            GameState::Playing
        }

        #[cfg(not(feature = "dev"))]
        {
            GameState::Loading
        }
    }
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameState::initial())
            .add_sub_state::<InGameView>();
    }
}
