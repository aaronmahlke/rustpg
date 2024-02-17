use crate::{debug::fps::FPSPlugin, window::systems::CustomWindowPlugin};

use super::{components::*, GamePlugin};
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugins(CustomWindowPlugin)
            .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
            .add_plugins(GamePlugin)
            .add_plugins((FrameTimeDiagnosticsPlugin, FPSPlugin));
    }
}
