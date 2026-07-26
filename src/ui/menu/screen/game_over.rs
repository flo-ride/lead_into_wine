use bevy::prelude::*;
use rand::RngExt;
use std::f32::consts::TAU;

use crate::core::states::GameState;
use crate::environment::PlayerLives;
use crate::features::level_loop::systems::LevelEntity;
use crate::ui::UiFont;
use crate::ui::menu::screen::{BTN_PRIMARY, BTN_PRIMARY_HOVER, BTN_SECONDARY, BTN_SECONDARY_HOVER};

// === PALETTE COULEURS AMBIANCE TAVERNE ÉTEINTE ===
const BG_DARK: Color = Color::srgb(0.06, 0.04, 0.04); // Cendres / Sombre
const TEXT_TITLE: Color = Color::srgb(0.85, 0.25, 0.20); // Rouge braise écarlate
const TEXT_SUBTITLE: Color = Color::srgb(0.70, 0.60, 0.50); // Parchemin vieilli
const TEXT_PRIMARY: Color = Color::srgb(0.95, 0.85, 0.65); // Or clair chaud

const BUTTON_WIDTH: Val = Val::Px(300.0);
const BUTTON_HEIGHT: Val = Val::Px(54.0);
const PARTICLE_COUNT: usize = 30;

// === COMPOSANTS ===
#[derive(Component)]
pub struct GameOverScreen;

#[derive(Component)]
pub struct GameOverBackground;

#[derive(Component)]
pub enum GameOverButtonAction {
    Restart,
    MainMenu,
}

#[derive(Component)]
pub struct EmberParticle {
    pub speed: f32,
    pub drift: f32,
    pub drift_phase: f32,
}

// === 1. FOND & BRAISES (ANIMATION) ===

pub fn spawn_game_over_background(mut commands: Commands) {
    let mut rng = rand::rng();

    commands
        .spawn((
            GameOverBackground,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            BackgroundColor(BG_DARK),
            ZIndex(0),
        ))
        .with_children(|root| {
            // Particules style "braises mourantes" de la taverne
            for _ in 0..PARTICLE_COUNT {
                let size = rng.random_range(3.0..10.0);
                let left = rng.random_range(0.0..100.0);
                let top = rng.random_range(0.0..100.0);
                let speed = rng.random_range(10.0..30.0);
                let drift = rng.random_range(6.0..20.0);
                let drift_phase = rng.random_range(0.0..TAU);
                let alpha = rng.random_range(0.15..0.45);

                // Nuances de rouge / orange feu
                let r = rng.random_range(0.7..0.95);
                let g = rng.random_range(0.15..0.35);

                root.spawn((
                    EmberParticle {
                        speed,
                        drift,
                        drift_phase,
                    },
                    Node {
                        width: Val::Px(size),
                        height: Val::Px(size),
                        position_type: PositionType::Absolute,
                        left: Val::Percent(left),
                        top: Val::Percent(top),
                        border_radius: BorderRadius::all(Val::Percent(50.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(r, g, 0.05, alpha)),
                ));
            }
        });
}

pub fn despawn_game_over_background(
    mut commands: Commands,
    query: Query<Entity, With<GameOverBackground>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub fn animate_game_over_background(
    time: Res<Time>,
    mut particles: Query<(&EmberParticle, &mut Node)>,
) {
    let elapsed = time.elapsed_secs();

    for (particle, mut node) in &mut particles {
        let Val::Percent(top) = node.top else {
            continue;
        };

        let mut new_top = top - particle.speed * time.delta_secs();
        if new_top < -5.0 {
            new_top = 105.0;
        }
        node.top = Val::Percent(new_top);

        let drift_offset = (elapsed * 0.8 + particle.drift_phase).sin() * particle.drift * 0.01;
        if let Val::Percent(left) = node.left {
            node.left = Val::Percent((left + drift_offset).clamp(0.0, 100.0));
        }
    }
}

// === 2. INTERFACE GAME OVER ===

pub fn spawn_game_over_screen(mut commands: Commands, ui_font: Res<UiFont>) {
    commands
        .spawn((
            GameOverScreen,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(16.0),
                padding: UiRect::all(Val::Px(32.0)),
                ..default()
            },
            ZIndex(10),
        ))
        .with_children(|root| {
            // Titre & Sous-titre
            root.spawn(Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(28.0)),
                row_gap: Val::Px(8.0),
                ..default()
            })
            .with_children(|header| {
                header.spawn((
                    Text::new("GAME OVER"),
                    ui_font.text(64.0),
                    TextColor(TEXT_TITLE),
                ));
                header.spawn((
                    Text::new("La taverne a ferme ses portes..."),
                    ui_font.text(20.0),
                    TextColor(TEXT_SUBTITLE),
                ));
            });

            // Bouton Réessayer (Bouton principal)
            spawn_button(
                root,
                "Recommencer le service",
                GameOverButtonAction::Restart,
                BTN_PRIMARY,
                &ui_font,
            );

            // Bouton Menu Principal (Bouton secondaire)
            spawn_button(
                root,
                "Menu Principal",
                GameOverButtonAction::MainMenu,
                BTN_SECONDARY,
                &ui_font,
            );
        });
}

pub fn despawn_game_over_screen(
    mut commands: Commands,
    query: Query<Entity, With<GameOverScreen>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn spawn_button(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    action: GameOverButtonAction,
    bg_color: Color,
    ui_font: &UiFont,
) {
    parent
        .spawn((
            action,
            Button,
            Node {
                width: BUTTON_WIDTH,
                height: BUTTON_HEIGHT,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border_radius: BorderRadius::all(Val::Px(12.0)),
                margin: UiRect::top(Val::Px(4.0)),
                ..default()
            },
            BackgroundColor(bg_color),
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(label),
                ui_font.text(20.0),
                TextColor(TEXT_PRIMARY),
            ));
        });
}

// === 3. GESTION DE LA NAVIGATION ===

pub fn update_game_over_buttons(
    mut buttons: Query<
        (&Interaction, &GameOverButtonAction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut lives: ResMut<PlayerLives>,
) {
    for (interaction, action, mut color) in &mut buttons {
        let (normal, hover) = match action {
            GameOverButtonAction::Restart => (BTN_PRIMARY, BTN_PRIMARY_HOVER),
            GameOverButtonAction::MainMenu => (BTN_SECONDARY, BTN_SECONDARY_HOVER),
        };

        *color = match *interaction {
            Interaction::Pressed | Interaction::Hovered => BackgroundColor(hover),
            Interaction::None => BackgroundColor(normal),
        };

        if *interaction == Interaction::Pressed {
            match action {
                GameOverButtonAction::Restart => {
                    lives.count = 3; // Réinitialise les 3 vies
                    next_state.set(GameState::Playing);
                }
                GameOverButtonAction::MainMenu => {
                    lives.count = 3; // Réinitialise les 3 vies
                    next_state.set(GameState::StartMenu);
                }
            }
        }
    }
}

pub fn cleanup_level(mut commands: Commands, query: Query<Entity, With<LevelEntity>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
