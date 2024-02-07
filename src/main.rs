use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    window::{PresentMode, WindowTheme},
};

use bevy_rapier2d::prelude::*;

mod fps;
use crate::fps::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "bevyaac".into(),
                        present_mode: PresentMode::Immediate,
                        prevent_default_event_handling: false,
                        window_theme: Some(WindowTheme::Dark),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins((FrameTimeDiagnosticsPlugin, FPSPlugin))
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, (setup_system, spawn_player, spawn_enemies))
        .add_systems(
            Update,
            (
                animate_player,
                move_player,
                flip_player,
                player_shoot,
                update_bullets,
                move_enemy,
                hurt_enemy,
            ),
        )
        .run();
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component, Deref, DerefMut)]
struct ShootTimer(Timer);

#[derive(Component, Deref, DerefMut)]
struct BulletDespawnTimer(Timer);

#[derive(Component)]
struct Bullet {
    direction: Vec3,
    speed: f32,
    size: f32,
}

#[derive(Component)]
struct Player {
    idle: AnimationIndices,
    walk: AnimationIndices,
    state: PlayerState,
    stats: PlayerStats,
}

struct PlayerStats {
    size: f32,
    shot_speed: f32,
    shot_damage: f32,
    move_speed: f32,
    health: i8,
}

struct PlayerState {
    moving: bool,
    facing: Vec3,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            idle: AnimationIndices {
                first: 241,
                last: 241,
            },
            walk: AnimationIndices {
                first: 242,
                last: 243,
            },
            state: PlayerState {
                moving: false,
                facing: Vec3::new(1.0, 0.0, 0.0),
            },
            stats: PlayerStats {
                size: 5.0,
                shot_speed: 0.4,
                shot_damage: 1.0,
                move_speed: 300.0,
                health: 10,
            },
        }
    }
}

#[derive(Component)]
struct Enemy {
    idle: AnimationIndices,
    walk: AnimationIndices,
    state: EnemyState,
    stats: EnemyStats,
}

struct EnemyState {
    moving: bool,
    facing: Vec3,
}

struct EnemyStats {
    size: f32,
    move_speed: f32,
    health: i8,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            idle: AnimationIndices {
                first: 321,
                last: 321,
            },
            walk: AnimationIndices {
                first: 322,
                last: 324,
            },
            state: EnemyState {
                moving: false,
                facing: Vec3::new(1.0, 0.0, 0.0),
            },
            stats: EnemyStats {
                size: 5.0,
                move_speed: 100.0,
                health: 20,
            },
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("monochrome_tilemap_transparent.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(16.0, 16.0),
        20,
        20,
        Some(Vec2::new(1.0, 1.0)),
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let player = Player::default();

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
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
    ));
}

fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("monochrome_tilemap_transparent.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(16.0, 16.0),
        20,
        20,
        Some(Vec2::new(1.0, 1.0)),
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let enemy = Enemy::default();

    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(enemy.idle.first),
                transform: Transform {
                    translation: Vec3::new(100.0, 100.0, 0.0),
                    scale: Vec3::splat(enemy.stats.size),
                    ..default()
                },
                ..default()
            },
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            LockedAxes::ROTATION_LOCKED,
            ActiveEvents::COLLISION_EVENTS,
            enemy,
        ))
        .insert((
            Collider::ball(Enemy::default().stats.size),
            RigidBody::Dynamic,
            Velocity::zero(),
        ));
}

fn setup_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
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

fn move_enemy(
    mut enemy_query: Query<(&mut Enemy, &Transform, &mut Velocity)>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let player_transform = player_query.single();
    for (enemy, transform, mut vel) in &mut enemy_query {
        let direction = player_transform.translation - transform.translation;
        let movement = direction.normalize();

        let move_delta = Vec2::new(movement.x, movement.y);

        vel.linvel = move_delta * enemy.stats.move_speed;
    }
}

fn hurt_enemy(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut enemy_query: Query<(Entity, &mut Enemy), With<Enemy>>,
    bullet_query: Query<Entity, With<Bullet>>,
    rapier_context: Res<RapierContext>,
) {
    for collision_event in collision_events.read() {
        for (enemy_entity, mut enemy) in &mut enemy_query {
            for bullet_entity in &bullet_query {
                if let Some(contact_pair) = rapier_context.contact_pair(bullet_entity, enemy_entity)
                {
                    // despan bullet
                    commands.entity(contact_pair.collider1()).despawn();

                    if enemy.stats.health > 0 {
                        enemy.stats.health -= 1;
                    } else {
                        commands.entity(enemy_entity).despawn();
                    }
                }
            }
        }
        println!("Received collision event: {:?}", collision_event);
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
                    speed: 1000.0,
                    size: 10.0,
                };

                // Rectangle
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(255., 255., 255.),
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
