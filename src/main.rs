use crate::components::*;
use crate::systems::*;

use bevy::{
    app::{App, Startup, Update},
    asset::{AssetMode, AssetPlugin},
    prelude::*,
};
mod components;
mod helpers;
mod systems;

fn startup_add_vending_machine(
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

    let idle_frames = helpers::create_frames(20, 20, 0.2);

    let animated_sprite = (
        Animation {
            name: "idle",
            time: 0.0,
            current: 0,
            frames: idle_frames,
        },
        sprite,
    );

    commands.spawn((VendingMachine, animated_sprite));
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    mode: AssetMode::Processed,
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_systems(
            Startup,
            (
                startup_add_player,
                startup_add_vending_machine,
                startup_add_camera,
                startup_add_keymap,
            ),
        )
        .add_systems(
            PreUpdate,
            (sort_y, animation_change_frame, sprite_face_velocity),
        )
        .add_systems(Update, (input_read, player_move).chain())
        .run();
}
