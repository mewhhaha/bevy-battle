use bevy::{
    ecs::{component::Component, system::Resource},
    input::keyboard::KeyCode,
    math::Vec2,
};

#[derive(Default, Component)]
pub struct Id(pub i64);

#[derive(Default, Component)]
pub struct Interactive(pub bool);

#[derive(Default, Component)]
pub struct Player;

#[derive(Default, Component)]
pub struct VendingMachine;

#[derive(Default, Component)]
pub struct Speed(pub f32);

#[derive(Default, Component)]
pub struct Velocity(pub Vec2);

#[derive(Default, Component)]
pub struct SortY;

#[derive(Default, Clone, Copy)]
pub struct Frame {
    pub index: usize,
    pub duration: f32,
}

#[derive(Default, Component)]
pub struct Animation {
    /** Value that goes from 0 to 1 where 0 is start and 1 is finished */
    pub t: f32,
    pub looping: bool,
    pub frame_index: usize,
    pub frames: Vec<Frame>,
}
