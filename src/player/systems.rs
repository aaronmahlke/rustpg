use bevy::{prelude::*, sprite::Anchor, window::PrimaryWindow};
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
        // TODO: Move to assets plugin
        app.add_plugins(AsepritesheetPlugin::new(&["sprite.json"]));

        // Setup
        app.add_systems(OnEnter(GameState::Playing), spawn_player);

        // Playing state
        app.add_systems(
            Update,
            (
                cleanup_dead,
                kill_player,
                hurt_player,
                move_player,
                animate_player,
                flip_player,
                player_shoot,
                update_bullets,
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        );

        // Upgrade state
        app.add_systems(
            Update,
            (animate_player, update_bullets, pause_move).run_if(in_state(GameState::Upgrade)),
        );
    }
}

fn pause_move(mut query: Query<(&mut Velocity, &mut Player)>) {
    for (mut vel, mut player) in &mut query {
        player.state.moving = false;
        vel.linvel = Vec2::ZERO;
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_query: Query<Option<&Player>>,
) {
    for player in &player_query {
        if player.is_some() {
            return;
        }
    }

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
                CollisionGroups::new(
                    Group::GROUP_1,               // Player group
                    Group::ALL & !Group::GROUP_2, // Can collide with everything except bullets
                ),
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

                // Calculate cone spread for multishot
                let bullet_count = player.stats.multishot;
                let max_cone_angle = std::f32::consts::PI; // 180 degrees max cone

                for i in 0..bullet_count {
                    let mut bullet_direction = direction;

                    if bullet_count > 1 {
                        // Calculate spread angle - more bullets = wider cone, up to 180 degrees
                        let cone_angle = (bullet_count as f32 - 1.0) * (max_cone_angle / 10.0);
                        let cone_angle = cone_angle.min(max_cone_angle);

                        // Calculate angle offset for this bullet
                        let angle_step = if bullet_count > 1 {
                            cone_angle / (bullet_count - 1) as f32
                        } else {
                            0.0
                        };
                        let angle_offset = -cone_angle / 2.0 + i as f32 * angle_step;

                        // Rotate the direction vector
                        let cos_angle = angle_offset.cos();
                        let sin_angle = angle_offset.sin();

                        bullet_direction = Vec3::new(
                            direction.x * cos_angle - direction.y * sin_angle,
                            direction.x * sin_angle + direction.y * cos_angle,
                            0.0,
                        );
                    }

                    let bullet = Bullet {
                        direction: bullet_direction,
                        speed: player.stats.bullet_speed,
                        size: player.stats.bullet_damage * 5.0,
                        damage: player.stats.bullet_damage,
                        piercing: player.stats.piercing,
                        hits_remaining: player.stats.piercing + 1,
                    };

                    // Spawn bullet
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
                        CollisionGroups::new(
                            Group::GROUP_2,               // Bullet group
                            Group::ALL & !Group::GROUP_2, // Can collide with everything except other bullets
                        ),
                        BulletDespawnTimer(Timer::from_seconds(1.5, TimerMode::Once)),
                        ActiveEvents::COLLISION_EVENTS,
                        Damage(bullet.damage),
                        bullet,
                    ));
                }
            }
        }

        shoot_timer.tick(time.delta());
    }
}

fn update_bullets(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        &Bullet,
        &mut BulletDespawnTimer,
        Entity,
        &mut Velocity,
        &Transform,
    )>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok((camera, camera_transform)) = camera_query.get_single() else {
        return;
    };
    let Ok(window) = window_query.get_single() else {
        return;
    };

    for (bullet, mut despawn_timer, entity, mut vel, transform) in &mut query {
        despawn_timer.0.tick(time.delta());

        if despawn_timer.0.just_finished() {
            commands.entity(entity).despawn();
            continue;
        }

        // Despawn bullets that go off-screen relative to camera (performance optimization)
        let bullet_pos = transform.translation;
        let Some(bullet_screen_pos) = camera.world_to_viewport(camera_transform, bullet_pos) else {
            // If we can't project to screen, it's probably off-screen
            commands.entity(entity).despawn();
            continue;
        };

        // Add some padding beyond screen edges
        let padding = 200.0;
        if bullet_screen_pos.x < -padding
            || bullet_screen_pos.x > window.width() + padding
            || bullet_screen_pos.y < -padding
            || bullet_screen_pos.y > window.height() + padding
        {
            commands.entity(entity).despawn();
            continue;
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
    mut player_query: Query<(Entity, &Health, &mut Velocity), (With<Player>, Without<Dead>)>,
) {
    for (entity, health, mut velocity) in &mut player_query {
        if health.current <= 0.0 {
            // commands.entity(entity).despawn()
            commands.entity(entity).insert(Dead);
            commands.entity(entity).remove::<Collider>();
            commands.entity(entity).remove::<Hurting>();
            commands.entity(entity).remove::<Damageable>();

            velocity.linvel = Vec2::ZERO;
        }
    }
}

fn cleanup_dead(
    mut commands: Commands,
    player_query: Query<Entity, (With<Player>, With<Dead>)>,
    parent_query: Query<&Parent, &Transform>,
    mut animation_events: EventReader<AnimFinishEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in animation_events.read() {
        for parent in parent_query.iter_ancestors(event.entity) {
            let player_entity = player_query.get(parent);
            if let Ok(entity) = player_entity {
                println!("Player death animation finished");
                next_state.set(GameState::Menu);
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
