use bevy::prelude::*;

use crate::{
    game::components::GameState,
    player::components::{Player, PlayerStats},
    ui::components::UpgradeButtonAction,
};

use super::components::*;

pub fn setup_upgrades(mut commands: Commands) {
    commands.insert_resource(UpgradePool::new());
}

pub fn generate_upgrades_and_setup_menu(
    mut commands: Commands,
    upgrade_pool: Res<UpgradePool>,
    existing_upgrades: Query<Entity, With<ShownUpgrades>>,
) {
    for entity in &existing_upgrades {
        commands.entity(entity).despawn();
    }

    let random_upgrades = upgrade_pool.get_random_upgrades(3);

    // Create the menu directly with the upgrades
    commands
        .spawn((
            crate::ui::components::TagUpgradeMenu,
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
                    // Generate upgrade cards based on the upgrades
                    for (i, upgrade) in random_upgrades.iter().enumerate() {
                        let button_action = match i {
                            0 => crate::ui::components::UpgradeButtonAction::Upgrade1,
                            1 => crate::ui::components::UpgradeButtonAction::Upgrade2,
                            2 => crate::ui::components::UpgradeButtonAction::Upgrade3,
                            _ => crate::ui::components::UpgradeButtonAction::Upgrade1,
                        };

                        parent
                            .spawn((
                                ButtonBundle {
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
                                button_action,
                            ))
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

    // Also spawn the ShownUpgrades component for the selection system
    commands.spawn(ShownUpgrades {
        upgrades: random_upgrades,
    });
}

pub fn handle_upgrade_selection(
    upgrade_query: Query<&ShownUpgrades>,
    interaction_query: Query<(&Interaction, &UpgradeButtonAction), Changed<Interaction>>,
    mut player_query: Query<&mut Player>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            if let Ok(shown_upgrades) = upgrade_query.get_single() {
                let upgrade_index = match button_action {
                    UpgradeButtonAction::Upgrade1 => 0,
                    UpgradeButtonAction::Upgrade2 => 1,
                    UpgradeButtonAction::Upgrade3 => 2,
                };

                if let Some(chosen_upgrade) = shown_upgrades.upgrades.get(upgrade_index) {
                    // Apply the upgrade immediately instead of spawning a component
                    if let Ok(mut player) = player_query.get_single_mut() {
                        apply_upgrade_to_player(&mut player.stats, &chosen_upgrade.upgrade_type);

                        // Return to playing state
                        next_state.set(GameState::Playing);
                    }
                }
            }
        }
    }
}

// This system is no longer needed - upgrade application is handled directly in handle_upgrade_selection
fn apply_upgrade_to_player(player_stats: &mut PlayerStats, upgrade_type: &UpgradeType) {
    match upgrade_type {
        UpgradeType::DamageUp(amount) => {
            player_stats.bullet_damage += amount;
        }
        UpgradeType::ShotSpeedUp(amount) => {
            player_stats.shot_speed -= amount; // Lower value = faster shooting
                                               // Prevent shot speed from going below a minimum
            if player_stats.shot_speed < 0.05 {
                player_stats.shot_speed = 0.05;
            }
        }
        UpgradeType::BulletSpeedUp(amount) => {
            player_stats.bullet_speed += amount;
        }
        UpgradeType::PickupRangeUp(amount) => {
            player_stats.pickup_range += amount;
        }
        UpgradeType::MoveSpeedUp(amount) => {
            player_stats.move_speed += amount;
        }
        UpgradeType::MultishotUp(amount) => {
            player_stats.multishot += amount;
        }
        UpgradeType::PiercingUp(amount) => {
            player_stats.piercing += amount;
        }
    }
}

pub fn cleanup_upgrade_entities(
    mut commands: Commands,
    upgrade_entities: Query<Entity, Or<(With<ShownUpgrades>, With<ChosenUpgrade>)>>,
) {
    for entity in upgrade_entities.iter() {
        commands.entity(entity).despawn();
    }
}
