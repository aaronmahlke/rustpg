use crate::{
    audio::components::{PlaySoundEffectEvent, SoundEffectType},
    base::components::Collectable,
    game::components::GameState,
    player::components::Player,
};

use super::components::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct XPPlugin;

impl Plugin for XPPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_xp,
                (collect_xp, move_xp_to_player)
                    .chain()
                    .run_if(in_state(GameState::Game)),
            ),
        )
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
    mut xp_query: Query<Entity, With<XP>>,
    mut collector_query: Query<Entity, With<XPCollector>>,
    rapier_context: Res<RapierContext>,
) {
    for _ in collision_events.read() {
        for xp_entity in &mut xp_query {
            for collector_entity in &mut collector_query {
                if rapier_context.intersection_pair(collector_entity, xp_entity) == Some(true) {
                    commands.entity(xp_entity).insert(CollectionAnimation);
                }
            }
        }
    }
}

fn move_xp_to_player(
    mut commands: Commands,
    mut xp_query: Query<(Entity, &XP, &mut Transform), With<CollectionAnimation>>,
    mut player_query: Query<(&mut Player, &Transform), Without<XP>>,
    time: Res<Time>,
    mut sound_event: EventWriter<PlaySoundEffectEvent>,
) {
    for (xp_entity, xp, mut xp_transform) in &mut xp_query {
        for (mut player, player_transform) in &mut player_query {
            let acceleration = 5.0;
            let smoothness = 0.9;
            let direction = xp_transform.translation - player_transform.translation;
            let acceleration = direction * acceleration;
            let distance = direction.length();
            // Apply smoothing
            let velocity = acceleration * time.delta_seconds();
            let velocity = velocity * (distance / 50.0);

            // get faster as it gets closer to the player
            if distance > player.stats.size + 40.0 {
                xp_transform.translation -= velocity / smoothness;
                // let xp shrink as it gets closer to the player
                let scale = distance / 100.0;
                xp_transform.scale = Vec3::splat(scale);
            } else {
                sound_event.send(PlaySoundEffectEvent {
                    sound: SoundEffectType::XPCollect,
                });
                commands.entity(xp_entity).despawn();
                player.stats.xp += xp.0;
            }
        }
    }
}
