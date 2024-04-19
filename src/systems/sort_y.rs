use bevy::{
    ecs::{query::With, system::Query},
    sprite::Sprite,
    transform::components::Transform,
};

pub fn sort_y(mut query: Query<&mut Transform, With<Sprite>>) {
    for mut transform in query.iter_mut() {
        transform.translation.z = -transform.translation.y;
    }
}
