use bevy::ui::{AlignItems, AlignSelf, FlexDirection, FlexWrap, JustifyContent, Val};
use el::HasStyle;

pub fn flex_col(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.flex_direction = FlexDirection::Column;
}

pub fn flex_row(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.flex_direction = FlexDirection::Row;
}

pub fn justify_end(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.justify_content = JustifyContent::FlexEnd;
}

pub fn justify_center(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.justify_content = JustifyContent::Center;
}

pub fn justify_between(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.justify_content = JustifyContent::SpaceBetween;
}

pub fn justify_around(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.justify_content = JustifyContent::SpaceAround;
}

pub fn justify_evenly(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.justify_content = JustifyContent::SpaceEvenly;
}

pub fn justify_start(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.justify_content = JustifyContent::FlexStart;
}

pub fn justify_stretch(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.justify_content = JustifyContent::Stretch;
}

pub fn justify_default(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.justify_content = JustifyContent::Default;
}

pub fn items_start(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.align_items = AlignItems::FlexStart;
}

pub fn items_center(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.align_items = AlignItems::Center;
}

pub fn items_end(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.align_items = AlignItems::FlexEnd;
}

pub fn items_baseline(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.align_items = AlignItems::Baseline;
}

pub fn items_stretch(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.align_items = AlignItems::Stretch;
}

pub fn items_default(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.align_items = AlignItems::Default;
}

pub fn self_start(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.align_self = AlignSelf::FlexStart;
}

pub fn self_center(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.align_self = AlignSelf::Center;
}

pub fn self_end(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.align_self = AlignSelf::FlexEnd;
}

pub fn self_baseline(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.align_self = AlignSelf::Baseline;
}

pub fn self_stretch(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.align_self = AlignSelf::Stretch;
}

pub fn grow(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.flex_grow = 1.0;
}

pub fn grow_0(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.flex_grow = 0.0;
}

pub fn shrink(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.flex_shrink = 1.0;
}

pub fn shrink_0(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.flex_shrink = 0.0;
}

pub fn flex_1(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.flex_grow = 1.0;
    style.flex_shrink = 1.0;
    style.flex_basis = Val::Percent(0.0);
}

pub fn flex_auto(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.flex_grow = 1.0;
    style.flex_shrink = 1.0;
    style.flex_basis = Val::Auto;
}

pub fn flex_initial(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.flex_grow = 0.0;
    style.flex_shrink = 1.0;
    style.flex_basis = Val::Auto;
}

pub fn flex_none(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.flex_grow = 0.0;
    style.flex_shrink = 0.0;
    style.flex_basis = Val::Auto;
}

pub fn flex_wrap(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.flex_wrap = FlexWrap::Wrap;
}

pub fn flex_wrap_reverse(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.flex_wrap = FlexWrap::WrapReverse;
}

pub fn flex_nowrap(bundle: &mut impl HasStyle) {
    let style = bundle.style();
    style.flex_wrap = FlexWrap::NoWrap;
}
