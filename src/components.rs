use bevy::prelude::*;

#[allow(dead_code)]
#[derive(Component)]
pub struct Player {
    /// The name of the player instance
    pub name: String,
}

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct BallOwner;

#[derive(Component)]
pub struct Direction(pub Vec2);
impl Direction {
    pub fn new() -> Self {
        Direction(Vec2::new(0.0, 1.0))
    }

    pub fn normalize(&mut self) {
        self.0 = self.0.normalize()
    }
}

#[derive(Component)]
pub struct Velocity(pub f32);
impl Velocity {
    pub fn new() -> Self {
        Velocity(0.0)
    }
}
