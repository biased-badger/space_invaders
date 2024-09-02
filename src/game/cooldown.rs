use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct Cooldown(Timer);

impl Cooldown {
    pub fn new(duration: Duration) -> Self {
        Cooldown(Timer::new(duration, TimerMode::Once))
    }
}

pub fn tick_cooldowns(
    mut commands: Commands,
    mut cooldowns: Query<(Entity, &mut Cooldown)>,
    time_fixed: Res<Time<Fixed>>,
) {
    for (entity, mut cooldown) in &mut cooldowns {
        cooldown.0.tick(time_fixed.delta());

        if cooldown.0.finished() {
            commands.entity(entity).remove::<Cooldown>();
        }
    }
}
