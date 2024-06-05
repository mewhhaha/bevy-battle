use bevy::{
    asset::Handle,
    ecs::bundle::Bundle,
    render::{texture::Image, view::RenderLayers},
    text::{Text, TextStyle},
    ui::{
        node_bundles::{ButtonBundle, ImageBundle, NodeBundle, TextBundle},
        *,
    },
    utils::default,
};

#[macro_export]
macro_rules! el {
    ($bundle:expr, $element:ident::<$($classes:tt),*>$(($($args:expr),*))?  $(, [$($children:expr),*])?) => {
        el!(@build, $bundle, $element(cn!($($classes),*) $(, $($args),*)?) $(, [$($children),*])?)
    };
    ($element:ident::<$($classes:tt),*>$(($($args:expr),*))?  $(, [$($children:expr),*])?) => {
        el!(@build, (), $element(cn!($($classes),*) $(, $($args),*)?) $(, [$($children),*])?)
    };
    (@build, $bundle:expr, $element:expr, [$($child:expr),*]) => {
        move |p: &mut bevy::prelude::ChildBuilder| {
            p.spawn(($element, $bundle)).with_children(el!(@children, $($child),*));
        }
    };
    (@build, $bundle:expr, $element:expr) => {
        move |p: &mut bevy::prelude::ChildBuilder| {
            p.spawn(($element, $bundle));
        }
    };
    (@build, $element:ident) => {
        move |p: &mut bevy::prelude::ChildBuilder| {
            $element(p);
        }
    };
    ($($child:expr),*) => {
        move |p: &mut bevy::prelude::ChildBuilder| {
            $(
                $child(p);
            )*
        }
    };
    (@children, $($child:expr),*) => {
        move |p: &mut bevy::prelude::ChildBuilder| {
            $(
                $child(p);
            )*
        }
    };
}

#[macro_export]
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

pub fn div(class: impl FnOnce(&mut NodeBundle)) -> impl Bundle {
    let mut bundle = NodeBundle::default();
    class(&mut bundle);

    (bundle, RenderLayers::layer(31))
}

pub fn text(class: impl FnOnce(&mut TextBundle), text: impl Into<String>) -> impl Bundle {
    let mut bundle = TextBundle {
        text: Text::from_section(text, TextStyle::default()),
        ..default()
    };

    class(&mut bundle);
    (bundle, RenderLayers::layer(31))
}

pub fn button(class: impl FnOnce(&mut ButtonBundle)) -> impl Bundle {
    let mut bundle = ButtonBundle::default();
    class(&mut bundle);

    (bundle, RenderLayers::layer(31))
}

pub fn img(class: impl FnOnce(&mut ImageBundle), src: Handle<Image>) -> impl Bundle {
    let mut bundle = ImageBundle::default();
    class(&mut bundle);

    bundle.image = UiImage::new(src);
    (bundle, RenderLayers::layer(31))
}
