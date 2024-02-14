use crate::{base::components::Collectable, player::components::Player};

use super::components::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct XPPlugin;

impl Plugin for XPPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_xp, collect_xp))
            .add_event::<XPDropEvent>();
    }
}

fn spawn_xp(mut commands: Commands, mut event_xp_dropped: EventReader<XPDropEvent>) {
    for event in event_xp_dropped.read() {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    color: Color::Rgba {
                        red: (1.0),
                        green: (1.0),
                        blue: (1.0),
                        alpha: (0.5),
                    },
                    ..default()
                },
                transform: Transform::from_translation(event.location),
                ..default()
            },
            XP(event.amount),
            Collectable,
            Collider::ball(100.0),
            Sensor,
        ));
    }
}

fn collect_xp(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut xp_query: Query<(Entity, &mut XP), With<Collectable>>,
    mut player_query: Query<(Entity, &mut Player)>,
    rapier_context: Res<RapierContext>,
) {
    for _ in collision_events.read() {
        for (xp_entity, xp) in &mut xp_query {
            for (player_entity, mut player) in &mut player_query {
                if let Some(_contact_pair) = rapier_context.contact_pair(player_entity, xp_entity) {
                    // despawn bullet
                    commands.entity(xp_entity).despawn();
                    player.stats.xp += xp.0;
                }
            }
        }
    }
}
