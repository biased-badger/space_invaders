use bevy::color::palettes::basic::RED;
use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::prelude::*;

use crate::game::app::{WINDOW_BOTTOM, WINDOW_TOP};
use crate::game::enemy::Enemy;
use crate::game::moveable::LinearMovement;
use crate::game::turret::ShootEvent;

const PROJECTILE_WIDTH: f32 = 3.;
const PROJECTILE_HEIGHT: f32 = 20.;
const PROJECTILE_SPEED: f32 = 360.;

#[derive(Component)]
pub struct Projectile;

pub fn shoot_projectile(mut commands: Commands, mut event_shoot: EventReader<ShootEvent>) {
    for shot in event_shoot.read() {
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(shot.position_x, WINDOW_BOTTOM, 1.),
                    scale: Vec3::new(PROJECTILE_WIDTH, PROJECTILE_HEIGHT, 1.),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::from(RED),
                    ..default()
                },
                ..default()
            },
            Projectile,
            LinearMovement::new(0., PROJECTILE_SPEED),
        ));
    }
}

pub fn hit_enemy(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform), With<Projectile>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (p_entity, p_transform) in projectile_query.iter() {
        if p_transform.translation.y >= WINDOW_TOP {
            commands.entity(p_entity).despawn();
            continue;
        }

        let p_aabb = Aabb2d::new(
            p_transform.translation.truncate(),
            p_transform.scale.truncate() / 2.,
        );

        for (e_entity, e_transform) in enemy_query.iter() {
            let e_aabb = Aabb2d::new(
                e_transform.translation.truncate(),
                e_transform.scale.truncate() / 2.,
            );

            if p_aabb.intersects(&e_aabb) {
                info!("projectile {:?} hit enemy", p_entity);
                commands.entity(p_entity).despawn();
                commands.entity(e_entity).despawn();
                break;
            }
        }
    }
}
