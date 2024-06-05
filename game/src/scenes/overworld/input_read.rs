use bevy::{
    ecs::{
        query::With,
        system::{Query, Res},
    },
    input::{keyboard::KeyCode, ButtonInput},
    math::Vec2,
    prelude::*,
};

#[derive(Resource)]
pub struct KeyMap {
    pub move_up: KeyCode,
    pub move_down: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub interact: KeyCode,
}

#[derive(Event)]
pub struct OnPlayerInteract;

use crate::components::*;

pub fn input_read(
    keymap: Res<KeyMap>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Velocity, With<Player>>,
    mut on_interact: EventWriter<OnPlayerInteract>,
) {
    let mut direction = Vec2::default();

    if keyboard_input.pressed(keymap.move_up) {
        direction += Vec2::new(0.0, 1.0);
    }

    if keyboard_input.pressed(keymap.move_down) {
        direction += Vec2::new(0.0, -1.0);
    }

    if keyboard_input.pressed(keymap.move_left) {
        direction += Vec2::new(-1.0, 0.0);
    }

    if keyboard_input.pressed(keymap.move_right) {
        direction += Vec2::new(1.0, 0.0);
    }

    if keyboard_input.just_pressed(keymap.interact) {
        on_interact.send(OnPlayerInteract);
    }

    let mut player_velocity = player_query.single_mut();
    player_velocity.0 = direction.normalize_or_zero();
}

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(KeyMap {
            move_up: KeyCode::KeyW,
            move_down: KeyCode::KeyS,
            move_left: KeyCode::KeyA,
            move_right: KeyCode::KeyD,
            interact: KeyCode::Space,
        })
        .add_systems(PreUpdate, input_read)
        .add_event::<OnPlayerInteract>();
    }
}
