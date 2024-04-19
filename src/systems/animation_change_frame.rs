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
pub fn animation_change_frame(
    time: Res<Time>,
    mut query: Query<(&mut Animation, &mut TextureAtlas)>,
) {
    for (mut animation, mut texture_atlas) in query.iter_mut() {
        let mut frame = animation.frames[animation.current];

        animation.time += time.delta_seconds();
        while animation.time >= frame.duration {
            animation.current = (animation.current + 1) % animation.frames.len();
            frame = animation.frames[animation.current];
            animation.time = 0.0;
        }

        if texture_atlas.index != frame.index {
            texture_atlas.index = frame.index;
        }
    }
}
