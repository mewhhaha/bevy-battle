use bevy::{
    asset::{Assets, Handle},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::system::{Commands, Query, ResMut},
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

use crate::materials::OutlineMaterial;

pub fn startup_add_camera(
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

    let mut image = Image {
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
    };

    image.resize(size);

    let handle = images.add(image);

    let material = materials.add(OutlineMaterial {
        texture: handle.clone(),
        color: Color::WHITE,
        thickness: 0.001,
    });

    let mesh = meshes
        .add(Mesh::from(Rectangle {
            half_size: Vec2::new(size.width as f32, size.height as f32) / 2.0,
        }))
        .into();

    commands.spawn(camera_interactive(handle.clone()));
    commands.spawn(overlay_interactive(mesh, material));

    commands.spawn((Camera2dBundle::default(), RenderLayers::layer(0)));
}

fn overlay_interactive(
    mesh: Handle<Mesh>,
    material: Handle<OutlineMaterial>,
) -> (MaterialMesh2dBundle<OutlineMaterial>,) {
    (MaterialMesh2dBundle {
        mesh: mesh.into(),
        material,
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 500.0),
            ..default()
        },
        ..default()
    },)
}

fn camera_interactive(handle: Handle<Image>) -> (Camera2dBundle, RenderLayers) {
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
        RenderLayers::layer(2),
    )
}
