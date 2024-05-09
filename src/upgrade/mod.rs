use bevy::prelude::*;

use crate::game::components::GameState;

pub mod components;
pub mod systems;

pub struct UpgradePlugin;

impl Plugin for UpgradePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), systems::setup_upgrades);
    }
}
