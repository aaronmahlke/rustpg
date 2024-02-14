use bevy::prelude::*;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Setup, setup_ui)
            .add_systems(Update, update_ui);
    }
}
