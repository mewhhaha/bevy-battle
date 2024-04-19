use crate::components::*;
use bevy::{ecs::system::Query, prelude::*};

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
