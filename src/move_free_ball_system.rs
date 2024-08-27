use crate::components;
use bevy::prelude::*;

pub fn move_free_ball(
    mut query: Query<
        (
            &mut Transform,
            &mut components::Velocity,
            &components::Direction,
        ),
        With<components::Ball>,
    >,
    time: Res<Time>,
) {
    let (mut transform, mut velocity, direction) = query.single_mut();

    transform.translation.x += direction.0.x * velocity.0 * time.delta_seconds();
    transform.translation.z -= direction.0.y * velocity.0 * time.delta_seconds();

    if velocity.0 < 0.01 {
        velocity.0 = 0.0;
    } else {
        velocity.0 *= (1.0 - 0.7 * time.delta_seconds()).max(0.0);
    }
}
