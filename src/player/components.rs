use bevy::prelude::*;

use crate::base::components::*;

#[derive(Component)]
pub struct Bullet {
    pub direction: Vec3,
    pub speed: f32,
    pub size: f32,
    pub damage: f32,
}

#[derive(Component)]
pub struct Player {
    pub idle: AnimationIndices,
    pub walk: AnimationIndices,
    pub state: PlayerState,
    pub stats: PlayerStats,
}

pub struct PlayerStats {
    pub size: f32,
    pub shot_speed: f32,
    pub move_speed: f32,
    pub xp: u32,
}

pub struct PlayerState {
    pub moving: bool,
    pub facing: Vec3,
}

#[derive(Component, Deref, DerefMut)]
pub struct ShootTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct BulletDespawnTimer(pub Timer);

impl Default for Player {
    fn default() -> Self {
        Self {
            idle: AnimationIndices {
                first: 241,
                last: 241,
            },
            walk: AnimationIndices {
                first: 242,
                last: 243,
            },
            state: PlayerState {
                moving: false,
                facing: Vec3::new(1.0, 0.0, 0.0),
            },
            stats: PlayerStats {
                size: 5.0,
                shot_speed: 0.4,
                move_speed: 300.0,
                xp: 0,
            },
        }
    }
}
