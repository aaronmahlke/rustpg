use bevy::gizmos;
use bevy::sprite::Anchor;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_asepritesheet::prelude::*;
use bevy_rapier2d::prelude::*;

use super::components::*;
use super::resources::*;

use crate::audio::components::{PlaySoundEffectEvent, SoundEffectType};
use crate::base::components::WINDOW_PADDING;
use crate::damagable::components::Damageable;
use crate::game::components::GameState;
use crate::health::components::{Dead, Health};
use crate::hurt::components::*;
use crate::particle::components::Particle;
use crate::player::components::{Bullet, Player};
use crate::xp::components::XPDropEvent;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_enemy_timer)
            .add_systems(OnExit(GameState::Playing), cleanup);

        // Playing
        app.add_systems(
            Update,
            ((
                spawn_enemies,
                move_enemy,
                hurt_enemy,
                flip_enemy,
                animate_enemy,
                kill_enemy,
                cleanup_dead,
            )
                .chain()
                .run_if(in_state(GameState::Playing)),),
        );

        // Upgrade
        app.add_systems(
            Update,
            (animate_enemy, pause_move, cleanup_dead).run_if(in_state(GameState::Upgrade)),
        );
    }
}

fn pause_move(mut query: Query<(&mut Velocity, &mut Enemy), With<Enemy>>) {
    for (mut vel, mut enemy) in &mut query {
        enemy.state.moving = false;
        enemy.state.attack = false;
        vel.linvel = Vec2::ZERO;
    }
}

fn setup_enemy_timer(mut commands: Commands) {
    commands.spawn(EnemySpawnTimer(Timer::from_seconds(
        0.2,
        TimerMode::Repeating,
    )));
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<EnemySpawnTimer>>) {
    for entity in &mut query.iter() {
        commands.entity(entity).despawn();
    }
}

fn spawn_enemies(
    mut commands: Commands,
    mut query: Query<&mut EnemySpawnTimer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut gizmos: Gizmos,
) {
    let spritesheet_handle = load_spritesheet_then(
        &mut commands,
        &asset_server,
        "enemy_3.sprite.json",
        Anchor::Center,
        |sheet| {
            let handle_death = sheet.get_anim_handle("death");

            if let Ok(anim_death) = sheet.get_anim_mut(&handle_death) {
                anim_death.end_action = AnimEndAction::Pause;
            }
        },
    );

    let window = window_query.get_single().unwrap();
    let (camera, camera_transform) = camera_query.single();
    for mut timer in &mut query {
        if timer.0.tick(time.delta()).just_finished() {
            let horizontal = rand::random::<bool>();

            let ver_flip = rand::random::<bool>();
            let hor_flip = rand::random::<bool>();

            let random_coordinates = if horizontal {
                let random_x = rand::random::<f32>() * window.width();
                let random_y = if ver_flip {
                    window.height() + WINDOW_PADDING
                } else {
                    -WINDOW_PADDING
                };
                Vec2::new(random_x, random_y)
            } else {
                let random_x = if hor_flip {
                    window.width() + WINDOW_PADDING
                } else {
                    -WINDOW_PADDING
                };
                let random_y = rand::random::<f32>() * window.height();
                Vec2::new(random_x, random_y)
            };
            let no_so_random = Vec2::new(window.width(), window.height());
            let Some(world_coordinates) =
                camera.viewport_to_world_2d(camera_transform, random_coordinates)
            else {
                return;
            };

            commands
                .spawn((
                    SpatialBundle::from(Transform::from_xyz(
                        world_coordinates.x,
                        world_coordinates.y,
                        0.5,
                    )),
                    RigidBody::Dynamic,
                    Damageable,
                    Health {
                        max: 10.0,
                        current: 10.0,
                    },
                    Velocity::zero(),
                    ActiveEvents::COLLISION_EVENTS,
                    LockedAxes::ROTATION_LOCKED,
                    Enemy::default(),
                    TagEnemy,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Collider::ball(15.0),
                        TransformBundle::from(Transform::from_xyz(0.0, -5.0, 0.0)),
                        Damage(1.0),
                        TagEnemy,
                    ));

                    parent.spawn((
                        AnimatedSpriteBundle {
                            spritesheet: spritesheet_handle.clone(),
                            sprite_bundle: SpriteSheetBundle {
                                transform: Transform {
                                    translation: Vec3::ZERO,
                                    scale: Vec3::splat(Enemy::default().stats.size),
                                    ..default()
                                },
                                ..default()
                            },
                            ..default()
                        },
                        AnimEventSender,
                        TagEnemy,
                    ));
                });
        }
    }
}

fn animate_enemy(
    mut q_animator: Query<&mut SpriteAnimator, With<TagEnemy>>,
    q_enemy: Query<(&Health, &Enemy, &Children)>,
) {
    for (health, enemy, children) in &q_enemy {
        for child in children {
            if let Ok(mut sprite_animator) = q_animator.get_mut(*child) {
                sprite_animator.time_scale = 1.0;

                if health.current <= 0.0 {
                    // death
                    sprite_animator.set_anim_index(4);
                } else if enemy.state.attack {
                    //attack
                    sprite_animator.set_anim_index(2);
                } else if enemy.state.moving {
                    // walk
                    sprite_animator.set_anim_index(0);
                } else {
                    // idle
                    sprite_animator.set_anim_index(1);
                }
            }
        }
    }
}

fn flip_enemy(
    query: Query<(&Enemy, &Children), Without<Dead>>,
    mut sprite_query: Query<&mut Transform, With<TextureAtlasSprite>>,
) {
    for (enemy, children) in &query {
        for children in children {
            if let Ok(mut sprite_transform) = sprite_query.get_mut(*children) {
                if enemy.state.facing.x < 0.0 {
                    sprite_transform.scale.x = -enemy.stats.size;
                } else {
                    sprite_transform.scale.x = enemy.stats.size;
                }
            }
        }
    }
}

fn move_enemy(
    mut enemy_query: Query<(&mut Enemy, &Transform, &mut Velocity, &Health)>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let player_transform = player_query.single();
    for (mut enemy, transform, mut vel, health) in &mut enemy_query {
        let direction = player_transform.translation - transform.translation;
        let movement = direction.normalize();
        let move_delta = Vec2::new(movement.x, movement.y);
        let distance = direction.length();

        // set moving state
        enemy.state.moving = move_delta != Vec2::ZERO;
        enemy.state.facing = Vec3::new(move_delta.x, move_delta.y, 0.0);

        // set attack state
        enemy.state.attack = distance < 60.0;

        // set velocity
        if health.current > 0.0 {
            vel.linvel = move_delta * enemy.stats.move_speed;
        } else {
            vel.linvel = Vec2::ZERO;
        }
    }
}

fn hurt_enemy(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut enemy_collider_query: Query<Entity, With<TagEnemy>>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
    parent_query: Query<&Parent, &Transform>,
    damage_query: Query<(Entity, &Damage), With<Bullet>>,
    rapier_context: Res<RapierContext>,
    mut sound_event: EventWriter<PlaySoundEffectEvent>,
) {
    for _ in collision_events.read() {
        for enemy_collider_entity in &mut enemy_collider_query {
            for (damage_entity, damage_source) in &damage_query {
                if let Some(contact_pair) =
                    rapier_context.contact_pair(damage_entity, enemy_collider_entity)
                {
                    for parent in parent_query.iter_ancestors(enemy_collider_entity) {
                        let enemy_transform = enemy_query.get(parent).unwrap();

                        // despawn damage source
                        commands.entity(damage_entity).despawn();
                        commands.entity(parent).insert(Hurting(damage_source.0));

                        let mut normal: Vec2 = Vec2::ZERO;

                        for manifold in contact_pair.manifolds() {
                            normal = manifold.normal();
                        }

                        // play sound effect
                        sound_event.send(PlaySoundEffectEvent {
                            sound: SoundEffectType::EnemyHurt,
                        });

                        // Spawn 3-5 particles in the opposite direction of the collision normal

                        let particle_amount = rand::random::<f32>() * 5.0 + 5.0;
                        for _ in 0..particle_amount as u32 {
                            let tangent = Vec2::new(normal.y, -normal.x);
                            let spread = rand::random::<f32>() * 2.0 - 1.0;
                            let speed = rand::random::<f32>() * 0.05;
                            let velocity = (normal + tangent * spread * 3.0) * speed;
                            let lifetime = rand::random::<f32>() * 0.5 + 0.3;

                            commands.spawn((
                                SpriteBundle {
                                    sprite: Sprite {
                                        custom_size: Some(Vec2::splat(3.0)),
                                        color: Color::rgba(1.0, 0.0, 0.0, 1.0),
                                        ..default()
                                    },
                                    transform: Transform {
                                        translation: Vec3::new(
                                            enemy_transform.translation.x,
                                            enemy_transform.translation.y,
                                            0.6,
                                        ),
                                        ..default()
                                    },
                                    ..default()
                                },
                                RigidBody::Dynamic,
                                Particle {
                                    initial_position: enemy_transform.translation,
                                    velocity,
                                    max_lifetime: lifetime,
                                    lifetime,
                                    ..default()
                                },
                            ));
                        }
                    }
                }
            }
        }
    }
}

fn kill_enemy(
    mut commands: Commands,
    mut enemy_query: Query<
        (Entity, &Health, &Transform, &mut Velocity, &Children),
        (With<Enemy>, Without<Dead>),
    >,
    mut event_drop_xp: EventWriter<XPDropEvent>,
) {
    for (entity, health, enemy_transform, mut vel, children) in &mut enemy_query {
        if health.current <= 0.0 {
            event_drop_xp.send(XPDropEvent {
                amount: 1,
                location: enemy_transform.translation,
            });
            commands.entity(entity).insert(Dead);

            for children in children {
                commands.entity(*children).remove::<Collider>();
            }

            vel.linvel = Vec2::ZERO;
        }
    }
}

fn cleanup_dead(
    mut commands: Commands,
    enemy_query: Query<Entity, (With<Enemy>, With<Dead>)>,
    parent_query: Query<&Parent, &Transform>,
    mut animation_events: EventReader<AnimFinishEvent>,
) {
    for event in animation_events.read() {
        for parent in parent_query.iter_ancestors(event.entity) {
            let enemy_entity = enemy_query.get(parent);
            if let Ok(entity) = enemy_entity {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
