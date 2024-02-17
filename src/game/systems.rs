use crate::{
    audio::{
        components::{MusicType, PlayMusicEvent},
        systems::AjmAudioPlugin,
    },
    debug::fps::FPSPlugin,
    window::systems::CustomWindowPlugin,
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

        // Game Music
        app.add_systems(OnEnter(GameState::Playing), setup_state_playing);

        // Menu Music
        app.add_systems(OnEnter(GameState::Menu), setup_state_menu);
    }
}

fn setup_state_playing(mut sound_event: EventWriter<PlayMusicEvent>) {
    println!("Playing game music");
    sound_event.send(PlayMusicEvent {
        sound: MusicType::Menu,
        looping: true,
    });
}

fn setup_state_menu(mut sound_event: EventWriter<PlayMusicEvent>) {
    println!("Playing menu music");
    sound_event.send(PlayMusicEvent {
        sound: MusicType::Menu,
        looping: true,
    });
}
