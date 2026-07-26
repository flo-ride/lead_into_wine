use bevy::prelude::*;
use rand::{Rng, RngExt};

use crate::core::states::GameState;
use crate::ui::UiFont;
use crate::ui::menu::screen::{BTN_PRIMARY, BTN_PRIMARY_HOVER, TEXT_MUTED, TEXT_PRIMARY};

const BUTTON_WIDTH: Val = Val::Px(300.0);
const PARTICLE_COUNT: usize = 24;
const BUTTON_HEIGHT: Val = Val::Px(54.0);

#[derive(Component)]
pub struct CreditsScreen;

#[derive(Component)]
pub struct CreditsButton;

#[derive(Component)]
pub struct CreditsBackground;

// Liste des crédits : (catégorie, nom)
const CREDITS: &[(&str, &str)] = &[
    ("Game Design", "Matlom\nFloRide\nMrVym"),
    ("Programming", "FloRide\nMrVym"),
    ("Art", "Matlom"),
    // ("Music & SFX", "TODO CORP TEAM"),
    ("Special Thanks", "Bevy Engine Community\nLibreSprite\nRust"),
];

#[derive(Component)]
pub struct CreditsParticle {
    pub speed: f32,
    pub drift: f32,
    pub drift_phase: f32,
}

pub fn spawn_credits_background(mut commands: Commands) {
    let mut rng = rand::rng();

    commands
        .spawn((
            CreditsBackground,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            BackgroundColor(Color::srgb(0.08, 0.06, 0.04)), // Fond sombre taverne
            ZIndex(0),
        ))
        .with_children(|root| {
            for _ in 0..PARTICLE_COUNT {
                let size = rng.random_range(4.0..14.0);
                let left = rng.random_range(0.0..100.0);
                let top = rng.random_range(0.0..100.0);
                let speed = rng.random_range(8.0..22.0);
                let drift = rng.random_range(4.0..16.0);
                let drift_phase = rng.random_range(0.0..std::f32::consts::TAU);
                let alpha = rng.random_range(0.08..0.25);

                root.spawn((
                    CreditsParticle {
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
                    BackgroundColor(Color::srgba(0.55, 0.15, 0.2, alpha)),
                ));
            }
        });
}

pub fn despawn_credits_background(
    mut commands: Commands,
    query: Query<Entity, With<CreditsBackground>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub fn animate_credits_background(
    time: Res<Time>,
    mut particles: Query<(&CreditsParticle, &mut Node)>,
) {
    let elapsed = time.elapsed_secs();

    for (particle, mut node) in &mut particles {
        // Fait remonter la particule lentement et boucle en bas une fois en haut
        let Val::Percent(top) = node.top else {
            continue;
        };
        let mut new_top = top - particle.speed * time.delta_secs();
        if new_top < -5.0 {
            new_top = 105.0;
        }
        node.top = Val::Percent(new_top);

        // Léger mouvement horizontal sinusoïdal
        let drift_offset = (elapsed * 0.6 + particle.drift_phase).sin() * particle.drift * 0.01;
        if let Val::Percent(left) = node.left {
            node.left = Val::Percent((left + drift_offset).clamp(0.0, 100.0));
        }
    }
}
pub fn spawn_credits_screen(mut commands: Commands, ui_font: Res<UiFont>) {
    commands
        .spawn((
            CreditsScreen,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(8.0),
                padding: UiRect::all(Val::Px(32.0)),
                ..default()
            },
            ZIndex(10),
        ))
        .with_children(|root| {
            spawn_title(root, &ui_font);
            spawn_credits_list(root, &ui_font);
            spawn_back_button(root, &ui_font);
        });
}

pub fn despawn_credits_screen(mut commands: Commands, query: Query<Entity, With<CreditsScreen>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn spawn_title(parent: &mut ChildSpawnerCommands, ui_font: &UiFont) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            margin: UiRect::bottom(Val::Px(36.0)),
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|title| {
            title.spawn((
                Text::new("Credits"),
                ui_font.text(56.0),
                TextColor(Color::srgb(0.95, 0.85, 0.65)),
            ));
        });
}

fn spawn_credits_list(parent: &mut ChildSpawnerCommands, ui_font: &UiFont) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(18.0),
            margin: UiRect::bottom(Val::Px(40.0)),
            ..default()
        })
        .with_children(|list| {
            for (category, name) in CREDITS {
                spawn_credit_entry(list, category, name, ui_font);
            }
        });
}

fn spawn_credit_entry(
    parent: &mut ChildSpawnerCommands,
    category: &str,
    name: &str,
    ui_font: &UiFont,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(4.0),
            ..default()
        })
        .with_children(|entry| {
            entry.spawn((
                Text::new(category.to_uppercase()),
                ui_font.text(14.0),
                TextColor(TEXT_MUTED),
            ));
            entry.spawn((
                Text::new(name.to_string()),
                ui_font.text(22.0),
                TextColor(TEXT_PRIMARY),
            ));
        });
}

fn spawn_back_button(parent: &mut ChildSpawnerCommands, ui_font: &UiFont) {
    parent
        .spawn((
            CreditsButton,
            Button,
            Node {
                width: BUTTON_WIDTH,
                height: BUTTON_HEIGHT,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border_radius: BorderRadius::all(Val::Px(12.0)),
                ..default()
            },
            BackgroundColor(BTN_PRIMARY),
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new("Back"),
                ui_font.text(22.0),
                TextColor(TEXT_PRIMARY),
            ));
        });
}

pub fn update_credits_button(
    mut buttons: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<CreditsButton>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color) in &mut buttons {
        *color = match *interaction {
            Interaction::Pressed | Interaction::Hovered => BackgroundColor(BTN_PRIMARY_HOVER),
            Interaction::None => BackgroundColor(BTN_PRIMARY),
        };

        if *interaction == Interaction::Pressed {
            next_state.set(GameState::StartMenu);
        }
    }
}
