pub mod base;
pub mod camera;
pub mod damagable;
pub mod debug;
pub mod enemy;
pub mod health;
pub mod hurt;
pub mod particle;
pub mod player;
pub mod ui;
pub mod window;
pub mod xp;

use base::resources::SpriteSheetPlugin;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_rapier2d::prelude::*;
use camera::systems::CameraPlugin;
use debug::fps::FPSPlugin;
use enemy::systems::EnemyPlugin;
use hurt::systems::HurtPlugin;
use particle::systems::ParticlePlugin;
use player::systems::PlayerPlugin;
use ui::systems::UIPlugin;
use window::systems::CustomWindowPlugin;
use xp::systems::XPPlugin;

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
            UIPlugin,
        ))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins((FrameTimeDiagnosticsPlugin, FPSPlugin))
        // .add_systems(Update, spawn_background)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        // .add_plugins(RapierDebugRenderPlugin::default())
        .run();
}
