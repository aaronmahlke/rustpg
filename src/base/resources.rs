use bevy::prelude::*;

#[derive(Resource)]
pub struct SpriteSheet(pub Handle<TextureAtlas>);

pub struct SpriteSheetPlugin;

impl Plugin for SpriteSheetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpriteSheet>();
    }
}

impl FromWorld for SpriteSheet {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let texture_handle = asset_server.load("monochrome_tilemap_transparent.png");

        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(16.0, 16.0),
            20,
            20,
            Some(Vec2::new(1.0, 1.0)),
            None,
        );

        let mut texture_atlases = world.get_resource_mut::<Assets<TextureAtlas>>().unwrap();
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        Self(texture_atlas_handle)
    }
}
