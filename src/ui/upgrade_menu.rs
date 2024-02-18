use bevy::prelude::*;

use crate::{
    audio::components::{PlaySoundEffectEvent, SoundEffectType},
    game::components::GameState,
};

use super::components::{ButtonStyle, TagUpgradeMenu};

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

pub fn setup_menu(mut commands: Commands) {
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
            parent
                .spawn(UpgradeCardBundle::new())
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Upgrade 1",
                        TextStyle {
                            font_size: 30.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });

            parent
                .spawn(UpgradeCardBundle::new())
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Upgrade 2",
                        TextStyle {
                            font_size: 30.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });

            parent
                .spawn(UpgradeCardBundle::new())
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Upgrade 3",
                        TextStyle {
                            font_size: 30.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                });
        });
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
