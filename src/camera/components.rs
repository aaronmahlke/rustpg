use bevy::prelude::*;

#[derive(Component)]
pub struct Target;

#[derive(Component)]
pub struct CameraFollow {
    pub acceleration: f32,
    pub smoothness: f32,
    pub target_position: Vec3,
}
