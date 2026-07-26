use bevy::prelude::*;
const BTN_PRIMARY: Color = Color::srgb(0.55, 0.27, 0.07); // Brown (SaddleBrown)
const BTN_PRIMARY_HOVER: Color = Color::srgb(0.70, 0.35, 0.10);
const BTN_SECONDARY: Color = Color::srgba(0.20, 0.15, 0.10, 0.90); // Darker wood/stone
const BTN_SECONDARY_HOVER: Color = Color::srgba(0.30, 0.22, 0.15, 0.95);
const TEXT_PRIMARY: Color = Color::srgb(0.95, 0.85, 0.65); // Warm parchment-like
const TEXT_MUTED: Color = Color::srgb(0.60, 0.50, 0.40);

pub mod background;
pub mod credits;
pub mod game_over;
pub mod playing;
pub mod start;
