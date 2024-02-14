use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::components::*;
use crate::{
    base::{components::*, resources::*},
    camera::components::Target,
    damagable::components::*,
    enemy::components::*,
    health::components::Health,
    hurt::components::*,
    xp::components::XPCollector,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player).add_systems(
            Update,
            (
                animate_player,
                move_player,
                flip_player,
                player_shoot,
                update_bullets,
                hurt_player,
                kill_player,
            ),
        );
    }
}

fn spawn_player(mut commands: Commands, sprite_sheet: Res<SpriteSheet>) {
    let player = Player::default();

    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: sprite_sheet.0.clone(),
                sprite: TextureAtlasSprite::new(player.idle.first),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    scale: Vec3::splat(player.stats.size),
                    ..Default::default()
                },
                ..default()
            },
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            ShootTimer(Timer::from_seconds(
                player.stats.shot_speed,
                TimerMode::Once,
            )),
            RigidBody::Dynamic,
            Velocity::zero(),
            Collider::ball(player.stats.size),
            Player::default(),
            ActiveEvents::COLLISION_EVENTS,
            LockedAxes::ROTATION_LOCKED,
            Damageable,
            Target,
            Health {
                max: 3.0,
                current: 3.0,
            },
        ))
        .with_children(|children| {
            children.spawn((
                Collider::ball(100.0),
                Sensor,
                XPCollector,
                ActiveEvents::COLLISION_EVENTS,
                ActiveHooks::FILTER_INTERSECTION_PAIR,
            ));
        });
}

fn animate_player(
    time: Res<Time>,
    mut query: Query<(&Player, &mut AnimationTimer, &mut TextureAtlasSprite)>,
) {
    for (player, mut timer, mut sprite) in &mut query {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            if player.state.moving {
                sprite.index = if sprite.index >= player.walk.last {
                    player.walk.first
                } else {
                    sprite.index + 1
                };
            } else {
                sprite.index = if sprite.index >= player.idle.last {
                    player.idle.first
                } else {
                    sprite.index + 1
                };
            }
        }
    }
}

fn flip_player(mut query: Query<(&Player, &mut Transform, &mut TextureAtlasSprite)>) {
    for (player, mut transform, _sprite) in &mut query {
        if player.state.facing.x < 0.0 {
            transform.scale.x = -player.stats.size;
        } else {
            transform.scale.x = player.stats.size;
        }
    }
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Velocity)>,
) {
    for (mut player, mut vel) in &mut query {
        let up = keyboard_input.any_pressed([KeyCode::W]);
        let down = keyboard_input.any_pressed([KeyCode::S]);
        let left = keyboard_input.any_pressed([KeyCode::A]);
        let right = keyboard_input.any_pressed([KeyCode::D]);

        let y_axis = -(left as i8) + right as i8;
        let x_axis = -(down as i8) + up as i8;

        let mut move_delta = Vec2::new(y_axis as f32, x_axis as f32);

        match y_axis {
            1 => {
                player.state.facing.x = 1.0;
            }
            -1 => {
                player.state.facing.x = -1.0;
            }
            _ => {}
        }

        if move_delta != Vec2::ZERO {
            player.state.moving = true;
            move_delta /= move_delta.length();
        } else {
            player.state.moving = false;
        }

        vel.linvel = move_delta * player.stats.move_speed;
    }
}

fn player_shoot(
    mut commands: Commands,
    mut query: Query<(&Player, &Transform, &mut ShootTimer)>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut shooting: bool = false;
    let mut direction: Vec3 = Vec3::ZERO;
    for (player, transform, mut shoot_timer) in &mut query {
        let offset = 10.0;
        let mut translation_with_offset = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::Left) {
            shooting = true;
            direction = Vec3::new(-1.0, 0.0, 0.0);
            translation_with_offset = transform.translation + Vec3::new(-offset, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) {
            shooting = true;
            direction = Vec3::new(1.0, 0.0, 0.0);
            translation_with_offset = transform.translation + Vec3::new(offset, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) {
            shooting = true;
            direction = Vec3::new(0.0, 1.0, 0.0);
            translation_with_offset = transform.translation + Vec3::new(0.0, offset, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) {
            shooting = true;
            direction = Vec3::new(0.0, -1.0, 0.0);
            translation_with_offset = transform.translation + Vec3::new(0.0, -offset, 0.0);
        }
        if shooting {
            let duration: f32 = shoot_timer.elapsed_secs();

            if duration >= player.stats.shot_speed {
                shoot_timer.reset();
                // spawn bullet
                let bullet = Bullet {
                    direction,
                    speed: 500.0,
                    size: 10.0,
                    damage: 2.0,
                };

                // Rectangle
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(255., 0., 0.),
                            custom_size: Some(Vec2::new(bullet.size, bullet.size)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(
                            translation_with_offset.x,
                            translation_with_offset.y,
                            0.0,
                        )),
                        ..default()
                    },
                    RigidBody::Dynamic,
                    Velocity::zero(),
                    Collider::ball(bullet.size),
                    BulletDespawnTimer(Timer::from_seconds(3.0, TimerMode::Once)),
                    ActiveEvents::COLLISION_EVENTS,
                    bullet,
                ));
            }
        }

        shoot_timer.tick(time.delta());
    }
}

fn update_bullets(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&Bullet, &mut BulletDespawnTimer, Entity, &mut Velocity)>,
) {
    for (bullet, mut despawn_timer, entity, mut vel) in &mut query {
        despawn_timer.0.tick(time.delta());

        if despawn_timer.0.just_finished() {
            commands.entity(entity).despawn();
        }

        let movement = bullet.direction * bullet.speed;
        let move_delta: Vec2 = Vec2::new(movement.x, movement.y);

        vel.linvel = move_delta;
    }
}

fn hurt_player(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut player_query: Query<(Entity, &mut Player), With<Player>>,
    enemy_query: Query<(Entity, &Enemy), With<Enemy>>,
    rapier_context: Res<RapierContext>,
) {
    for _ in collision_events.read() {
        for (player_entity, _player) in &mut player_query {
            for (enemy_entity, _enemy) in &enemy_query {
                if let Some(contact_pair) = rapier_context.contact_pair(player_entity, enemy_entity)
                {
                    commands.entity(player_entity).insert(Hurting(1.0));
                }
            }
        }
    }
}

fn kill_player(_commands: Commands, player_query: Query<(Entity, &Health), With<Player>>) {
    for (_entity, health) in &player_query {
        if health.current <= 0.0 {
            // commands.entity(entity).despawn()
            println!("Player is dead");
        }
    }
}
