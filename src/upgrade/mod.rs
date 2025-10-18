use bevy::prelude::*;

use crate::game::components::GameState;

pub mod components;
pub mod systems;

pub struct UpgradePlugin;

impl Plugin for UpgradePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), systems::setup_upgrades)
            .add_systems(
                OnEnter(GameState::Upgrade),
                systems::generate_upgrades_and_setup_menu,
            )
            .add_systems(
                Update,
                systems::handle_upgrade_selection.run_if(in_state(GameState::Upgrade)),
            )
            .add_systems(
                OnExit(GameState::Upgrade),
                systems::cleanup_upgrade_entities,
            );
    }
}
