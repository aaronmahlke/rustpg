mod base;
mod camera;
mod damagable;
mod enemy;
mod fps;
mod health;
mod hurt;
mod player;
mod window;
mod xp;

use crate::base::resources::*;
use crate::camera::systems::*;
use crate::enemy::systems::*;
use crate::fps::*;
use crate::hurt::systems::*;
use crate::player::systems::*;
use crate::window::systems::*;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_rapier2d::prelude::*;
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
        ))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins((FrameTimeDiagnosticsPlugin, FPSPlugin))
        // .add_systems(Update, spawn_background)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(RapierDebugRenderPlugin::default())
        .run();
}
