use crate::{
    debug::fps::FPSPlugin, game::systems::GamePlugin, window::systems::CustomWindowPlugin,
};

use super::components::*;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugins(CustomWindowPlugin)
            .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
            .add_plugins(GamePlugin)
            .add_plugins((FrameTimeDiagnosticsPlugin, FPSPlugin))
            .add_systems(Startup, setup);
    }
}

fn setup() {}
