use bevy::prelude::*;

#[derive(Component)]
pub struct Hurting(pub f32);

pub const HURT_DURATION: f32 = 0.1;
