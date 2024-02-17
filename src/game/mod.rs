pub mod components;
pub mod systems;

use crate::base::resources::SpriteSheetPlugin;
use crate::camera::systems::CameraPlugin;
use crate::enemy::systems::EnemyPlugin;
use crate::hurt::systems::HurtPlugin;
use crate::particle::systems::ParticlePlugin;
use crate::player::systems::PlayerPlugin;
use crate::ui::systems::GameUIPlugin;
use crate::xp::systems::XPPlugin;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SpriteSheetPlugin,
            PlayerPlugin,
            EnemyPlugin,
            HurtPlugin,
            CameraPlugin,
            XPPlugin,
            ParticlePlugin,
            GameUIPlugin,
        ))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
    }
}
