use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    Splash,
    Menu,
    #[default]
    Game,
    Upgrade,
}

#[derive(Resource)]
pub struct GameRules {
    pub level: u32,
    pub level_multiplier: f32,
}

impl Default for GameRules {
    fn default() -> Self {
        Self {
            level: 0,
            level_multiplier: 1.5,
        }
    }
}
