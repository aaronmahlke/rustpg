use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    Splash,
    Menu,
    Playing,
    Upgrade,
}

#[derive(Resource)]
pub struct GameRules {
    pub xp: u32,
    pub level: u32,
    // the amount of xp the player needs to advance
    pub level_xp_base: u32,
    // the amount of additonal xp the player neeeds per level (multiplied)
    pub level_xp_multiplier: f32,
    pub enemy_spawn_interval: f32,
    // enemy scaling factors (linear)
    pub enemy_health_base: f32,
    pub enemy_health_per_level: f32,
    pub enemy_damage_base: f32,
    pub enemy_damage_per_level: f32,
    pub enemy_speed_base: f32,
    pub enemy_speed_per_level: f32,
}

impl Default for GameRules {
    fn default() -> Self {
        Self {
            xp: 0,
            level: 0,
            level_xp_base: 10,
            level_xp_multiplier: 1.0,
            enemy_spawn_interval: 1.5,
            enemy_health_base: 3.0,
            enemy_health_per_level: 3.0,
            enemy_damage_base: 1.0,
            enemy_damage_per_level: 0.8,
            enemy_speed_base: 100.0,
            enemy_speed_per_level: 20.0,
        }
    }
}

impl GameRules {
    pub fn get_level_xp(&self) -> u32 {
        (self.level_xp_base as f32 * self.level_xp_multiplier) as u32
    }

    pub fn set_level(&mut self, level: u32) {
        self.level = level;
    }

    pub fn set_xp(&mut self, xp: u32) {
        self.xp = xp;
    }

    pub fn get_enemy_health(&self) -> f32 {
        self.enemy_health_base + (self.level as f32 * self.enemy_health_per_level)
    }

    pub fn get_enemy_damage(&self) -> f32 {
        self.enemy_damage_base + (self.level as f32 * self.enemy_damage_per_level)
    }

    pub fn get_enemy_speed(&self) -> f32 {
        self.enemy_speed_base + (self.level as f32 * self.enemy_speed_per_level)
    }

    pub fn reset(&mut self) {
        self.xp = 0;
        self.level = 0;
        self.level_xp_multiplier = 1.0;
        self.enemy_spawn_interval = 1.5;
        self.enemy_health_base = 3.0;
        self.enemy_health_per_level = 3.0;
        self.enemy_damage_base = 1.0;
        self.enemy_damage_per_level = 0.8;
        self.enemy_speed_base = 100.0;
        self.enemy_speed_per_level = 20.0;
    }
}
