use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::{Animation, AseAnimation};

use crate::{core::components::Scroll, features::level_loop::components::CustomerArrived};
// + ton import AseAnimation / Animation existant

#[derive(Component)]
pub struct ScrollEntering {
    pub timer: Timer,
    pub start_x: f32,
    pub end_x: f32,
}

#[derive(Component)]
pub struct ScrollExiting {
    pub timer: Timer,
    pub start_x: f32,
    pub end_x: f32,
}

const SCROLL_ANIM_DURATION: f32 = 0.4;
const SCROLL_REST_X: f32 = -510.0;
const SCROLL_REST_Y: f32 = 190.0;
const SCROLL_REST_Z: f32 = -10.0;
const SCROLL_OFFSET_X: f32 = 150.0; // départ plus à gauche, glisse vers sa position finale

pub fn spawn_scroll(
    mut commands: Commands,
    mut arrived_events: MessageReader<CustomerArrived>,
    asset_server: Res<AssetServer>,
) {
    for _ in arrived_events.read() {
        let start_x = SCROLL_REST_X - SCROLL_OFFSET_X;

        commands.spawn((
            Scroll,
            AseAnimation {
                aseprite: asset_server.load("models/scroll.aseprite"),
                animation: Animation::default(),
            },
            Sprite {
                color: Color::srgba(1.0, 1.0, 1.0, 0.0),
                ..default()
            },
            Transform {
                translation: Vec3::new(start_x, SCROLL_REST_Y, SCROLL_REST_Z),
                scale: Vec3::splat(0.7),
                ..default()
            },
            ScrollEntering {
                timer: Timer::from_seconds(SCROLL_ANIM_DURATION, TimerMode::Once),
                start_x,
                end_x: SCROLL_REST_X,
            },
        ));
    }
}

pub fn animate_scroll_entering(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Sprite, &mut ScrollEntering)>,
) {
    for (entity, mut transform, mut sprite, mut entering) in &mut query {
        entering.timer.tick(time.delta());
        let t = entering.timer.fraction();

        transform.translation.x = entering.start_x.lerp(entering.end_x, t);
        sprite.color.set_alpha(t);

        if entering.timer.is_finished() {
            info!("Finish the fade-in");
            commands.entity(entity).remove::<ScrollEntering>();
        }
    }
}

pub fn start_scroll_exiting(
    mut commands: Commands,
    mut day_ended_events: MessageReader<CustomerArrived>,
    scroll_query: Query<(Entity, &Transform), (With<Scroll>, Without<ScrollEntering>)>,
) {
    if day_ended_events.read().next().is_none() {
        return;
    }

    info!("Start the scroll exiting");
    for (entity, transform) in &scroll_query {
        commands.entity(entity).insert(ScrollExiting {
            timer: Timer::from_seconds(SCROLL_ANIM_DURATION, TimerMode::Once),
            start_x: transform.translation.x,
            end_x: transform.translation.x - SCROLL_OFFSET_X,
        });
    }
}

pub fn animate_scroll_exiting(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Sprite, &mut ScrollExiting)>,
) {
    for (entity, mut transform, mut sprite, mut exiting) in &mut query {
        exiting.timer.tick(time.delta());
        let t = exiting.timer.fraction();

        transform.translation.x = exiting.start_x.lerp(exiting.end_x, t);
        sprite.color.set_alpha(1.0 - t);

        if exiting.timer.is_finished() {
            info!("Finish the scroll exiting");
            commands.entity(entity).despawn();
        }
    }
}
