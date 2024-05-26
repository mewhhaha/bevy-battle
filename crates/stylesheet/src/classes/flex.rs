use bevy::ui::{Display, FlexDirection, JustifyContent};
use el::HasStyle;

pub fn flex(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.display = Display::Flex;
}

pub fn flex_col(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.flex_direction = FlexDirection::Column;
}

pub fn justify_end(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.justify_content = JustifyContent::FlexEnd;
}

pub fn justify_center(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.justify_content = JustifyContent::Center;
}
