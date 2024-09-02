use bevy::prelude::*;

// Window size
pub const WINDOW_WIDTH: f32 = 800.;
pub const WINDOW_HEIGHT: f32 = 600.;

// Window bounds for convenience
pub const WINDOW_TOP: f32 = WINDOW_HEIGHT / 2.;
pub const WINDOW_BOTTOM: f32 = -WINDOW_HEIGHT / 2.;
pub const WINDOW_LEFT: f32 = -WINDOW_WIDTH / 2.;
pub const WINDOW_RIGHT: f32 = WINDOW_WIDTH / 2.;

// System that exits the app
pub fn exit_system(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit::Success);
}
