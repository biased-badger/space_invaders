use bevy::color::palettes::basic::GREEN;
use bevy::math::*;
use bevy::prelude::*;
use std::time::Duration;

use crate::game::app::*;
use crate::game::cooldown::Cooldown;

const TURRET_WIDTH: f32 = 50.;
const TURRET_HEIGHT: f32 = 20.;
const TURRET_SPEED: f32 = 120.;

// Cooldown in seconds
const SHOOT_COOLDOWN: f32 = 1.;

#[derive(Component)]
pub struct Turret;

#[derive(Event)]
pub struct ShootEvent {
    pub position_x: f32,
}

pub fn setup_turret(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0., WINDOW_BOTTOM + (TURRET_HEIGHT / 2.), 0.),
                scale: Vec3::new(TURRET_WIDTH, TURRET_HEIGHT, 1.),
                ..default()
            },
            sprite: Sprite {
                color: Color::from(GREEN),
                ..default()
            },
            ..default()
        },
        Turret,
    ));
}

pub fn move_turret(
    input: Res<ButtonInput<KeyCode>>,
    time_fixed: Res<Time<Fixed>>,
    mut turret_query: Query<&mut Transform, With<Turret>>,
) {
    let mut turret_transform = turret_query.single_mut();

    let mut direction = 0.;

    if input.pressed(KeyCode::KeyA) {
        direction -= 1.;
    }

    if input.pressed(KeyCode::KeyD) {
        direction += 1.;
    }

    let mut new_x =
        turret_transform.translation.x + direction * TURRET_SPEED * time_fixed.delta_seconds();

    new_x = new_x.min(WINDOW_RIGHT - TURRET_WIDTH / 2.);
    new_x = new_x.max(WINDOW_LEFT + TURRET_WIDTH / 2.);

    turret_transform.translation.x = new_x;
}

pub fn shoot_turret(
    mut commands: Commands,
    mut event_shoot: EventWriter<ShootEvent>,
    turret_entity: Query<(Entity, &Transform), With<Turret>>,
    cooldown_query: Query<&Cooldown, With<Turret>>,
) {
    let turret = turret_entity.single();

    if cooldown_query.is_empty() {
        commands
            .entity(turret.0)
            .insert(Cooldown::new(Duration::from_secs_f32(SHOOT_COOLDOWN)));

        event_shoot.send(ShootEvent {
            position_x: turret.1.translation.x,
        });
    }
}
