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
}

impl Default for GameRules {
    fn default() -> Self {
        Self {
            xp: 0,
            level: 0,
            level_xp_base: 10,
            level_xp_multiplier: 1.0,
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
}
