pub mod components;
pub mod main_menu;
pub mod systems;

use crate::game::components::GameState;
use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        // Game UI
        app.add_systems(OnEnter(GameState::Playing), systems::setup_game_ui)
            .add_systems(OnExit(GameState::Playing), systems::despawn_game_ui)
            .add_systems(
                Update,
                systems::update_ui.run_if(in_state(GameState::Playing)),
            );

        // Main Menu
        app.add_systems(OnEnter(GameState::Menu), main_menu::setup_menu)
            .add_systems(
                Update,
                main_menu::update_menu.run_if(in_state(GameState::Menu)),
            )
            .add_systems(OnExit(GameState::Menu), main_menu::cleanup_menu);
    }
}
