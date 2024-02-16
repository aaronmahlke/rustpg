use bevy::prelude::*;

use crate::{gamestate::components::GameState, player::components::Player};

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup_ui)
            .add_systems(Update, update_ui.run_if(in_state(GameState::Game)));
    }
}

#[derive(Component)]
pub struct UIXPBar;

fn setup_ui(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
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
        })
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

fn update_ui(
    mut xp_query: Query<(&UIXPBar, &mut Style)>,
    player_query: Query<&Player>,
    time: Res<Time>,
) {
    for (_, mut style) in &mut xp_query {
        for player in &player_query {
            // tween the width of the xp bar so that it animates over time

            let xp_bar_width = style.width;
            let xp_bar_width = match xp_bar_width {
                Val::Percent(percent) => percent,
                _ => 0.0,
            };
            let target_width = player.stats.xp as f32 / 10.0 * 100.0;
            style.width =
                Val::Percent(xp_bar_width + (target_width - xp_bar_width) * time.delta_seconds());
        }
    }
}
