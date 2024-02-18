use std::time::Duration;

use crate::game::components::GameState;

use super::components::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct AjmAudioPlugin;

impl Plugin for AjmAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlaySoundEffectEvent>();
        app.add_event::<PlayMusicEvent>();
        app.add_event::<StopSoundEvent>();
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Menu)
                .load_collection::<GameAudioAssets>(),
        );
        app.init_resource::<GameAudioAssets>();
        app.add_systems(Update, play_music);
        app.add_systems(Update, play_sound_effect_system);
        app.add_systems(Update, stop_sound_system);
    }
}

fn play_music(
    mut play_sound_event_reader: EventReader<PlayMusicEvent>,
    audio: Res<Audio>,
    audio_assets: Res<GameAudioAssets>,
) {
    for event in play_sound_event_reader.read() {
        if event.looping {
            audio
                .play(audio_assets.get_music(&event.sound))
                .fade_in(AudioTween::new(
                    Duration::from_secs(2),
                    AudioEasing::OutPowi(2),
                ))
                .looped();
            return;
        } else {
            audio.play(audio_assets.get_music(&event.sound));
        }
    }
}

fn play_sound_effect_system(
    mut play_sound_event_reader: EventReader<PlaySoundEffectEvent>,
    audio: Res<Audio>,
    audio_assets: Res<GameAudioAssets>,
) {
    for event in play_sound_event_reader.read() {
        audio.play(audio_assets.get_sound_effect(&event.sound));
    }
}

fn stop_sound_system(mut stop_sound_event_reader: EventReader<StopSoundEvent>, audio: Res<Audio>) {
    for _ in stop_sound_event_reader.read() {
        audio.stop();
    }
}
