use bevy::prelude::*;

#[derive(Component)]
pub struct Particle {
    pub initial_position: Vec3,
    pub velocity: Vec2,
    pub max_lifetime: f32,
    pub lifetime: f32,
    pub gravity: f32,
}

// impl default for particle
impl Default for Particle {
    fn default() -> Self {
        Self {
            initial_position: Vec3::ZERO,
            velocity: Vec2::ZERO,
            max_lifetime: 4.0,
            lifetime: 4.0,
            gravity: -9.81,
        }
    }
}
