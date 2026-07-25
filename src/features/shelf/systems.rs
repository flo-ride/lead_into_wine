use crate::features::shelf::assets::BackgroundAssets;

use crate::core::states::ShelfState;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_aseprite_ultra::prelude::{Animation, AseAnimation};

#[derive(Component)]
pub struct Shelf;

#[derive(Component)]
pub struct ShelfPullButton;

const SHELF_OPEN_X: f32 = 0.0; // position tirée, vers la gauche
const SHELF_CLOSED_X: f32 = 965.0; // position de repos, à droite
const SHELF_MOVE_SPEED: f32 = 5.0;

pub fn setup_shelf(
    mut commands: Commands,
    assets: Res<BackgroundAssets>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single().unwrap();

    // La shelf elle-même, part fermée (à droite)
    commands.spawn((
        AseAnimation {
            aseprite: assets.front_shelf.clone(),
            animation: Animation::default(),
        },
        Sprite {
            custom_size: Some(Vec2::new(window.width(), window.height())),
            ..default()
        },
        Transform::from_xyz(SHELF_CLOSED_X, 0.0, 10.0),
        Shelf,
    ));

    // Bouton pour tirer la shelf
    commands.spawn((
        Button,
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(15.0),
            top: Val::Percent(20.0),
            width: Val::Px(60.0),
            height: Val::Px(300.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        #[cfg(feature = "dev")]
        {
            BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.5))
        },
        ShelfPullButton,
    ));
}

pub fn handle_player_keyboard_shelf(
    current_state: Res<State<ShelfState>>,
    mut next_state: ResMut<NextState<ShelfState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        let next = match current_state.get() {
            ShelfState::Closed => ShelfState::Open,
            ShelfState::Open => ShelfState::Closed,
        };
        next_state.set(next);
    }
}

pub fn handle_shelf_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<ShelfPullButton>)>,
    current_state: Res<State<ShelfState>>,
    mut next_state: ResMut<NextState<ShelfState>>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            let next = match current_state.get() {
                ShelfState::Closed => ShelfState::Open,
                ShelfState::Open => ShelfState::Closed,
            };
            next_state.set(next);
        }
    }
}

pub fn animate_shelf_transition(
    time: Res<Time>,
    current_state: Res<State<ShelfState>>,
    mut shelf_query: Query<&mut Transform, With<Shelf>>,
    mut button_query: Query<&mut Node, With<ShelfPullButton>>,
) {
    let target_x = match current_state.get() {
        ShelfState::Closed => SHELF_CLOSED_X,
        ShelfState::Open => SHELF_OPEN_X,
    };

    if let Ok(mut transform) = shelf_query.single_mut() {
        transform.translation.x = transform
            .translation
            .x
            .lerp(target_x, time.delta_secs() * SHELF_MOVE_SPEED);
    }

    let button_target_x = match current_state.get() {
        ShelfState::Closed => SHELF_OPEN_X,
        ShelfState::Open => SHELF_CLOSED_X,
    };

    if let Ok(mut node) = button_query.single_mut() {
        node.right = Val::Px(15.0 + button_target_x);
    }
}
