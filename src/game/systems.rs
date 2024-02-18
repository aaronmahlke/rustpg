use crate::{
    audio::{
        components::{
            MusicType, PlayMusicEvent, PlaySoundEffectEvent, SoundEffectType, StopSoundEvent,
        },
        systems::AjmAudioPlugin,
    },
    debug::fps::FPSPlugin,
    window::systems::CustomWindowPlugin,
};

use super::{components::*, GamePlugin};
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_kira_audio::prelude::*;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugins((CustomWindowPlugin, AudioPlugin, AjmAudioPlugin))
            .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
            .add_plugins(GamePlugin)
            .add_plugins((FrameTimeDiagnosticsPlugin, FPSPlugin));

        // Menu Music
        app.add_systems(OnEnter(GameState::Menu), setup_state_menu);

        // Game Music
        app.add_systems(OnEnter(GameState::Playing), setup_state_playing);

        // Upgade Music
        app.add_systems(OnEnter(GameState::Upgrade), setup_state_upgrade);
    }
}

fn setup_state_menu(
    mut sound_event: EventWriter<PlayMusicEvent>,
    audio: Res<Audio>,
    mut game: ResMut<GameRules>,
) {
    audio.stop();
    sound_event.send(PlayMusicEvent {
        sound: MusicType::Menu,
        looping: true,
        fade_in: 1000,
    });

    game.reset();
}

fn setup_state_playing(mut sound_event: EventWriter<PlayMusicEvent>, audio: Res<Audio>) {
    audio.stop();
    sound_event.send(PlayMusicEvent {
        sound: MusicType::Game,
        looping: true,
        ..default()
    });
}

fn setup_state_upgrade(
    mut music_event: EventWriter<PlayMusicEvent>,
    mut sound_event: EventWriter<PlaySoundEffectEvent>,
    audio: Res<Audio>,
) {
    audio.stop();
    sound_event.send(PlaySoundEffectEvent {
        sound: SoundEffectType::EnterLevelUp,
    });
    music_event.send(PlayMusicEvent {
        sound: MusicType::Upgrade,
        looping: true,
        fade_in: 1000,
    });
}
