use bevy::{
    ecs::{component::Component, system::Resource},
    input::keyboard::KeyCode,
    math::Vec2,
};

#[derive(Default, Component)]
pub struct Interactive(pub bool);

#[derive(Default, Component)]
pub struct RangeInteraction(pub f32);

#[derive(Default, Component)]
pub struct Player;

#[derive(Default, Component)]
pub struct VendingMachine;

#[derive(Default, Component)]
pub struct Speed(pub f32);

#[derive(Default, Component)]
pub struct Velocity(pub Vec2);

#[derive(Default, Clone, Copy)]
pub struct Frame {
    pub index: usize,
    pub duration: f32,
}

#[derive(Default, Component)]
pub struct Animation {
    pub name: &'static str,
    pub time: f32,
    pub current: usize,
    pub frames: Vec<Frame>,
}

#[derive(Resource)]
pub struct KeyMap {
    pub move_up: KeyCode,
    pub move_down: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
}
