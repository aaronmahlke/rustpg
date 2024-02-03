use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, (setup_system, spawn_player))
        .add_systems(
            Update,
            (
                animate_player,
                move_player,
                flip_player,
                player_shoot,
                update_bullets,
            ),
        )
        .run();
}

pub const MOVE_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 5.0;

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
    time: f32,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component, Deref, DerefMut)]
struct ShootTimer(Timer);

#[derive(Component)]
struct Bullet {
    direction: Vec3,
    speed: f32,
}

#[derive(Component)]
struct Player {
    idle: AnimationIndices,
    walk: AnimationIndices,
    state: PlayerState,
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
                time: 0.1,
            },
            walk: AnimationIndices {
                first: 242,
                last: 243,
                time: 0.1,
            },
            state: PlayerState {
                moving: false,
                facing: Vec3::new(1.0, 0.0, 0.0),
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
            transform: Transform::from_scale(Vec3::splat(PLAYER_SIZE)),
            ..default()
        },
        AnimationTimer(Timer::from_seconds(player.idle.time, TimerMode::Repeating)),
        ShootTimer(Timer::from_seconds(0.5, TimerMode::Once)),
        player,
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
    for (player, mut transform, mut sprite) in &mut query {
        if player.state.facing.x < 0.0 {
            transform.scale.x = -PLAYER_SIZE;
        } else {
            transform.scale.x = PLAYER_SIZE;
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
            transform.translation += movement * MOVE_SPEED * time.delta_seconds();
        } else {
            player.state.moving = false;
        }
    }
}

fn player_shoot(
    mut commands: Commands,
    mut query: Query<(&Player, &Transform, &mut ShootTimer)>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut shooting: bool = false;
    let mut direction: Vec3 = Vec3::ZERO;
    for (player, mut transform, mut shoot_timer) in &mut query {
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

            if duration >= 0.5 {
                shoot_timer.reset();
                // spawn bullet
                let bullet = Bullet {
                    direction,
                    speed: 1000.0,
                };

                // Rectangle
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(255., 255., 255.),
                            custom_size: Some(Vec2::new(5.0, 5.0)),
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
