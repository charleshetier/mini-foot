use crate::components;
use bevy::prelude::*;

pub fn move_players(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<
        (
            &mut Transform,
            &mut components::Direction,
            &mut components::Velocity,
        ),
        With<components::Player>,
    >,
) {
    for (mut transform, mut direction, mut velocity) in &mut query {
        let mut new_direction = Vec2::ZERO;
        velocity.0 = 0.0;

        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            new_direction.x -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::ArrowRight) {
            new_direction.x += 1.0;
        }

        if keyboard_input.pressed(KeyCode::ArrowUp) {
            new_direction.y += 1.0;
        }

        if keyboard_input.pressed(KeyCode::ArrowDown) {
            new_direction.y -= 1.0;
        }

        if new_direction.x != 0.0 || new_direction.y != 0.0 {
            direction.0 = new_direction;
            direction.normalize();
            velocity.0 = 1.0;
        }

        transform.translation.x += direction.0.x * velocity.0 * time.delta_seconds();
        transform.translation.z -= direction.0.y * velocity.0 * time.delta_seconds();
    }
}
