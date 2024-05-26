use bevy::{
    asset::Handle,
    ecs::bundle::Bundle,
    prelude::default,
    render::{color::Color, texture::Image, view::RenderLayers},
    text::{Text, TextStyle},
    ui::{
        node_bundles::{ButtonBundle, ImageBundle, NodeBundle, TextBundle},
        BackgroundColor, Display, FlexDirection, JustifyContent, Style, UiImage, Val, ZIndex,
    },
};

use crate::helpers::LAYER_UI;

macro_rules! cn {
    [$($name:expr),*] => {
        |mut _bundle| {$(
            $name(&mut _bundle)
        );*}
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
     ($name:ident, SetZIndex) => {
        impl SetZIndex for &mut $name {
            fn z_index(&mut self, z: i32) {
                self.z_index = ZIndex::Local(z);
            }
        }
    };
    ($name:ident, HasText) => {
        impl HasText for &mut $name {
            fn text(&mut self) -> &mut Text {
                &mut self.text
            }
        }
    };
    ($name:ident, $($tr:tt),*) => {
        $(style!($name, $tr);)*
    };
    () => {};
}

pub trait HasStyle {
    fn style(&mut self) -> &mut Style;
}

pub trait HasBackgroundColor {
    fn background_color(&mut self) -> &mut BackgroundColor;
}

pub trait SetZIndex {
    fn z_index(&mut self, z: i32);
}

pub trait HasText {
    fn text(&mut self) -> &mut Text;
}

style!(NodeBundle, HasStyle, HasBackgroundColor);
style!(ImageBundle, HasStyle, HasBackgroundColor, SetZIndex);
style!(ButtonBundle, HasStyle, HasBackgroundColor);
style!(TextBundle, HasText);

/** STYLES */

pub fn bg_black(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::BLACK;
}

pub fn bg_white(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::WHITE;
}

pub fn text_black(bundle: &mut impl HasText) {
    let text = &mut bundle.text();
    text.sections.iter_mut().for_each(|section| {
        section.style.color = Color::BLACK;
    });
}

pub fn _z_1(bundle: &mut impl SetZIndex) {
    bundle.z_index(-1);
}
