use crate::components::*;
use bevy::{
    ecs::{
        query::With,
        system::{Query, Res},
    },
    time::Time,
    transform::components::Transform,
};
pub fn player_move(
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &Speed, &Velocity), With<Player>>,
) {
    let (mut transform, speed, velocity) = player_query.single_mut();
    let movement = velocity.0 * speed.0 * time.delta_seconds();

    if movement.length() > 0.0 {
        transform.translation += movement.extend(0.0);
    }
}
