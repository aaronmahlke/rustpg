use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct HurtTimer(pub Timer);
