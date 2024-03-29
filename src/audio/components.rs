use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

#[derive(Resource)]
pub struct SoundEffectsAudioChannel;

#[derive(Event)]
pub struct PlaySoundEffectEvent {
    pub sound: SoundEffectType,
}

#[derive(Event)]
pub struct PlayMusicEvent {
    pub sound: MusicType,
    pub looping: bool,
    pub fade_in: u64,
}

#[derive(Event)]
pub struct StopSoundEvent;

impl Default for PlayMusicEvent {
    fn default() -> Self {
        Self {
            sound: MusicType::Menu,
            looping: false,
            fade_in: 0,
        }
    }
}

#[derive(Debug)]
pub enum SoundEffectType {
    PlayerShoot,
    PlayerHurt,
    PlayerDeath,
    EnemyWalk,
    EnemyHurt,
    EnemyDeath,
    XPCollect,
    UIHover,
    UIEnter,
    EnterLevelUp,
}

#[derive(Debug)]
pub enum MusicType {
    Game,
    Menu,
    Upgrade,
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
    #[asset(path = "audio/ui_hover.wav")]
    ui_hover: Handle<AudioSource>,
    #[asset(path = "audio/ui_enter.wav")]
    ui_press: Handle<AudioSource>,
    #[asset(path = "audio/music_game.wav")]
    music_game: Handle<AudioSource>,
    #[asset(path = "audio/music_menu.wav")]
    music_menu: Handle<AudioSource>,
    #[asset(path = "audio/music_upgrade.wav")]
    music_upgrade: Handle<AudioSource>,
    #[asset(path = "audio/enter_level_up.wav")]
    enter_level_up: Handle<AudioSource>,
}

impl GameAudioAssets {
    pub fn get_sound_effect(&self, sound: &SoundEffectType) -> Handle<AudioSource> {
        match sound {
            SoundEffectType::PlayerShoot => self.player_shoot.clone(),
            SoundEffectType::PlayerHurt => self.player_hurt.clone(),
            SoundEffectType::PlayerDeath => self.player_death.clone(),
            SoundEffectType::EnemyWalk => self.enemy_walk.clone(),
            SoundEffectType::EnemyHurt => self.enemy_hurt.clone(),
            SoundEffectType::EnemyDeath => self.enemy_death.clone(),
            SoundEffectType::XPCollect => self.xp_collect.clone(),
            SoundEffectType::UIHover => self.ui_hover.clone(),
            SoundEffectType::UIEnter => self.ui_press.clone(),
            SoundEffectType::EnterLevelUp => self.enter_level_up.clone(),
        }
    }

    pub fn get_music(&self, music: &MusicType) -> Handle<AudioSource> {
        match music {
            MusicType::Game => self.music_game.clone(),
            MusicType::Menu => self.music_menu.clone(),
            MusicType::Upgrade => self.music_upgrade.clone(),
        }
    }
}
