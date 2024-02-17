use crate::damagable::components::Damageable;
use crate::game::components::GameState;
use crate::health::components::Health;

use super::components::*;
use super::resources::*;
use bevy::prelude::*;

pub struct HurtPlugin;

impl Plugin for HurtPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                start_hurt,
                flash_sprite_red,
                tick_hurt_timer,
                apply_damage,
                stop_hurt,
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn flash_sprite_red(
    mut query: Query<(Option<&Hurting>, &Children)>,
    mut sprite_query: Query<&mut TextureAtlasSprite>,
) {
    for (hurting, children) in query.iter_mut() {
        match hurting {
            Some(_) => {
                for child in children {
                    if let Ok(mut sprite) = sprite_query.get_mut(*child) {
                        sprite.color = Color::RED;
                    }
                }
            }
            None => {
                for child in children.iter() {
                    if let Ok(mut sprite) = sprite_query.get_mut(*child) {
                        sprite.color = Color::WHITE;
                    }
                }
            }
        }
    }
}

fn tick_hurt_timer(mut query: Query<&mut HurtTimer>, time: Res<Time>) {
    for mut timer in query.iter_mut() {
        timer.0.tick(time.delta());
    }
}

fn start_hurt(mut commands: Commands, query: Query<Entity, (Added<Hurting>)>) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(HurtTimer(Timer::from_seconds(
                HURT_DURATION,
                TimerMode::Once,
            )));
    }
}

fn apply_damage(mut query: Query<(&mut Health, &Hurting, &HurtTimer), With<Damageable>>) {
    for (mut health, hurting, timer) in query.iter_mut() {
        if timer.0.just_finished() {
            health.current -= hurting.0;
        }
    }
}

fn stop_hurt(mut commands: Commands, query: Query<(Entity, &HurtTimer), With<Hurting>>) {
    for (entity, timer) in query.iter() {
        if timer.0.finished() {
            commands.entity(entity).remove::<Hurting>();
            commands.entity(entity).remove::<HurtTimer>();
        }
    }
}
