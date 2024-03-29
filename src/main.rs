pub mod audio;
pub mod base;
pub mod camera;
pub mod damagable;
pub mod debug;
pub mod enemy;
pub mod game;
pub mod health;
pub mod hurt;
pub mod particle;
pub mod player;
pub mod ui;
pub mod upgrade;
pub mod window;
pub mod xp;

use bevy::prelude::*;

use bevy_rapier2d::render::RapierDebugRenderPlugin;
use game::systems::GameStatePlugin;

fn main() {
    App::new()
        .add_plugins(GameStatePlugin)
        // .add_plugins(RapierDebugRenderPlugin::default())
        .run();
}
