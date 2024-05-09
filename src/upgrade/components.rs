use bevy::prelude::*;

pub struct Upgrade {
    pub name: String,
    pub description: String,
    pub upgrade_type: UpgradeType,
}

pub struct UpgradeType {
    damage: f32,
    move_speed: f32,
    shot_speed: f32,
    effect: EffectType,
}

pub enum EffectType {
    Fire,
    Ice,
}
