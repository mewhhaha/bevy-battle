use bevy::ui::Val;
use el::HasStyle;

pub fn inset_0(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.right = Val::Px(0.0);
    style.left = Val::Px(0.0);
    style.top = Val::Px(0.0);
    style.bottom = Val::Px(0.0);
}
