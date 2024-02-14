use bevy::prelude::*;

#[derive(Component)]
pub struct XP(pub u32);

#[derive(Component)]
pub struct XPDropped {
    pub amount: u32,
    pub location: Vec3,
}

#[derive(Event)]
pub struct XPDropEvent {
    pub amount: u32,
    pub location: Vec3,
}
