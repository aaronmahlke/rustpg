use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

use super::components::*;
use super::resources::*;
use crate::base::components::WINDOW_PADDING;
use crate::base::{components::AnimationTimer, resources::*};
use crate::hurt::{components::*, resources::*};
use crate::player::components::{Bullet, Player};

const ENEMY_AMOUNT: u32 = 10;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_enemy_timer).add_systems(
            Update,
            (spawn_enemies, move_enemy, (hurt_enemy, kill_enemy).chain()),
        );
    }
}

fn setup_enemy_timer(mut commands: Commands) {
    commands.spawn(EnemySpawnTimer(Timer::from_seconds(
        2.0,
        TimerMode::Repeating,
    )));
}

fn spawn_enemies(
    mut commands: Commands,
    mut query: Query<&mut EnemySpawnTimer>,
    sprite_sheet: Res<SpriteSheet>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    let window = window_query.get_single().unwrap();
    for mut timer in query.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            let random_x = (rand::random::<f32>() + window.width() / 2.0 + WINDOW_PADDING)
                * if rand::random() { 1.0 } else { -1.0 };
            let random_y = (rand::random::<f32>() - window.height() / 2.0 + WINDOW_PADDING)
                * if rand::random() { 1.0 } else { -1.0 };
            // clamp to out of window
            println!("random: {}, {}", random_x, random_y);
            commands
                .spawn((
                    SpriteSheetBundle {
                        texture_atlas: sprite_sheet.0.clone(),
                        sprite: TextureAtlasSprite {
                            index: Enemy::default().idle.first,
                            color: Color::WHITE,
                            ..default()
                        },
                        transform: Transform {
                            translation: Vec3::new(random_x, random_y, 0.0),
                            scale: Vec3::splat(Enemy::default().stats.size),
                            ..default()
                        },
                        ..default()
                    },
                    AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                    Enemy::default(),
                    LockedAxes::ROTATION_LOCKED,
                    ActiveEvents::COLLISION_EVENTS,
                ))
                .insert((
                    Collider::ball(Enemy::default().stats.size),
                    RigidBody::Dynamic,
                    Velocity::zero(),
                ));
        }
    }
}

fn move_enemy(
    mut enemy_query: Query<(&mut Enemy, &Transform, &mut Velocity)>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let player_transform = player_query.single();
    for (mut enemy, transform, mut vel) in &mut enemy_query {
        let direction = player_transform.translation - transform.translation;
        let movement = direction.normalize();

        let move_delta = Vec2::new(movement.x, movement.y);

        // set moving state
        enemy.state.moving = move_delta != Vec2::ZERO;

        // set velocity
        vel.linvel = move_delta * enemy.stats.move_speed;
    }
}

fn hurt_enemy(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut enemy_query: Query<(Entity, &mut Enemy), With<Enemy>>,
    bullet_query: Query<(Entity, &Bullet), With<Bullet>>,
    rapier_context: Res<RapierContext>,
) {
    for _ in collision_events.read() {
        for (enemy_entity, mut enemy) in &mut enemy_query {
            for (bullet_entity, bullet) in &bullet_query {
                if let Some(_contact_pair) =
                    rapier_context.contact_pair(bullet_entity, enemy_entity)
                {
                    // despawn bullet
                    commands.entity(bullet_entity).despawn();
                    commands.entity(enemy_entity).insert(Hurting);
                    commands
                        .entity(enemy_entity)
                        .insert(HurtTimer(Timer::from_seconds(
                            HURT_DURATION,
                            TimerMode::Repeating,
                        )));

                    if enemy.stats.health > 0.0 {
                        enemy.stats.health -= bullet.damage;
                    }
                    println!("Enemy health: {}", enemy.stats.health);
                }
            }
        }
    }
}

fn kill_enemy(mut commands: Commands, enemy_query: Query<(Entity, &Enemy)>) {
    for (entity, enemy) in &enemy_query {
        if enemy.stats.health <= 0.0 {
            commands.entity(entity).despawn()
        }
    }
}