use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

use bevy_rapier2d::prelude::*;

mod base;
mod camera;
mod enemy;
mod fps;
mod hurt;
mod player;
mod window;

use crate::base::resources::*;
use crate::camera::systems::*;
use crate::enemy::systems::*;
use crate::fps::*;
use crate::hurt::systems::*;
use crate::player::components::Player;
use crate::player::systems::*;
use crate::window::systems::*;

fn main() {
    App::new()
        .add_plugins((
            CustomWindowPlugin,
            SpriteSheetPlugin,
            PlayerPlugin,
            EnemyPlugin,
            HurtPlugin,
            CameraPlugin,
        ))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins((FrameTimeDiagnosticsPlugin, FPSPlugin))
        // .add_systems(Update, spawn_background)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        // .add_plugins(RapierDebugRenderPlugin::default())
        .run();
}
