use super::components::*;
use super::resources::*;
use bevy::prelude::*;

pub struct HurtPlugin;

impl Plugin for HurtPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (tick_hurt_timer, flash_sprite_red, stop_hurt));
    }
}

fn flash_sprite_red(mut query: Query<(&mut TextureAtlasSprite, Option<&Hurting>)>) {
    for (mut sprite, hurting) in query.iter_mut() {
        match hurting {
            Some(_) => {
                sprite.color = Color::RED;
            }
            None => {
                sprite.color = Color::WHITE;
            }
        }
    }
}

fn tick_hurt_timer(mut query: Query<&mut HurtTimer>, time: Res<Time>) {
    for mut timer in query.iter_mut() {
        timer.0.tick(time.delta());
    }
}

// fn start_hurt(mut commands: Commands, query: Query<Entity, With<Hurting>>) {
//     for entity in query.iter() {
//         commands
//             .entity(entity)
//             .insert(HurtTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
//     }
// }

fn stop_hurt(mut commands: Commands, query: Query<(Entity, &HurtTimer), With<Hurting>>) {
    for (entity, timer) in query.iter() {
        if timer.0.finished() {
            commands.entity(entity).remove::<Hurting>();
            commands.entity(entity).remove::<HurtTimer>();
        }
    }
}
