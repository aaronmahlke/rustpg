use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    window::{PresentMode, WindowTheme},
};

use bevy_rapier2d::prelude::*;

mod base;
mod enemy;
mod fps;
mod hurt;
mod player;

use crate::base::resources::*;
use crate::enemy::systems::*;
use crate::fps::*;
use crate::hurt::systems::*;
use crate::player::systems::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "bevyaac".into(),
                        present_mode: PresentMode::Immediate,
                        prevent_default_event_handling: false,
                        window_theme: Some(WindowTheme::Dark),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins((SpriteSheetPlugin, PlayerPlugin, EnemyPlugin, HurtPlugin))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins((FrameTimeDiagnosticsPlugin, FPSPlugin))
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, setup_system)
        .run();
}

fn setup_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
