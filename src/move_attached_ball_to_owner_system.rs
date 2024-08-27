use crate::components;
use bevy::prelude::*;

pub fn ball_is_owned(
    ball_owner_query: Query<&components::Player, With<components::BallOwner>>,
) -> bool {
    !ball_owner_query.is_empty()
}

pub fn ball_is_not_owned(
    ball_owner_query: Query<&components::Player, With<components::BallOwner>>,
) -> bool {
    ball_owner_query.is_empty()
}

pub fn attach_ball_to_owner(
    ball_owner_query: Query<
        (&Transform, &components::Direction),
        (With<components::Player>, Without<components::Ball>),
    >,
    mut ball_query: Query<&mut Transform, (With<components::Ball>, Without<components::Player>)>,
    time: Res<Time>,
) {
    let ball_owner = ball_owner_query.get_single();
    let mut ball_transform = ball_query.single_mut();

    if let Ok((owner_transform, owner_direction)) = ball_owner {
        let target_position = Vec3::new(
            owner_transform.translation.x + owner_direction.0.x * 0.4,
            0.2,
            owner_transform.translation.z - owner_direction.0.y * 0.4,
        );

        ball_transform.translation = ball_transform
            .translation
            .lerp(target_position, time.delta_seconds() * 10.0)
    }
}
