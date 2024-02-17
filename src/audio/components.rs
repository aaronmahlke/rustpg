use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

#[derive(Resource)]
pub struct SoundEffectsAudioChannel;

#[derive(Event)]
pub struct PlaySoundEffectEvent {
    pub sound: SoundEffectType,
}

pub enum SoundEffectType {
    PlayerShoot,
    PlayerHurt,
    PlayerDeath,
    EnemyWalk,
    EnemyHurt,
    EnemyDeath,
    XPCollect,
}

#[derive(AssetCollection, Resource, Default)]
pub struct GameAudioAssets {
    #[asset(path = "audio/player_shoot.wav")]
    player_shoot: Handle<AudioSource>,
    #[asset(path = "audio/player_hurt.wav")]
    player_hurt: Handle<AudioSource>,
    #[asset(path = "audio/player_death.wav")]
    player_death: Handle<AudioSource>,
    #[asset(path = "audio/enemy_walk.wav")]
    enemy_walk: Handle<AudioSource>,
    #[asset(path = "audio/enemy_hurt.wav")]
    enemy_hurt: Handle<AudioSource>,
    #[asset(path = "audio/enemy_death.wav")]
    enemy_death: Handle<AudioSource>,
    #[asset(path = "audio/xp_collect.wav")]
    xp_collect: Handle<AudioSource>,
}

impl GameAudioAssets {
    pub fn get(&self, sound: &SoundEffectType) -> Handle<AudioSource> {
        match sound {
            SoundEffectType::PlayerShoot => self.player_shoot.clone(),
            SoundEffectType::PlayerHurt => self.player_hurt.clone(),
            SoundEffectType::PlayerDeath => self.player_death.clone(),
            SoundEffectType::EnemyWalk => self.enemy_walk.clone(),
            SoundEffectType::EnemyHurt => self.enemy_hurt.clone(),
            SoundEffectType::EnemyDeath => self.enemy_death.clone(),
            SoundEffectType::XPCollect => self.xp_collect.clone(),
        }
    }
}
