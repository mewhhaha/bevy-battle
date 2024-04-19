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
pub fn sprite_face_velocity(mut query: Query<(&Velocity, &mut Sprite)>) {
    for (velocity, mut sprite) in query.iter_mut() {
        if velocity.0.x > 0.0 {
            sprite.flip_x = false;
        }

        if velocity.0.x < 0.0 {
            sprite.flip_x = true;
        }
    }
}
