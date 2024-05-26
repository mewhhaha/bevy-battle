use bevy::{
    asset::Handle,
    ecs::bundle::Bundle,
    prelude::default,
    render::{texture::Image, view::RenderLayers},
    text::{Text, TextStyle},
    ui::{
        node_bundles::{ButtonBundle, ImageBundle, NodeBundle, TextBundle},
        UiImage,
    },
};

use crate::helpers::LAYER_UI;

pub fn div(class: impl FnOnce(&mut NodeBundle)) -> impl Bundle {
    let mut bundle = NodeBundle::default();
    class(&mut bundle);

    (bundle, RenderLayers::layer(LAYER_UI))
}

pub fn text(class: impl FnOnce(&mut TextBundle), text: impl Into<String>) -> impl Bundle {
    let mut bundle = TextBundle {
        text: Text::from_section(text, TextStyle::default()),
        ..default()
    };

    class(&mut bundle);
    (bundle, RenderLayers::layer(LAYER_UI))
}

pub fn button(class: impl FnOnce(&mut ButtonBundle)) -> impl Bundle {
    let mut bundle = ButtonBundle::default();
    class(&mut bundle);

    (bundle, RenderLayers::layer(LAYER_UI))
}

pub fn img(class: impl FnOnce(&mut ImageBundle), src: Handle<Image>) -> impl Bundle {
    let mut bundle = ImageBundle::default();
    class(&mut bundle);

    bundle.image = UiImage::new(src);
    (bundle, RenderLayers::layer(LAYER_UI))
}
