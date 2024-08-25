use crate::components;
use bevy::prelude::*;

pub fn move_camera(
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<components::Ball>)>,
    ball_query: Query<&Transform, (With<components::Ball>, Without<Camera>)>,
    time: Res<Time>,
) {
    let mut camera_transform = camera_query.single_mut();
    let ball_transform = ball_query.single();

    // camera_transform.translation = Vec3 {
    //     y: 10.0,
    //     ..ball_transform.translation
    // };

    camera_transform.translation = camera_transform.translation.lerp(
        Vec3 {
            y: 10.0,
            ..ball_transform.translation
        },
        (0.8 * time.delta_seconds()).min(1.0),
    );

    camera_transform.look_at(ball_transform.translation, -Vec3::Z);
}
