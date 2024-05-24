use bevy::{
    asset::Handle,
    ecs::bundle::Bundle,
    prelude::default,
    render::{color::Color, texture::Image, view::RenderLayers},
    text::{Text, Text2dBundle, TextStyle},
    ui::{
        node_bundles::{ImageBundle, NodeBundle},
        BackgroundColor, Display, Style, UiImage, Val,
    },
};

use crate::helpers::LAYER_UI;

macro_rules! cn {
    [$($name:expr),*] => {
        |mut bundle| {$({
            $name(&mut bundle)
        });*}
    };
}

macro_rules! style {
    ($name:ident, HasStyle) => {
        impl HasStyle for &mut $name {
            fn style(&mut self) -> &mut Style {
                &mut self.style
            }
        }
     };
     ($name:ident, HasBackgroundColor) => {
         impl HasBackgroundColor for &mut $name {
             fn background_color(&mut self) -> &mut BackgroundColor {
                 &mut self.background_color
             }
         }
     };
    ($name:ident, $($tr:tt),*) => {
        $(style!($name, $tr);)*
    };
    () => {};
}

pub fn div(class: impl FnOnce(&mut NodeBundle)) -> impl Bundle {
    let mut bundle = NodeBundle::default();
    class(&mut bundle);

    (bundle, RenderLayers::layer(LAYER_UI))
}

pub fn text(class: impl FnOnce(&mut Text2dBundle), text: impl Into<String>) -> impl Bundle {
    let mut bundle = Text2dBundle {
        text: Text::from_section(text, TextStyle::default()),
        ..default()
    };

    class(&mut bundle);
    (bundle, RenderLayers::layer(LAYER_UI))
}

pub fn img(class: impl FnOnce(&mut ImageBundle), src: Handle<Image>) -> impl Bundle {
    let mut bundle = ImageBundle::default();
    class(&mut bundle);

    bundle.image = UiImage::new(src);
    (bundle, RenderLayers::layer(LAYER_UI))
}

pub trait HasStyle {
    fn style(&mut self) -> &mut Style;
}

pub trait HasBackgroundColor {
    fn background_color(&mut self) -> &mut BackgroundColor;
}

style!(NodeBundle, HasStyle, HasBackgroundColor);
style!(ImageBundle, HasStyle, HasBackgroundColor);

pub fn flex(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.display = Display::Flex;
}

pub fn w_full(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.width = Val::Percent(100.0);
}

pub fn h_full(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.height = Val::Percent(100.0);
}

pub fn bg_black(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::BLACK;
}

pub fn bg_white(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::WHITE;
}
