use crate::game::*;
use bevy::input::common_conditions::{input_just_pressed, input_pressed};
use bevy::prelude::*;
use bevy::window::EnabledButtons;

mod game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Space Invaders".into(),
                resolution: (app::WINDOW_WIDTH, app::WINDOW_HEIGHT).into(),
                resizable: false,
                enabled_buttons: EnabledButtons {
                    minimize: false,
                    maximize: false,
                    ..default()
                },
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb_u8(10, 0, 40)))
        .add_event::<turret::ShootEvent>()
        .add_systems(
            Startup,
            (
                camera::setup_camera,
                turret::setup_turret,
                enemy::setup_enemy,
            ),
        )
        .add_systems(
            Update,
            (
                app::exit_system.run_if(input_just_pressed(KeyCode::Escape)),
                turret::move_turret,
                projectile::shoot_projectile,
                moveable::move_linear,
                cooldown::tick_cooldowns,
                turret::shoot_turret.run_if(input_pressed(KeyCode::Space)),
                projectile::hit_enemy,
            ),
        )
        // .add_systems(FixedUpdate)
        .run();
}
