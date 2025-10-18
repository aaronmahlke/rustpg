use super::components::{GameRules, GameState};
use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, level_up.run_if(in_state(GameState::Playing)));
    }
}

fn level_up(mut game: ResMut<GameRules>, mut next_state: ResMut<NextState<GameState>>) {
    if game.xp >= game.get_level_xp() {
        next_state.set(GameState::Upgrade);
        let current_level = game.level;
        game.set_level(current_level + 1);
        game.level_xp_multiplier += 0.2;
        game.xp = 0;

        //subtract more in the beginning to make the game harder faster
        //but don't go below minimum to prevent crashes
        if current_level < 3 {
            game.enemy_spawn_interval -= 0.4;
        } else if current_level < 8 {
            game.enemy_spawn_interval -= 0.2;
        } else {
            game.enemy_spawn_interval -= 0.1;
        }

        // Ensure spawn interval never goes below 0.05 seconds
        if game.enemy_spawn_interval < 0.05 {
            game.enemy_spawn_interval = 0.05;
        }
    }
}
