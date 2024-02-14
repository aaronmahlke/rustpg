use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_rapier2d::prelude::*;

pub mod base;
pub mod camera;
pub mod damagable;
pub mod debug;
pub mod enemy;
pub mod health;
pub mod hurt;
pub mod particle;
pub mod player;
pub mod window;
pub mod xp;

use crate::base::resources::SpriteSheetPlugin;
use crate::camera::systems::CameraPlugin;
use crate::debug::fps::FPSPlugin;
use crate::enemy::systems::EnemyPlugin;
use crate::hurt::systems::HurtPlugin;
use crate::particle::systems::ParticlePlugin;
use crate::player::systems::PlayerPlugin;
use crate::window::systems::CustomWindowPlugin;
use crate::xp::systems::XPPlugin;

fn main() {
    App::new()
        .add_plugins((
            CustomWindowPlugin,
            SpriteSheetPlugin,
            PlayerPlugin,
            EnemyPlugin,
            HurtPlugin,
            CameraPlugin,
            XPPlugin,
            ParticlePlugin,
        ))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins((FrameTimeDiagnosticsPlugin, FPSPlugin))
        // .add_systems(Update, spawn_background)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        // .add_plugins(RapierDebugRenderPlugin::default())
        .run();
}
