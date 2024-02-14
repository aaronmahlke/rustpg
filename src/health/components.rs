use bevy::prelude::*;

#[derive(Component, PartialEq, PartialOrd)]
pub struct Health {
    pub max: f32,
    pub current: f32,
}
