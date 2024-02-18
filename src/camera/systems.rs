use crate::game::components::GameState;

use super::components::*;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);

        // Playing
        app.add_systems(
            Update,
            camera_follow_system.run_if(in_state(GameState::Playing)),
        );

        // Upgrade
        app.add_systems(
            Update,
            camera_follow_system.run_if(in_state(GameState::Upgrade)),
        );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        CameraFollow {
            acceleration: 1.0,
            smoothness: 0.5,
            target_position: Vec3::ZERO,
        },
    ));
}

fn camera_follow_system(
    time: Res<Time>,
    mut query: Query<(&CameraFollow, &mut Transform), With<Camera>>,
    target_query: Query<&Transform, (With<Target>, Without<Camera>)>,
) {
    if let Ok(target_transform) = target_query.get_single() {
        for (camera_follow, mut transform) in query.iter_mut() {
            let direction = target_transform.translation - transform.translation;
            let acceleration = direction * camera_follow.acceleration;

            // Apply smoothing
            let velocity = acceleration * time.delta_seconds();
            transform.translation += velocity / camera_follow.smoothness;
        }
    }
}
