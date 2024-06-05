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
        let mut remaining_time = time.delta_seconds();

        while remaining_time > 0.0 {
            let current_frame = &animation.frames[animation.frame_index];
            let remaining_frame_time = (1.0 - animation.t) * current_frame.duration;

            if remaining_time < remaining_frame_time {
                animation.t += remaining_time / current_frame.duration;
                break;
            }

            remaining_time -= remaining_frame_time;
            animation.t = 0.0;

            let is_last_frame = animation.frame_index >= animation.frames.len() - 1;
            match (is_last_frame, animation.looping) {
                (true, true) => {
                    animation.frame_index = 0;
                    animation.t = 0.0;
                }
                (true, false) => {
                    animation.frame_index = animation.frames.len() - 1;
                    animation.t = 1.0;
                    break;
                }
                _ => {
                    animation.frame_index += 1;
                    animation.t = 0.0;
                }
            }
        }

        let current_frame = &animation.frames[animation.frame_index];
        if texture_atlas.index != current_frame.index {
            texture_atlas.index = current_frame.index;
        }
    }
}
