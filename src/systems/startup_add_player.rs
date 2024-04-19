use crate::components::*;
use bevy::{
    asset::AssetServer,
    ecs::system::{Commands, Res},
    math::Vec2,
    prelude::*,
};

pub fn startup_add_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load::<Image>("spritesheet.png");
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

    fn create_frames(from: usize, to: usize, duration: f32) -> Vec<Frame> {
        let mut frames = Vec::new();

        for i in from..=to {
            frames.push(Frame { index: i, duration });
        }

        frames
    }

    let idle_frames = create_frames(0, 2, 0.2);

    let animated_sprite = (
        Animation {
            name: "idle",
            time: 0.0,
            current: 0,
            frames: idle_frames,
        },
        sprite,
    );

    commands.spawn((Player, animated_sprite, Speed(200.0), Velocity::default()));
}
