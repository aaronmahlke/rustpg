use bevy::prelude::*;

#[derive(Component)]
pub struct TagGameUI;

#[derive(Component)]
pub struct TagMainMenu;

#[derive(Component)]
pub struct TagUpgradeMenu;

pub struct ButtonStyle {
    pub background: ButtonBackground,
    pub foreground: ButtonForeground,
}

pub struct ButtonBackground {
    pub default: BackgroundColor,
    pub hover: BackgroundColor,
    pub active: BackgroundColor,
}

pub struct ButtonForeground {
    pub default: Color,
    pub hover: Color,
    pub active: Color,
}

#[derive(Component)]
pub enum UpgradeButtonAction {
    Upgrade1,
    Upgrade2,
    Upgrade3,
}

impl Default for ButtonStyle {
    fn default() -> Self {
        ButtonStyle {
            background: ButtonBackground {
                default: Color::rgb(0.0, 0.0, 0.0).into(),
                hover: Color::rgb(1.0, 1.0, 1.0).into(),
                active: Color::rgb(0.5, 0.5, 0.5).into(),
            },
            foreground: ButtonForeground {
                default: Color::WHITE,
                hover: Color::BLACK,
                active: Color::BLACK,
            },
        }
    }
}
