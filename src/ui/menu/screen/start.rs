use bevy::prelude::*;

use crate::core::components::{StartMenuButton, StartScreen};
use crate::core::states::GameState;
use crate::ui::UiFont;
use crate::ui::menu::screen::{
    BTN_PRIMARY, BTN_PRIMARY_HOVER, BTN_SECONDARY, BTN_SECONDARY_HOVER, TEXT_MUTED, TEXT_PRIMARY,
};

const BUTTON_WIDTH: Val = Val::Px(300.0);
const BUTTON_HEIGHT: Val = Val::Px(54.0);

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
                Text::new("Lead Into Wine"),
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
            spawn_action_button(column, "Credits", false, ui_font);
        });
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
            if button.primary {
                next_state.set(GameState::Playing);
            } else {
                next_state.set(GameState::Credits);
            }
        }
    }
}
