use crate::{
    components::*,
    helpers::{create_frames, LAYER_WORLD},
};
use bevy::{
    asset::AssetServer,
    ecs::system::{Commands, Res},
    math::Vec2,
    prelude::*,
    render::view::RenderLayers,
};

pub fn startup_add_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load::<Image>("textures/spritesheet.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(32.0, 32.0), 8, 7, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let sprite = SpriteSheetBundle {
        texture,
        atlas: TextureAtlas {
            layout: texture_atlas_layout,
            index: 0,
        },
        ..default()
    };

    let idle_frames = create_frames(0, 2, 0.2);

    commands.spawn((
        Player,
        Animation::new(idle_frames),
        sprite,
        Speed(200.0),
        Velocity::default(),
        RangeInteraction(32.0),
        RenderLayers::layer(LAYER_WORLD),
    ));
}
