use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct EnemySpawnTimer(pub Timer);
