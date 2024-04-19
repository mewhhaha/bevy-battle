use crate::components::*;
use bevy::{
    ecs::system::{Query, Res},
    prelude::*,
    time::Time,
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
