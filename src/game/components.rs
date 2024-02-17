use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    Splash,
    Menu,
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
