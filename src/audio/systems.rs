use crate::game::components::GameState;

use super::components::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct AjmAudioPlugin;

impl Plugin for AjmAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlaySoundEffectEvent>();
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Game)
                .load_collection::<GameAudioAssets>(),
        );
        app.init_resource::<GameAudioAssets>();
        app.add_systems(Update, play_sound_effect_system);
    }
}

fn play_sound_effect_system(
    mut play_sound_event_reader: EventReader<PlaySoundEffectEvent>,
    audio: Res<Audio>,
    audio_assets: Res<GameAudioAssets>,
) {
    for event in play_sound_event_reader.read() {
        audio.play(audio_assets.get(&event.sound));
    }
}
