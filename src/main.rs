use crate::components::*;
use crate::systems::*;

use bevy::{
    app::{App, Startup, Update},
    asset::{AssetMode, AssetPlugin},
    prelude::*,
};
mod components;
mod systems;

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
            (startup_add_player, startup_add_camera, startup_add_keymap),
        )
        .add_systems(Update, (input_read, player_move).chain())
        .add_systems(PostUpdate, (animation_change_frame, sprite_face_velocity))
        .run();
}
