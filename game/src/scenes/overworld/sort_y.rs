use bevy::{
    ecs::{query::With, system::Query},
    transform::components::Transform,
};

use crate::SortY;

pub fn sort_y(mut query: Query<&mut Transform, With<SortY>>) {
    for mut transform in query.iter_mut() {
        transform.translation.z = -100.0 - (transform.translation.y / 100.0);
    }
}
