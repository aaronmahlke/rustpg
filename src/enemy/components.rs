use crate::{base::components::*, health::components::Health};
use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub idle: AnimationIndices,
    pub walk: AnimationIndices,
    pub state: EnemyState,
    pub stats: EnemyStats,
}

pub struct EnemyState {
    pub moving: bool,
    pub facing: Vec3,
}

pub struct EnemyStats {
    pub size: f32,
    pub move_speed: f32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            idle: AnimationIndices {
                first: 321,
                last: 321,
            },
            walk: AnimationIndices {
                first: 322,
                last: 324,
            },
            state: EnemyState {
                moving: false,
                facing: Vec3::new(1.0, 0.0, 0.0),
            },
            stats: EnemyStats {
                size: 5.0,
                move_speed: 100.0,
            },
        }
    }
}
