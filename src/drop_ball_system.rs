use crate::components;
use bevy::prelude::*;

pub fn drop_ball(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    query_ball_owner_entity: Query<
        (Entity, &components::Direction),
        (With<components::BallOwner>, With<components::Player>),
    >,
    query_ball_entity: Query<Entity, With<components::Ball>>,
) {
    if input.just_pressed(KeyCode::Space) {
        // detach from owner
        let (owner_entity, owner_direction) = query_ball_owner_entity.single();
        commands
            .entity(owner_entity)
            .remove::<components::BallOwner>();

        // Attach movement
        let ball_entity = query_ball_entity.single();
        commands
            .entity(ball_entity)
            .insert(components::Direction(owner_direction.0.clone()))
            .insert(components::Velocity(3.0));
    }
}
