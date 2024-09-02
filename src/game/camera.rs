use bevy::prelude::*;

#[derive(Component)]
struct AppCamera;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), AppCamera));
}
