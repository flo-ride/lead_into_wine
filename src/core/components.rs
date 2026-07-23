use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct LoadingScreen;

#[derive(Component)]
pub struct LoadingText;

#[derive(Component)]
pub struct StartScreen;

#[derive(Component)]
pub struct AbstractBackground;

#[derive(Component)]
pub struct BackgroundBlob {
    pub base_x: f32,
    pub base_y: f32,
    pub speed: f32,
    pub phase: f32,
}

#[derive(Component)]
pub struct StartMenuButton {
    pub primary: bool,
}

#[derive(Component)]
pub struct PlayingHud;
