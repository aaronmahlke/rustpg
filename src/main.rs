use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    window::{PresentMode, WindowTheme},
};

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
                health: 3,
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
            transform: Transform::from_scale(Vec3::splat(player.stats.size)),
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ShootTimer(Timer::from_seconds(
            player.stats.shot_speed,
            TimerMode::Once,
        )),
        player,
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

    commands.spawn((
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
        enemy,
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
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Transform)>,
) {
    for (mut player, mut transform) in &mut query {
        let mut movement = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::W) {
            movement.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::S) {
            movement.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::A) {
            player.state.facing.x = -1.0;
            movement.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::D) {
            player.state.facing.x = 1.0;
            movement.x += 1.0;
        }
        if movement.length() > 0.0 {
            player.state.moving = true;
            player.state.facing = movement.normalize();
            transform.translation += movement * player.stats.move_speed * time.delta_seconds();
        } else {
            player.state.moving = false;
        }
    }
}

fn move_enemy(
    time: Res<Time>,
    mut query: Query<(&mut Enemy, &mut Transform)>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let player_transform = player_query.single();
    for (enemy, mut transform) in &mut query {
        let direction = player_transform.translation - transform.translation;
        let movement = direction.normalize();
        transform.translation += movement * enemy.stats.move_speed * time.delta_seconds();
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
        if keyboard_input.pressed(KeyCode::Left) {
            shooting = true;
            direction = Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) {
            shooting = true;
            direction = Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) {
            shooting = true;
            direction = Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) {
            shooting = true;
            direction = Vec3::new(0.0, -1.0, 0.0);
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
                            transform.translation.x,
                            transform.translation.y,
                            0.0,
                        )),
                        ..default()
                    },
                    bullet,
                ));
            }
        }

        shoot_timer.tick(time.delta());
    }
}

fn update_bullets(time: Res<Time>, mut query: Query<(&Bullet, &mut Transform)>) {
    for (bullet, mut transform) in &mut query {
        transform.translation += bullet.direction * bullet.speed * time.delta_seconds();
    }
}

fn enemy_chase_player(
    mut commands: Commands,
    time: Res<Time>,
    mut eneym_query: Query<(&Transform, &mut Enemy)>,
    player_query: Query<&Transform, With<Player>>,
) {
}
