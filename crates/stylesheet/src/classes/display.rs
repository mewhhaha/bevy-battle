use bevy::ui::Display;
use el::HasStyle;

pub fn flex(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.display = Display::Flex;
}

pub fn grid(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.display = Display::Grid;
}

pub fn hidden(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.display = Display::None;
}
