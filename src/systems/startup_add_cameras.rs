use bevy::{
    asset::{Assets, Handle},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        bundle::Bundle,
        system::{Commands, Query, ResMut},
    },
    math::{primitives::Rectangle, Vec2, Vec3},
    prelude::default,
    render::{
        camera::{Camera, ClearColorConfig, RenderTarget},
        color::Color,
        mesh::Mesh,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        texture::Image,
        view::RenderLayers,
    },
    sprite::{MaterialMesh2dBundle, SpriteBundle},
    transform::components::Transform,
    window::Window,
};

use crate::{
    helpers::{LAYER_INTERACTIVE, LAYER_OUTLINE, LAYER_POST_PROCESS, LAYER_WORLD},
    materials::OutlineMaterial,
};

pub fn startup_add_cameras(
    mut commands: Commands,
    windows: Query<&Window>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<OutlineMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let window = windows.single();

    let size = Extent3d {
        width: window.physical_width(),
        height: window.physical_height(),
        ..default()
    };
    let mut outline_image = new_image(size);
    outline_image.resize(size);

    let mut world_image = new_image(size);
    world_image.resize(size);

    let outline_handle = images.add(outline_image);
    let world_handle = images.add(world_image);

    let material = materials.add(OutlineMaterial {
        texture: outline_handle.clone(),
        color: Color::WHITE,
        thickness: 0.001,
    });

    let mesh = meshes
        .add(Mesh::from(Rectangle {
            half_size: Vec2::new(size.width as f32, size.height as f32) / 2.0,
        }))
        .into();

    commands.spawn(camera_interactive(outline_handle.clone()));
    commands.spawn(camera_world(world_handle.clone()));
    commands.spawn(overlay_post_process(mesh, material));
    commands.spawn(overlay_world(world_handle.clone()));
    commands.spawn((
        Camera2dBundle::default(),
        RenderLayers::layer(LAYER_POST_PROCESS),
    ));
}

fn overlay_world(handle: Handle<Image>) -> impl Bundle {
    (
        SpriteBundle {
            texture: handle.into(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        },
        RenderLayers::layer(LAYER_POST_PROCESS),
    )
}

fn overlay_post_process(mesh: Handle<Mesh>, material: Handle<OutlineMaterial>) -> impl Bundle {
    (
        MaterialMesh2dBundle {
            mesh: mesh.into(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
            material,
            ..default()
        },
        RenderLayers::layer(LAYER_POST_PROCESS),
    )
}

fn camera_world(handle: Handle<Image>) -> impl Bundle {
    (
        Camera2dBundle {
            camera: Camera {
                order: 0,
                target: RenderTarget::Image(handle),
                clear_color: ClearColorConfig::Custom(Color::rgba_u8(0, 0, 0, 0)),
                ..default()
            },

            ..default()
        },
        RenderLayers::from_layers(&[LAYER_WORLD, LAYER_OUTLINE]),
    )
}

fn camera_interactive(handle: Handle<Image>) -> impl Bundle {
    (
        Camera2dBundle {
            camera: Camera {
                order: -1,
                target: RenderTarget::Image(handle),
                clear_color: ClearColorConfig::Custom(Color::rgba_u8(0, 0, 0, 0)),
                ..default()
            },

            ..default()
        },
        RenderLayers::layer(LAYER_INTERACTIVE),
    )
}

fn new_image(size: Extent3d) -> Image {
    Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    }
}
