use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct UpgradePlugin;

impl Plugin for UpgradePlugin {
    fn build(&self, _app: &mut App) {}
}
