use crate::game::components::GameState;

use super::components::*;
use bevy::prelude::*;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_particle.run_if(in_state(GameState::Game)));
    }
}

fn update_particle(
    mut commands: Commands,
    mut particle_query: Query<(Entity, &mut Particle, &mut Transform, &mut Sprite)>,
    time: Res<Time>,
) {
    for (entity, mut particle, mut transform, mut sprite) in &mut particle_query {
        particle.lifetime -= time.delta_seconds();

        let translation = transform.translation;

        // Apply gravity
        let mut new_translation = translation + particle.velocity.extend(0.0);

        // fade out the particle at end of its lifetime
        sprite.color = Color::rgba(
            sprite.color.r(),
            sprite.color.g(),
            sprite.color.b(),
            particle.lifetime / particle.max_lifetime,
        );

        if particle.lifetime <= 0.0 {
            // Despawn particles that have run out of lifetime
            commands.entity(entity).despawn();
        }

        // apply gravity
        new_translation.y += particle.gravity * 10.0 * time.delta_seconds();

        // bounce off the ground
        let ground = 50.0;
        if new_translation.y < particle.initial_position.y - ground {
            new_translation.y = particle.initial_position.y - ground;
            particle.velocity.y = -particle.velocity.y * 0.7;
        }

        // update the particle's position
        transform.translation = new_translation;
    }
}
