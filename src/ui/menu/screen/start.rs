use bevy::prelude::*;

use crate::core::components::{StartMenuButton, StartScreen};
use crate::core::states::GameState;
use crate::ui::UiFont;

const BUTTON_WIDTH: Val = Val::Px(300.0);
const BUTTON_HEIGHT: Val = Val::Px(54.0);

const BTN_PRIMARY: Color = Color::srgb(0.55, 0.27, 0.07); // Brown (SaddleBrown)
const BTN_PRIMARY_HOVER: Color = Color::srgb(0.70, 0.35, 0.10);
const BTN_SECONDARY: Color = Color::srgba(0.20, 0.15, 0.10, 0.90); // Darker wood/stone
const BTN_SECONDARY_HOVER: Color = Color::srgba(0.30, 0.22, 0.15, 0.95);
const TEXT_PRIMARY: Color = Color::srgb(0.95, 0.85, 0.65); // Warm parchment-like
const TEXT_MUTED: Color = Color::srgb(0.60, 0.50, 0.40);

pub fn spawn_start_screen(mut commands: Commands, ui_font: Res<UiFont>) {
    commands
        .spawn((
            StartScreen,
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
            spawn_button_column(root, &ui_font);
            spawn_disclaimer(root, &ui_font);
            spawn_footer(root, &ui_font);
        });
}

pub fn despawn_start_screen(mut commands: Commands, query: Query<Entity, With<StartScreen>>) {
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
            ..default()
        })
        .with_children(|title| {
            title.spawn((
                Text::new("THE THIRSTY SKELETON"),
                ui_font.text(72.0),
                TextColor(Color::srgb(0.95, 0.85, 0.65)),
            ));
            title.spawn((
                Text::new("Une aventure de comptoir"),
                ui_font.text(22.0),
                TextColor(TEXT_MUTED),
                Node {
                    margin: UiRect::top(Val::Px(8.0)),
                    ..default()
                },
            ));
        });
}

fn spawn_button_column(parent: &mut ChildSpawnerCommands, ui_font: &UiFont) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(12.0),
            ..default()
        })
        .with_children(|column| {
            spawn_action_button(column, "Play", true, ui_font);
            spawn_action_button(column, "Volume", false, ui_font);
            spawn_action_button(column, "Settings", false, ui_font);
            spawn_action_button(column, "Credits", false, ui_font);
        });
}

fn spawn_disclaimer(parent: &mut ChildSpawnerCommands, ui_font: &UiFont) {
    parent.spawn((
        Text::new(
            "Some assets are made with AI but it's temporary, they are only used for development.",
        ),
        ui_font.text(14.0),
        TextColor(Color::srgba(0.8, 0.4, 0.4, 0.9)),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(50.0),
            ..default()
        },
    ));
}

fn spawn_footer(parent: &mut ChildSpawnerCommands, ui_font: &UiFont) {
    parent.spawn((
        Text::new("v0.1.0 — TODO CORP TEAM "),
        ui_font.text(14.0),
        TextColor(Color::srgba(0.55, 0.58, 0.68, 0.9)),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.0),
            ..default()
        },
    ));
}

fn spawn_action_button(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    primary: bool,
    ui_font: &UiFont,
) {
    let color = if primary { BTN_PRIMARY } else { BTN_SECONDARY };

    parent
        .spawn((
            StartMenuButton { primary },
            Button,
            Node {
                width: BUTTON_WIDTH,
                height: BUTTON_HEIGHT,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border_radius: BorderRadius::all(Val::Px(12.0)),
                ..default()
            },
            BackgroundColor(color),
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(label),
                ui_font.text(22.0),
                TextColor(TEXT_PRIMARY),
            ));
        });
}

pub fn update_start_menu_buttons(
    mut buttons: Query<
        (&Interaction, &mut BackgroundColor, &StartMenuButton),
        Changed<Interaction>,
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, button) in &mut buttons {
        let (normal, hover) = if button.primary {
            (BTN_PRIMARY, BTN_PRIMARY_HOVER)
        } else {
            (BTN_SECONDARY, BTN_SECONDARY_HOVER)
        };

        *color = match *interaction {
            Interaction::Pressed | Interaction::Hovered => BackgroundColor(hover),
            Interaction::None => BackgroundColor(normal),
        };

        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Playing);
        }
    }
}
