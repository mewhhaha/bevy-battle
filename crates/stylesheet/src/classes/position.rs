use bevy::ui::PositionType;
use el::HasStyle;

pub fn absolute(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.position_type = PositionType::Absolute;
}

pub fn relative(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.position_type = PositionType::Relative;
}
