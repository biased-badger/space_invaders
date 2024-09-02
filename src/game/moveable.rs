use bevy::prelude::*;

#[derive(Component)]
pub struct LinearMovement(pub Vec2);

impl LinearMovement {
    pub fn new(x: f32, y: f32) -> Self {
        LinearMovement(Vec2::new(x, y))
    }

    pub fn from_vec(v: Vec2) -> Self {
        LinearMovement(v)
    }
}

pub fn move_linear(
    time_fixed: Res<Time<Fixed>>,
    mut query: Query<(&mut Transform, &LinearMovement)>,
) {
    for (mut transform, &ref movement) in query.iter_mut() {
        if movement.0.x != 0. {
            transform.translation.x =
                transform.translation.x + movement.0.x * time_fixed.delta_seconds();
        }

        if movement.0.y != 0. {
            transform.translation.y =
                transform.translation.y + movement.0.y * time_fixed.delta_seconds();
        }
    }
}

#[derive(Component)]
pub struct PatternMovement {
    current: LinearMovement,
    next: LinearMovement,
}

impl PatternMovement {
    pub fn new(current: LinearMovement, next: LinearMovement) -> Self {
        PatternMovement { current, next }
    }

    pub fn next_step(mut self, next: LinearMovement) {
        self.current = self.next;
        self.next = next;
    }
}
