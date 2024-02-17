use bevy::{prelude::*, sprite::Anchor};
use bevy_asepritesheet::prelude::*;
use bevy_rapier2d::prelude::*;

use super::components::*;
use crate::{
    audio::components::{PlaySoundEffectEvent, SoundEffectType},
    camera::components::Target,
    damagable::components::*,
    enemy::components::*,
    game::components::GameState,
    health::components::{Dead, Health},
    hurt::components::*,
    xp::components::XPCollector,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AsepritesheetPlugin::new(&["sprite.json"]))
            .add_systems(OnEnter(GameState::Game), spawn_player)
            .add_systems(
                Update,
                (
                    move_player,
                    animate_player,
                    flip_player,
                    player_shoot,
                    update_bullets,
                    hurt_player,
                    kill_player,
                )
                    .run_if(in_state(GameState::Game)),
            );
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let spritesheet_handle = load_spritesheet_then(
        &mut commands,
        &asset_server,
        "npc_4.sprite.json",
        Anchor::Center,
        |sheet| {
            let handle_death = sheet.get_anim_handle("death");

            if let Ok(anim_death) = sheet.get_anim_mut(&handle_death) {
                anim_death.end_action = AnimEndAction::Pause;
            }
        },
    );

    let player = Player::default();

    commands
        .spawn((
            SpatialBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)),
            Health {
                max: 3.0,
                current: 3.0,
            },
            ShootTimer(Timer::from_seconds(
                player.stats.shot_speed,
                TimerMode::Once,
            )),
            RigidBody::Dynamic,
            Velocity::zero(),
            ActiveEvents::COLLISION_EVENTS,
            LockedAxes::ROTATION_LOCKED,
            Damageable,
            Target,
            Player::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                Collider::ball(player.stats.size * 3.0),
                TransformBundle::from(Transform::from_xyz(0.0, -5.0, 0.0)),
                TagPlayer,
            ));

            parent.spawn((
                Collider::ball(100.0),
                Sensor,
                ActiveEvents::COLLISION_EVENTS,
                ActiveHooks::FILTER_INTERSECTION_PAIR,
                XPCollector,
                TagPlayer,
            ));

            parent.spawn((
                AnimatedSpriteBundle {
                    spritesheet: spritesheet_handle,
                    sprite_bundle: SpriteSheetBundle {
                        transform: Transform {
                            translation: Vec3::new(0.0, 0.0, 0.0),
                            scale: Vec3::splat(player.stats.size),
                            ..Default::default()
                        },
                        ..default()
                    },
                    ..default()
                },
                AnimEventSender,
                TagPlayer,
            ));
        });
}

fn flip_player(
    mut query: Query<(&Player, &Children), Without<Dead>>,
    mut sprite_query: Query<&mut Transform, With<TextureAtlasSprite>>,
) {
    for (player, children) in &mut query {
        for child in children {
            if let Ok(mut transform) = sprite_query.get_mut(*child) {
                if player.state.facing.x < 0.0 {
                    transform.scale.x = -player.stats.size;
                } else {
                    transform.scale.x = player.stats.size;
                }
            }
        }
    }
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Velocity), Without<Dead>>,
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

fn animate_player(
    mut q_animator: Query<&mut SpriteAnimator, With<TagPlayer>>,
    q_player: Query<(&Health, &Player, &Children)>,
) {
    for (health, player, children) in &q_player {
        for child in children {
            if let Ok(mut sprite_animator) = q_animator.get_mut(*child) {
                if health.current <= 0.0 {
                    sprite_animator.time_scale = 1.0;
                    sprite_animator.set_anim_index(4);
                } else if player.state.moving {
                    sprite_animator.time_scale = 1.0;
                    sprite_animator.set_anim_index(0);
                } else {
                    sprite_animator.time_scale = 1.0;
                    sprite_animator.set_anim_index(2);
                }
            }
        }
    }
}

fn player_shoot(
    mut commands: Commands,
    mut query: Query<(&Player, &Transform, &mut ShootTimer)>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<ColorMaterial>>,
    mut event_sound: EventWriter<PlaySoundEffectEvent>,
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
                // send player sound event
                event_sound.send(PlaySoundEffectEvent {
                    sound: SoundEffectType::PlayerShoot,
                });
                shoot_timer.reset();
                // spawn bullet
                let bullet = Bullet {
                    direction,
                    speed: 500.0,
                    size: 10.0,
                    damage: 10.0,
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
                    Damage(bullet.damage),
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
    mut player_collider_query: Query<Entity, With<TagPlayer>>,
    parent_query: Query<&Parent>,
    damage_query: Query<(Entity, &Damage), With<TagEnemy>>,
    rapier_context: Res<RapierContext>,
    mut sound_event: EventWriter<PlaySoundEffectEvent>,
) {
    for _ in collision_events.read() {
        for player_collider_entity in &mut player_collider_query {
            for (damage_entity, damage_source) in &damage_query {
                if let Some(_contact_pair) =
                    rapier_context.contact_pair(damage_entity, player_collider_entity)
                {
                    for parent in parent_query.iter_ancestors(player_collider_entity) {
                        sound_event.send(PlaySoundEffectEvent {
                            sound: SoundEffectType::PlayerHurt,
                        });
                        commands.entity(parent).insert(Hurting(damage_source.0));
                    }
                }
            }
        }
    }
}

fn kill_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Health, &mut Velocity), With<Player>>,
) {
    for (entity, health, mut velocity) in &mut player_query {
        if health.current <= 0.0 {
            // commands.entity(entity).despawn()
            commands.entity(entity).insert(Dead);
            commands.entity(entity).remove::<Collider>();

            velocity.linvel = Vec2::ZERO;

            println!("Player is dead");
        }
    }
}
