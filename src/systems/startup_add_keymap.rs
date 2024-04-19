use crate::components::*;
use bevy::{
    app::{App, Startup, Update},
    asset::{AssetMode, AssetPlugin, AssetServer},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        component::Component,
        query::With,
        schedule::IntoSystemConfigs,
        system::{Commands, Query, Res, Resource},
    },
    input::{keyboard::KeyCode, ButtonInput},
    math::Vec2,
    prelude::*,
    time::Time,
    transform::components::Transform,
};
pub fn startup_add_keymap(mut commands: Commands) {
    commands.insert_resource(KeyMap {
        move_up: KeyCode::KeyW,
        move_down: KeyCode::KeyS,
        move_left: KeyCode::KeyA,
        move_right: KeyCode::KeyD,
    });
}
