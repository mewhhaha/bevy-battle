use crate::components::*;
use bevy::{ecs::system::Commands, input::keyboard::KeyCode};

pub fn startup_add_keymap(mut commands: Commands) {
    commands.insert_resource(KeyMap {
        move_up: KeyCode::KeyW,
        move_down: KeyCode::KeyS,
        move_left: KeyCode::KeyA,
        move_right: KeyCode::KeyD,
    });
}
