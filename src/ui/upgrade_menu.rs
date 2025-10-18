use bevy::prelude::*;

use crate::{
    audio::components::{PlaySoundEffectEvent, SoundEffectType},
    game::components::GameState,
    upgrade::components::ShownUpgrades,
};

use super::components::{ButtonStyle, TagUpgradeMenu, UpgradeButtonAction};

#[derive(Bundle)]
pub struct UpgradeCardBundle {
    node: ButtonBundle,
}

impl UpgradeCardBundle {
    fn new() -> UpgradeCardBundle {
        UpgradeCardBundle {
            node: ButtonBundle {
                style: Style {
                    width: Val::Px(300.0),
                    height: Val::Px(500.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                background_color: Color::BLACK.into(),
                border_color: Color::WHITE.into(),
                ..default()
            },
        }
    }
}

pub fn setup_menu(mut commands: Commands, shown_upgrades_query: Query<&ShownUpgrades>) {
    if let Ok(shown_upgrades) = shown_upgrades_query.get_single() {
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        row_gap: Val::Px(20.0),
                        ..default()
                    },
                    ..default()
                },
                TagUpgradeMenu,
            ))
            .with_children(|parent| {
                // Title
                parent.spawn(TextBundle::from_section(
                    "CHOOSE AN UPGRADE",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));

                // Container for upgrade cards
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            column_gap: Val::Px(30.0),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        // Generate upgrade cards based on shown upgrades
                        for (i, upgrade) in shown_upgrades.upgrades.iter().enumerate() {
                            let button_action = match i {
                                0 => UpgradeButtonAction::Upgrade1,
                                1 => UpgradeButtonAction::Upgrade2,
                                2 => UpgradeButtonAction::Upgrade3,
                                _ => UpgradeButtonAction::Upgrade1, // fallback
                            };

                            parent
                                .spawn((UpgradeCardBundle::new(), button_action))
                                .with_children(|parent| {
                                    // Upgrade name
                                    parent.spawn(TextBundle::from_section(
                                        upgrade.name,
                                        TextStyle {
                                            font_size: 24.0,
                                            color: Color::WHITE,
                                            ..default()
                                        },
                                    ));

                                    // Upgrade effect display
                                    parent.spawn(TextBundle::from_section(
                                        upgrade.upgrade_type.get_description(),
                                        TextStyle {
                                            font_size: 20.0,
                                            color: Color::YELLOW,
                                            ..default()
                                        },
                                    ));

                                    // Upgrade description
                                    parent.spawn(TextBundle::from_section(
                                        upgrade.description,
                                        TextStyle {
                                            font_size: 16.0,
                                            color: Color::GRAY,
                                            ..default()
                                        },
                                    ));
                                });
                        }
                    });
            });
    } else {
        // Fallback if no upgrades are available (shouldn't happen normally)
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        column_gap: Val::Px(20.0),
                        ..default()
                    },
                    ..default()
                },
                TagUpgradeMenu,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "No upgrades available",
                    TextStyle {
                        font_size: 30.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });
    }
}

pub fn update_menu(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut sound_event: EventWriter<PlaySoundEffectEvent>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        for child in children {
            let text = text_query.get_mut(*child);
            if let Ok(mut text) = text {
                match *interaction {
                    Interaction::Pressed => {
                        *color = ButtonStyle::default().background.active;
                        text.sections[0].style.color = ButtonStyle::default().foreground.active;
                        sound_event.send(PlaySoundEffectEvent {
                            sound: SoundEffectType::UIEnter,
                        });
                        next_state.set(GameState::Playing);
                    }
                    Interaction::Hovered => {
                        *color = ButtonStyle::default().background.hover;
                        text.sections[0].style.color = ButtonStyle::default().foreground.hover;
                        sound_event.send(PlaySoundEffectEvent {
                            sound: SoundEffectType::UIHover,
                        });
                    }
                    Interaction::None => {
                        *color = ButtonStyle::default().background.default;
                        text.sections[0].style.color = ButtonStyle::default().foreground.default;
                    }
                }
            }
        }
    }
}

pub fn cleanup_menu(mut commands: Commands, menu_query: Query<Entity, With<TagUpgradeMenu>>) {
    for entity in &menu_query {
        commands.entity(entity).despawn_recursive();
    }
}
