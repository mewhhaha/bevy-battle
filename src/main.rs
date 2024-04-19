use bevy::{
    app::{App, Startup, Update},
    asset::{self, AssetMode, AssetPlugin, AssetServer},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        component::Component,
        query::With,
        schedule::IntoSystemConfigs,
        system::{Commands, Query, Res, Resource},
    },
    input::{keyboard::KeyCode, ButtonInput},
    math::Vec2,
    prelude::*,
    time::Time,
    transform::components::Transform,
};

#[derive(Default, Component)]
struct Player;

#[derive(Default, Component)]
struct Speed(f32);

#[derive(Default, Component)]
struct Velocity(Vec2);

#[derive(Default, Clone, Copy)]
struct Frame {
    index: usize,
    duration: f32,
}

#[derive(Default, Component)]
struct Animation {
    name: &'static str,
    time: f32,
    current: usize,
    frames: Vec<Frame>,
}

fn add_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn add_keymap(mut commands: Commands) {
    commands.insert_resource(KeyMap {
        move_up: KeyCode::KeyW,
        move_down: KeyCode::KeyS,
        move_left: KeyCode::KeyA,
        move_right: KeyCode::KeyD,
    });
}

fn add_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load::<Image>("spritesheet.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(32.0, 32.0), 8, 7, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let sprite = SpriteSheetBundle {
        texture,
        atlas: TextureAtlas {
            layout: texture_atlas_layout,
            index: 0,
        },
        ..default()
    };

    fn create_frames(from: usize, to: usize, duration: f32) -> Vec<Frame> {
        let mut frames = Vec::new();

        for i in from..=to {
            frames.push(Frame { index: i, duration });
        }

        frames
    }

    let idle_frames = create_frames(0, 2, 0.2);

    let animated_sprite = (
        Animation {
            name: "idle",
            time: 0.0,
            current: 0,
            frames: idle_frames,
        },
        sprite,
    );

    commands.spawn((Player, animated_sprite, Speed(200.0), Velocity::default()));
}

#[derive(Resource)]
struct KeyMap {
    move_up: KeyCode,
    move_down: KeyCode,
    move_left: KeyCode,
    move_right: KeyCode,
}

fn read_input(
    keymap: Res<KeyMap>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Velocity, With<Player>>,
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

    let mut player_velocity = player_query.single_mut();
    player_velocity.0 = direction.normalize_or_zero();
}

fn move_player(
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &Speed, &Velocity), With<Player>>,
) {
    let (mut transform, speed, velocity) = player_query.single_mut();
    let movement = velocity.0 * speed.0 * time.delta_seconds();

    if movement.length() > 0.0 {
        transform.translation += movement.extend(0.0);
    }
}

fn change_animation_frame(time: Res<Time>, mut query: Query<(&mut Animation, &mut TextureAtlas)>) {
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

fn face_velocity(mut query: Query<(&Velocity, &mut Sprite)>) {
    for (velocity, mut sprite) in query.iter_mut() {
        if velocity.0.x > 0.0 {
            sprite.flip_x = false;
        }

        if velocity.0.x < 0.0 {
            sprite.flip_x = true;
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            mode: AssetMode::Processed,
            ..default()
        }))
        .add_systems(Startup, (add_player, add_camera, add_keymap))
        .add_systems(Update, (read_input, move_player).chain())
        .add_systems(PostUpdate, (change_animation_frame, face_velocity))
        .run();
}
