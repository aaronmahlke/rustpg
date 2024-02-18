use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct PowerupsPlugin;

impl Plugin for PowerupsPlugin {
    fn build(&self, _app: &mut App) {}
}
