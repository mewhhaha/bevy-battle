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
    sprite::MaterialMesh2dBundle,
    transform::components::Transform,
    window::Window,
};

use crate::{
    helpers::{LAYER_INTERACTIVE, LAYER_POST_PROCESS, LAYER_UI, LAYER_WORLD},
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

    let size = size_window(window);

    let mut outline_image = new_image(size);
    outline_image.resize(size);

    let outline_handle = images.add(outline_image);

    let material = materials.add(outline_material(outline_handle.clone()));

    let mesh = meshes.add(mesh_rectangle(size)).into();

    commands.spawn(overlay_post_process(mesh, material));

    commands.spawn(camera_interactive(outline_handle.clone()));
    commands.spawn(camera_world(0));
    commands.spawn(camera_post_process(1));
    commands.spawn(camera_ui(2));
}

fn size_window(window: &Window) -> Extent3d {
    Extent3d {
        width: window.physical_width(),
        height: window.physical_height(),
        ..default()
    }
}

fn outline_material(handle: Handle<Image>) -> OutlineMaterial {
    OutlineMaterial {
        texture: handle,
        color: Color::WHITE,
        thickness: 0.001,
    }
}

fn mesh_rectangle(size: Extent3d) -> Mesh {
    Mesh::from(Rectangle {
        half_size: Vec2::new(size.width as f32, size.height as f32) / 2.0,
    })
}

fn overlay_post_process(mesh: Handle<Mesh>, material: Handle<OutlineMaterial>) -> impl Bundle {
    (
        MaterialMesh2dBundle {
            mesh: mesh.into(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            material,
            ..default()
        },
        RenderLayers::layer(LAYER_POST_PROCESS),
    )
}

fn camera_world(order: isize) -> impl Bundle {
    (
        Camera2dBundle {
            camera: Camera {
                order,
                clear_color: ClearColorConfig::Custom(Color::rgba_u8(0, 0, 0, 0)),
                ..default()
            },
            ..default()
        },
        RenderLayers::layer(LAYER_WORLD),
    )
}

fn camera_post_process(order: isize) -> impl Bundle {
    (
        Camera2dBundle {
            camera: Camera {
                order,
                clear_color: ClearColorConfig::None,
                ..default()
            },
            ..default()
        },
        RenderLayers::layer(LAYER_POST_PROCESS),
    )
}

fn camera_ui(order: isize) -> impl Bundle {
    (
        Camera2dBundle {
            camera: Camera {
                order,
                clear_color: ClearColorConfig::None,
                ..default()
            },
            ..default()
        },
        RenderLayers::layer(LAYER_UI),
    )
}

fn camera_interactive(handle: Handle<Image>) -> impl Bundle {
    (
        Camera2dBundle {
            camera: Camera {
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
