use bevy::prelude::*;

use super::components::*;
use crate::game::components::GameRules;
use crate::player::components::Player;

#[derive(Component)]
pub struct UIXPBar;

#[derive(Component)]
pub struct UIPlayerStats;

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

    // Player stats UI in bottom left
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(20.0),
                    left: Val::Px(20.0),
                    padding: UiRect::all(Val::Px(15.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(5.0),
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.8).into(),
                ..default()
            },
            TagGameUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Stats Loading...",
                    TextStyle {
                        font_size: 16.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                UIPlayerStats,
            ));
        });
}

pub fn update_ui(
    mut xp_query: Query<(&UIXPBar, &mut Style)>,
    mut stats_query: Query<&mut Text, With<UIPlayerStats>>,
    player_query: Query<&Player>,
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

    // Update player stats display
    if let (Ok(mut stats_text), Ok(player)) =
        (stats_query.get_single_mut(), player_query.get_single())
    {
        stats_text.sections[0].value = format!(
            "Level: {}\nDamage: {:.1}\nProjectile Speed: {:.0}\nFire Rate: {:.3}s\nMultishot: {}\nPiercing: {}\nPickup Range: {:.0}\nMove Speed: {:.0}",
            game.level,
            player.stats.bullet_damage,
            player.stats.bullet_speed,
            player.stats.shot_speed,
            player.stats.multishot,
            player.stats.piercing,
            player.stats.pickup_range,
            player.stats.move_speed
        );
    }
}
