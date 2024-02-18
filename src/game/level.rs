use super::components::{GameRules, GameState};
use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, level_up.run_if(in_state(GameState::Playing)));
    }
}

fn level_up(mut game: ResMut<GameRules>) {
    if game.xp >= game.get_level_xp() {
        let current_level = game.level;
        game.set_level(current_level + 1);
        game.level_xp_multiplier += 0.2;
        game.xp = 0;
    }
}
