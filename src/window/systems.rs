use bevy::{
    prelude::*,
    window::{PresentMode, WindowTheme},
};

pub struct CustomWindowPlugin;

impl Plugin for CustomWindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "bevyaac".into(),
                        present_mode: PresentMode::Immediate,
                        prevent_default_event_handling: false,
                        window_theme: Some(WindowTheme::Dark),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        );
    }
}
