use crate::{
    audio::systems::AjmAudioPlugin, debug::fps::FPSPlugin, window::systems::CustomWindowPlugin,
};

use super::{components::*, GamePlugin};
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_kira_audio::AudioPlugin;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugins((CustomWindowPlugin, AudioPlugin, AjmAudioPlugin))
            .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
            .add_plugins(GamePlugin)
            .add_plugins((FrameTimeDiagnosticsPlugin, FPSPlugin));
    }
}
