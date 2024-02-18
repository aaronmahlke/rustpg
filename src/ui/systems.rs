use bevy::prelude::*;

use super::components::*;
use crate::game::components::GameRules;

#[derive(Component)]
pub struct UIXPBar;

pub fn despawn_game_ui(mut commands: Commands, ui_query: Query<Entity, With<TagGameUI>>) {
    for entity in &ui_query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn setup_game_ui(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(50.0),
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.0),
                    padding: UiRect {
                        left: Val::Px(20.0),
                        right: Val::Px(20.0),
                        top: Val::Px(20.0),
                        bottom: Val::Px(20.0),
                    },
                    ..default()
                },
                ..default()
            },
            TagGameUI,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.1, 0.1, 0.1).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(0.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            background_color: Color::rgb(1.0, 1.0, 1.0).into(),
                            ..default()
                        },
                        UIXPBar,
                    ));
                });
        });
}

pub fn update_ui(
    mut xp_query: Query<(&UIXPBar, &mut Style)>,
    game: Res<GameRules>,
    time: Res<Time>,
) {
    for (_, mut style) in &mut xp_query {
        // tween the width of the xp bar so that it animates over time

        let xp_bar_width = style.width;
        let xp_bar_width = match xp_bar_width {
            Val::Percent(percent) => percent,
            _ => 0.0,
        };
        let target_width = game.xp as f32 / game.get_level_xp() as f32 * 100.0;
        style.width =
            Val::Percent(xp_bar_width + (target_width - xp_bar_width) * time.delta_seconds());
    }
}
