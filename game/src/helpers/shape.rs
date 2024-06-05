use bevy::{
    asset::Handle,
    ecs::bundle::Bundle,
    math::Vec3,
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle},
    transform::components::Transform,
};

pub fn create_shape(
    mesh: Mesh2dHandle,
    color: Handle<ColorMaterial>,
    translation: Vec3,
) -> impl Bundle {
    MaterialMesh2dBundle {
        mesh,
        material: color,
        transform: Transform::from_translation(translation),
        ..Default::default()
    }
}
