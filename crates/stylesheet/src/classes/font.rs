use bevy::text::JustifyText;
use el::HasText;

pub fn text_center(bundle: &mut impl HasText) {
    let text = bundle.text();
    text.justify = JustifyText::Center;
}

pub fn text_left(bundle: &mut impl HasText) {
    let text = bundle.text();
    text.justify = JustifyText::Left;
}

pub fn text_right(bundle: &mut impl HasText) {
    let text = bundle.text();
    text.justify = JustifyText::Right;
}

pub fn text_xs(bundle: &mut impl HasText) {
    let text = bundle.text();
    text.sections.iter_mut().for_each(|section| {
        section.style.font_size = 12.0;
    });
}

pub fn text_sm(bundle: &mut impl HasText) {
    let text = bundle.text();
    text.sections.iter_mut().for_each(|section| {
        section.style.font_size = 14.0;
    });
}

pub fn text_base(bundle: &mut impl HasText) {
    let text = bundle.text();
    text.sections.iter_mut().for_each(|section| {
        section.style.font_size = 16.0;
    });
}

pub fn text_lg(bundle: &mut impl HasText) {
    let text = bundle.text();
    text.sections.iter_mut().for_each(|section| {
        section.style.font_size = 18.0;
    });
}

pub fn text_xl(bundle: &mut impl HasText) {
    let text = bundle.text();
    text.sections.iter_mut().for_each(|section| {
        section.style.font_size = 20.0;
    });
}

pub fn text_2xl(bundle: &mut impl HasText) {
    let text = bundle.text();
    text.sections.iter_mut().for_each(|section| {
        section.style.font_size = 24.0;
    });
}

pub fn text_3xl(bundle: &mut impl HasText) {
    let text = bundle.text();
    text.sections.iter_mut().for_each(|section| {
        section.style.font_size = 30.0;
    });
}

pub fn text_4xl(bundle: &mut impl HasText) {
    let text = bundle.text();
    text.sections.iter_mut().for_each(|section| {
        section.style.font_size = 36.0;
    });
}

pub fn text_5xl(bundle: &mut impl HasText) {
    let text = bundle.text();
    text.sections.iter_mut().for_each(|section| {
        section.style.font_size = 48.0;
    });
}

pub fn text_6xl(bundle: &mut impl HasText) {
    let text = bundle.text();
    text.sections.iter_mut().for_each(|section| {
        section.style.font_size = 64.0;
    });
}

pub fn text_7xl(bundle: &mut impl HasText) {
    let text = bundle.text();
    text.sections.iter_mut().for_each(|section| {
        section.style.font_size = 80.0;
    });
}

pub fn text_8xl(bundle: &mut impl HasText) {
    let text = bundle.text();
    text.sections.iter_mut().for_each(|section| {
        section.style.font_size = 96.0;
    });
}

pub fn text_9xl(bundle: &mut impl HasText) {
    let text = bundle.text();
    text.sections.iter_mut().for_each(|section| {
        section.style.font_size = 128.0;
    });
}
