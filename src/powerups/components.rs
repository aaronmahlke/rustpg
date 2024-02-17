use bevy::prelude::*;

struct Powerup {
    pub kind: PowerupType,
}

pub struct PowerupType {
    pub name: String,
    pub effect: Some(PowerupEffect),
}

pub enum PowerupEffect {
    MoveSpeed { value: f32 },
    ShotSpeed { value: f32 },
    Damage { value: f32 },
    SplitShot { value: u32 },
}
