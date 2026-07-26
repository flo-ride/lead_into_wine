use crate::core::states::BookState;
use crate::features::{book::assets::BookAssets, level_loop::systems::LevelEntity};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_aseprite_ultra::prelude::{Animation, AseAnimation};

#[derive(Component)]
pub struct Book;

#[derive(Component)]
pub struct BookPullButton;

const BOOK_OPEN_Y: f32 = 0.0; // position tirée, visible
const BOOK_CLOSED_Y: f32 = -720.0; // position de repos, en bas
const BOOK_MOVE_SPEED: f32 = 5.0;

pub fn setup_book(mut commands: Commands, assets: Res<BookAssets>) {
    // Le livre lui-même, part fermé (en bas)
    commands.spawn((
        LevelEntity,
        AseAnimation {
            aseprite: assets.book.clone(),
            animation: Animation::default(),
        },
        Sprite {
            custom_size: Some(Vec2::new(1280., 720.)),
            ..default()
        },
        Transform::from_xyz(0.0, BOOK_CLOSED_Y, 3.0), // Z=3.0 pour être au-dessus du reste
        Book,
    ));

    // Bouton pour tirer le livre
    commands.spawn((
        Button,
        LevelEntity,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(100.0), // En bas de l'écran quand fermé (sera ajusté)
            left: Val::Percent(45.0),
            width: Val::Px(200.0), // À ajuster selon l'image
            height: Val::Px(60.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ImageNode::default(),
        AseAnimation {
            aseprite: assets.book_icon.clone(),
            animation: Animation::default(),
        },
        #[cfg(feature = "dev")]
        {
            BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.5))
        },
        BookPullButton,
    ));
}

pub fn handle_player_keyboard_book(
    current_state: Res<State<BookState>>,
    mut next_state: ResMut<NextState<BookState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyB) {
        // Use B for Book instead of Space to not conflict
        let next = match current_state.get() {
            BookState::Closed => BookState::Open,
            BookState::Open => BookState::Closed,
        };
        next_state.set(next);
    }
}

pub fn handle_book_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<BookPullButton>)>,
    current_state: Res<State<BookState>>,
    mut next_state: ResMut<NextState<BookState>>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            let next = match current_state.get() {
                BookState::Closed => BookState::Open,
                BookState::Open => BookState::Closed,
            };
            next_state.set(next);
        }
    }
}

pub fn animate_book_transition(
    current_state: Res<State<BookState>>,
    mut book_query: Query<&mut Transform, With<Book>>,
    mut button_query: Query<&mut Node, With<BookPullButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    let target_y = match current_state.get() {
        BookState::Closed => BOOK_CLOSED_Y,
        BookState::Open => BOOK_OPEN_Y,
    };

    let mut current_y = BOOK_CLOSED_Y;
    if let Ok(mut transform) = book_query.single_mut() {
        let diff = target_y - transform.translation.y;
        // On modifie directement la translation puisqu'il n'y a pas de corps physique
        transform.translation.y += diff * BOOK_MOVE_SPEED * time.delta_secs();
        current_y = transform.translation.y;
    }

    if let Ok(mut node) = button_query.single_mut() {
        if let Ok(window) = window_query.single() {
            let window_height = window.resolution.height();
            let ratio = (current_y - BOOK_CLOSED_Y) / (BOOK_OPEN_Y - BOOK_CLOSED_Y);

            // Quand ratio = 1 (ouvert), le bouton est tout en haut (top = 0)
            // Quand ratio = 0 (fermé), le bouton est tout en bas de la fenêtre (top = window_height - 60)
            node.top = Val::Px((1.0 - ratio) * (window_height - 60.0));
            node.bottom = Val::Auto;
        }
    }
}
