use bevy::render::color::Color;
use el::HasBackgroundColor;

/// ```
///
/// Color::rgb_u8(0, 0, 0)
/// ```
pub fn bg_black(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(0, 0, 0);
}

/// ```
///
/// Color::rgb_u8(255, 255, 255)
/// ```
pub fn bg_white(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(255, 255, 255);
}

/// ```
///
/// Color::rgb_u8(248, 250, 252)
/// ```
pub fn bg_slate_50(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(248, 250, 252);
}

/// ```
///
/// Color::rgb_u8(241, 245, 249)
/// ```
pub fn bg_slate_100(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(241, 245, 249);
}

/// ```
///
/// Color::rgb_u8(226, 232, 240)
/// ```
pub fn bg_slate_200(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(226, 232, 240);
}

/// ```
///
/// Color::rgb_u8(203, 213, 225)
/// ```
pub fn bg_slate_300(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(203, 213, 225);
}

/// ```
///
/// Color::rgb_u8(148, 163, 184)
/// ```
pub fn bg_slate_400(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(148, 163, 184);
}

/// ```
///
/// Color::rgb_u8(100, 116, 139)
/// ```
pub fn bg_slate_500(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(100, 116, 139);
}

/// ```
///
/// Color::rgb_u8(71, 85, 105)
/// ```
pub fn bg_slate_600(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(71, 85, 105);
}

/// ```
///
/// Color::rgb_u8(51, 65, 85)
/// ```
pub fn bg_slate_700(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(51, 65, 85);
}

/// ```
///
/// Color::rgb_u8(30, 41, 59)
/// ```
pub fn bg_slate_800(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(30, 41, 59);
}

/// ```
///
/// Color::rgb_u8(15, 23, 42)
/// ```
pub fn bg_slate_900(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(15, 23, 42);
}

/// ```
///
/// Color::rgb_u8(2, 6, 23)
/// ```
pub fn bg_slate_950(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(2, 6, 23);
}

/// ```
///
/// Color::rgb_u8(249, 250, 251)
/// ```
pub fn bg_gray_50(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(249, 250, 251);
}

/// ```
///
/// Color::rgb_u8(243, 244, 246)
/// ```
pub fn bg_gray_100(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(243, 244, 246);
}

/// ```
///
/// Color::rgb_u8(229, 231, 235)
/// ```
pub fn bg_gray_200(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(229, 231, 235);
}

/// ```
///
/// Color::rgb_u8(209, 213, 219)
/// ```
pub fn bg_gray_300(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(209, 213, 219);
}

/// ```
///
/// Color::rgb_u8(156, 163, 175)
/// ```
pub fn bg_gray_400(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(156, 163, 175);
}

/// ```
///
/// Color::rgb_u8(107, 114, 128)
/// ```
pub fn bg_gray_500(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(107, 114, 128);
}

/// ```
///
/// Color::rgb_u8(75, 85, 99)
/// ```
pub fn bg_gray_600(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(75, 85, 99);
}

/// ```
///
/// Color::rgb_u8(55, 65, 81)
/// ```
pub fn bg_gray_700(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(55, 65, 81);
}

/// ```
///
/// Color::rgb_u8(31, 41, 55)
/// ```
pub fn bg_gray_800(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(31, 41, 55);
}

/// ```
///
/// Color::rgb_u8(17, 24, 39)
/// ```
pub fn bg_gray_900(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(17, 24, 39);
}

/// ```
///
/// Color::rgb_u8(3, 7, 18)
/// ```
pub fn bg_gray_950(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(3, 7, 18);
}

/// ```
///
/// Color::rgb_u8(250, 250, 250)
/// ```
pub fn bg_zinc_50(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(250, 250, 250);
}

/// ```
///
/// Color::rgb_u8(244, 244, 245)
/// ```
pub fn bg_zinc_100(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(244, 244, 245);
}

/// ```
///
/// Color::rgb_u8(228, 228, 231)
/// ```
pub fn bg_zinc_200(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(228, 228, 231);
}

/// ```
///
/// Color::rgb_u8(212, 212, 216)
/// ```
pub fn bg_zinc_300(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(212, 212, 216);
}

/// ```
///
/// Color::rgb_u8(161, 161, 170)
/// ```
pub fn bg_zinc_400(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(161, 161, 170);
}

/// ```
///
/// Color::rgb_u8(113, 113, 122)
/// ```
pub fn bg_zinc_500(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(113, 113, 122);
}

/// ```
///
/// Color::rgb_u8(82, 82, 91)
/// ```
pub fn bg_zinc_600(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(82, 82, 91);
}

/// ```
///
/// Color::rgb_u8(63, 63, 70)
/// ```
pub fn bg_zinc_700(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(63, 63, 70);
}

/// ```
///
/// Color::rgb_u8(39, 39, 42)
/// ```
pub fn bg_zinc_800(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(39, 39, 42);
}

/// ```
///
/// Color::rgb_u8(24, 24, 27)
/// ```
pub fn bg_zinc_900(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(24, 24, 27);
}

/// ```
///
/// Color::rgb_u8(9, 9, 11)
/// ```
pub fn bg_zinc_950(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(9, 9, 11);
}

/// ```
///
/// Color::rgb_u8(250, 250, 250)
/// ```
pub fn bg_neutral_50(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(250, 250, 250);
}

/// ```
///
/// Color::rgb_u8(245, 245, 245)
/// ```
pub fn bg_neutral_100(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(245, 245, 245);
}

/// ```
///
/// Color::rgb_u8(229, 229, 229)
/// ```
pub fn bg_neutral_200(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(229, 229, 229);
}

/// ```
///
/// Color::rgb_u8(212, 212, 212)
/// ```
pub fn bg_neutral_300(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(212, 212, 212);
}

/// ```
///
/// Color::rgb_u8(163, 163, 163)
/// ```
pub fn bg_neutral_400(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(163, 163, 163);
}

/// ```
///
/// Color::rgb_u8(115, 115, 115)
/// ```
pub fn bg_neutral_500(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(115, 115, 115);
}

/// ```
///
/// Color::rgb_u8(82, 82, 82)
/// ```
pub fn bg_neutral_600(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(82, 82, 82);
}

/// ```
///
/// Color::rgb_u8(64, 64, 64)
/// ```
pub fn bg_neutral_700(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(64, 64, 64);
}

/// ```
///
/// Color::rgb_u8(38, 38, 38)
/// ```
pub fn bg_neutral_800(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(38, 38, 38);
}

/// ```
///
/// Color::rgb_u8(23, 23, 23)
/// ```
pub fn bg_neutral_900(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(23, 23, 23);
}

/// ```
///
/// Color::rgb_u8(10, 10, 10)
/// ```
pub fn bg_neutral_950(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(10, 10, 10);
}

/// ```
///
/// Color::rgb_u8(250, 250, 249)
/// ```
pub fn bg_stone_50(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(250, 250, 249);
}

/// ```
///
/// Color::rgb_u8(245, 245, 244)
/// ```
pub fn bg_stone_100(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(245, 245, 244);
}

/// ```
///
/// Color::rgb_u8(231, 229, 228)
/// ```
pub fn bg_stone_200(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(231, 229, 228);
}

/// ```
///
/// Color::rgb_u8(214, 211, 209)
/// ```
pub fn bg_stone_300(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(214, 211, 209);
}

/// ```
///
/// Color::rgb_u8(168, 162, 158)
/// ```
pub fn bg_stone_400(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(168, 162, 158);
}

/// ```
///
/// Color::rgb_u8(120, 113, 108)
/// ```
pub fn bg_stone_500(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(120, 113, 108);
}

/// ```
///
/// Color::rgb_u8(87, 83, 78)
/// ```
pub fn bg_stone_600(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(87, 83, 78);
}

/// ```
///
/// Color::rgb_u8(68, 64, 60)
/// ```
pub fn bg_stone_700(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(68, 64, 60);
}

/// ```
///
/// Color::rgb_u8(41, 37, 36)
/// ```
pub fn bg_stone_800(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(41, 37, 36);
}

/// ```
///
/// Color::rgb_u8(28, 25, 23)
/// ```
pub fn bg_stone_900(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(28, 25, 23);
}

/// ```
///
/// Color::rgb_u8(12, 10, 9)
/// ```
pub fn bg_stone_950(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(12, 10, 9);
}

/// ```
///
/// Color::rgb_u8(254, 242, 242)
/// ```
pub fn bg_red_50(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(254, 242, 242);
}

/// ```
///
/// Color::rgb_u8(254, 226, 226)
/// ```
pub fn bg_red_100(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(254, 226, 226);
}

/// ```
///
/// Color::rgb_u8(254, 202, 202)
/// ```
pub fn bg_red_200(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(254, 202, 202);
}

/// ```
///
/// Color::rgb_u8(252, 165, 165)
/// ```
pub fn bg_red_300(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(252, 165, 165);
}

/// ```
///
/// Color::rgb_u8(248, 113, 113)
/// ```
pub fn bg_red_400(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(248, 113, 113);
}

/// ```
///
/// Color::rgb_u8(239, 68, 68)
/// ```
pub fn bg_red_500(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(239, 68, 68);
}

/// ```
///
/// Color::rgb_u8(220, 38, 38)
/// ```
pub fn bg_red_600(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(220, 38, 38);
}

/// ```
///
/// Color::rgb_u8(185, 28, 28)
/// ```
pub fn bg_red_700(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(185, 28, 28);
}

/// ```
///
/// Color::rgb_u8(153, 27, 27)
/// ```
pub fn bg_red_800(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(153, 27, 27);
}

/// ```
///
/// Color::rgb_u8(127, 29, 29)
/// ```
pub fn bg_red_900(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(127, 29, 29);
}

/// ```
///
/// Color::rgb_u8(69, 10, 10)
/// ```
pub fn bg_red_950(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(69, 10, 10);
}

/// ```
///
/// Color::rgb_u8(255, 247, 237)
/// ```
pub fn bg_orange_50(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(255, 247, 237);
}

/// ```
///
/// Color::rgb_u8(255, 237, 213)
/// ```
pub fn bg_orange_100(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(255, 237, 213);
}

/// ```
///
/// Color::rgb_u8(254, 215, 170)
/// ```
pub fn bg_orange_200(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(254, 215, 170);
}

/// ```
///
/// Color::rgb_u8(253, 186, 116)
/// ```
pub fn bg_orange_300(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(253, 186, 116);
}

/// ```
///
/// Color::rgb_u8(251, 146, 60)
/// ```
pub fn bg_orange_400(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(251, 146, 60);
}

/// ```
///
/// Color::rgb_u8(249, 115, 22)
/// ```
pub fn bg_orange_500(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(249, 115, 22);
}

/// ```
///
/// Color::rgb_u8(234, 88, 12)
/// ```
pub fn bg_orange_600(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(234, 88, 12);
}

/// ```
///
/// Color::rgb_u8(194, 65, 12)
/// ```
pub fn bg_orange_700(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(194, 65, 12);
}

/// ```
///
/// Color::rgb_u8(154, 52, 18)
/// ```
pub fn bg_orange_800(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(154, 52, 18);
}

/// ```
///
/// Color::rgb_u8(124, 45, 18)
/// ```
pub fn bg_orange_900(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(124, 45, 18);
}

/// ```
///
/// Color::rgb_u8(67, 20, 7)
/// ```
pub fn bg_orange_950(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(67, 20, 7);
}

/// ```
///
/// Color::rgb_u8(255, 251, 235)
/// ```
pub fn bg_amber_50(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(255, 251, 235);
}

/// ```
///
/// Color::rgb_u8(254, 243, 199)
/// ```
pub fn bg_amber_100(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(254, 243, 199);
}

/// ```
///
/// Color::rgb_u8(253, 230, 138)
/// ```
pub fn bg_amber_200(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(253, 230, 138);
}

/// ```
///
/// Color::rgb_u8(252, 211, 77)
/// ```
pub fn bg_amber_300(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(252, 211, 77);
}

/// ```
///
/// Color::rgb_u8(251, 191, 36)
/// ```
pub fn bg_amber_400(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(251, 191, 36);
}

/// ```
///
/// Color::rgb_u8(245, 158, 11)
/// ```
pub fn bg_amber_500(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(245, 158, 11);
}

/// ```
///
/// Color::rgb_u8(217, 119, 6)
/// ```
pub fn bg_amber_600(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(217, 119, 6);
}

/// ```
///
/// Color::rgb_u8(180, 83, 9)
/// ```
pub fn bg_amber_700(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(180, 83, 9);
}

/// ```
///
/// Color::rgb_u8(146, 64, 14)
/// ```
pub fn bg_amber_800(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(146, 64, 14);
}

/// ```
///
/// Color::rgb_u8(120, 53, 15)
/// ```
pub fn bg_amber_900(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(120, 53, 15);
}

/// ```
///
/// Color::rgb_u8(69, 26, 3)
/// ```
pub fn bg_amber_950(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(69, 26, 3);
}

/// ```
///
/// Color::rgb_u8(254, 252, 232)
/// ```
pub fn bg_yellow_50(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(254, 252, 232);
}

/// ```
///
/// Color::rgb_u8(254, 249, 195)
/// ```
pub fn bg_yellow_100(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(254, 249, 195);
}

/// ```
///
/// Color::rgb_u8(254, 240, 138)
/// ```
pub fn bg_yellow_200(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(254, 240, 138);
}

/// ```
///
/// Color::rgb_u8(253, 224, 71)
/// ```
pub fn bg_yellow_300(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(253, 224, 71);
}

/// ```
///
/// Color::rgb_u8(250, 204, 21)
/// ```
pub fn bg_yellow_400(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(250, 204, 21);
}

/// ```
///
/// Color::rgb_u8(234, 179, 8)
/// ```
pub fn bg_yellow_500(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(234, 179, 8);
}

/// ```
///
/// Color::rgb_u8(202, 138, 4)
/// ```
pub fn bg_yellow_600(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(202, 138, 4);
}

/// ```
///
/// Color::rgb_u8(161, 98, 7)
/// ```
pub fn bg_yellow_700(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(161, 98, 7);
}

/// ```
///
/// Color::rgb_u8(133, 77, 14)
/// ```
pub fn bg_yellow_800(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(133, 77, 14);
}

/// ```
///
/// Color::rgb_u8(113, 63, 18)
/// ```
pub fn bg_yellow_900(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(113, 63, 18);
}

/// ```
///
/// Color::rgb_u8(66, 32, 6)
/// ```
pub fn bg_yellow_950(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(66, 32, 6);
}

/// ```
///
/// Color::rgb_u8(247, 254, 231)
/// ```
pub fn bg_lime_50(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(247, 254, 231);
}

/// ```
///
/// Color::rgb_u8(236, 252, 203)
/// ```
pub fn bg_lime_100(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(236, 252, 203);
}

/// ```
///
/// Color::rgb_u8(217, 249, 157)
/// ```
pub fn bg_lime_200(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(217, 249, 157);
}

/// ```
///
/// Color::rgb_u8(190, 242, 100)
/// ```
pub fn bg_lime_300(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(190, 242, 100);
}

/// ```
///
/// Color::rgb_u8(163, 230, 53)
/// ```
pub fn bg_lime_400(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(163, 230, 53);
}

/// ```
///
/// Color::rgb_u8(132, 204, 22)
/// ```
pub fn bg_lime_500(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(132, 204, 22);
}

/// ```
///
/// Color::rgb_u8(101, 163, 13)
/// ```
pub fn bg_lime_600(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(101, 163, 13);
}

/// ```
///
/// Color::rgb_u8(77, 124, 15)
/// ```
pub fn bg_lime_700(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(77, 124, 15);
}

/// ```
///
/// Color::rgb_u8(63, 98, 18)
/// ```
pub fn bg_lime_800(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(63, 98, 18);
}

/// ```
///
/// Color::rgb_u8(54, 83, 20)
/// ```
pub fn bg_lime_900(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(54, 83, 20);
}

/// ```
///
/// Color::rgb_u8(26, 46, 5)
/// ```
pub fn bg_lime_950(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(26, 46, 5);
}

/// ```
///
/// Color::rgb_u8(240, 253, 244)
/// ```
pub fn bg_green_50(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(240, 253, 244);
}

/// ```
///
/// Color::rgb_u8(220, 252, 231)
/// ```
pub fn bg_green_100(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(220, 252, 231);
}

/// ```
///
/// Color::rgb_u8(187, 247, 208)
/// ```
pub fn bg_green_200(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(187, 247, 208);
}

/// ```
///
/// Color::rgb_u8(134, 239, 172)
/// ```
pub fn bg_green_300(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(134, 239, 172);
}

/// ```
///
/// Color::rgb_u8(74, 222, 128)
/// ```
pub fn bg_green_400(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(74, 222, 128);
}

/// ```
///
/// Color::rgb_u8(34, 197, 94)
/// ```
pub fn bg_green_500(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(34, 197, 94);
}

/// ```
///
/// Color::rgb_u8(22, 163, 74)
/// ```
pub fn bg_green_600(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(22, 163, 74);
}

/// ```
///
/// Color::rgb_u8(21, 128, 61)
/// ```
pub fn bg_green_700(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(21, 128, 61);
}

/// ```
///
/// Color::rgb_u8(22, 101, 52)
/// ```
pub fn bg_green_800(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(22, 101, 52);
}

/// ```
///
/// Color::rgb_u8(20, 83, 45)
/// ```
pub fn bg_green_900(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(20, 83, 45);
}

/// ```
///
/// Color::rgb_u8(5, 46, 22)
/// ```
pub fn bg_green_950(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(5, 46, 22);
}

/// ```
///
/// Color::rgb_u8(236, 253, 245)
/// ```
pub fn bg_emerald_50(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(236, 253, 245);
}

/// ```
///
/// Color::rgb_u8(209, 250, 229)
/// ```
pub fn bg_emerald_100(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(209, 250, 229);
}

/// ```
///
/// Color::rgb_u8(167, 243, 208)
/// ```
pub fn bg_emerald_200(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(167, 243, 208);
}

/// ```
///
/// Color::rgb_u8(110, 231, 183)
/// ```
pub fn bg_emerald_300(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(110, 231, 183);
}

/// ```
///
/// Color::rgb_u8(52, 211, 153)
/// ```
pub fn bg_emerald_400(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(52, 211, 153);
}

/// ```
///
/// Color::rgb_u8(16, 185, 129)
/// ```
pub fn bg_emerald_500(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(16, 185, 129);
}

/// ```
///
/// Color::rgb_u8(5, 150, 105)
/// ```
pub fn bg_emerald_600(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(5, 150, 105);
}

/// ```
///
/// Color::rgb_u8(4, 120, 87)
/// ```
pub fn bg_emerald_700(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(4, 120, 87);
}

/// ```
///
/// Color::rgb_u8(6, 95, 70)
/// ```
pub fn bg_emerald_800(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(6, 95, 70);
}

/// ```
///
/// Color::rgb_u8(6, 78, 59)
/// ```
pub fn bg_emerald_900(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(6, 78, 59);
}

/// ```
///
/// Color::rgb_u8(2, 44, 34)
/// ```
pub fn bg_emerald_950(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(2, 44, 34);
}

/// ```
///
/// Color::rgb_u8(240, 253, 250)
/// ```
pub fn bg_teal_50(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(240, 253, 250);
}

/// ```
///
/// Color::rgb_u8(204, 251, 241)
/// ```
pub fn bg_teal_100(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(204, 251, 241);
}

/// ```
///
/// Color::rgb_u8(153, 246, 228)
/// ```
pub fn bg_teal_200(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(153, 246, 228);
}

/// ```
///
/// Color::rgb_u8(94, 234, 212)
/// ```
pub fn bg_teal_300(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(94, 234, 212);
}

/// ```
///
/// Color::rgb_u8(45, 212, 191)
/// ```
pub fn bg_teal_400(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(45, 212, 191);
}

/// ```
///
/// Color::rgb_u8(20, 184, 166)
/// ```
pub fn bg_teal_500(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(20, 184, 166);
}

/// ```
///
/// Color::rgb_u8(13, 148, 136)
/// ```
pub fn bg_teal_600(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(13, 148, 136);
}

/// ```
///
/// Color::rgb_u8(15, 118, 110)
/// ```
pub fn bg_teal_700(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(15, 118, 110);
}

/// ```
///
/// Color::rgb_u8(17, 94, 89)
/// ```
pub fn bg_teal_800(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(17, 94, 89);
}

/// ```
///
/// Color::rgb_u8(19, 78, 74)
/// ```
pub fn bg_teal_900(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(19, 78, 74);
}

/// ```
///
/// Color::rgb_u8(4, 47, 46)
/// ```
pub fn bg_teal_950(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(4, 47, 46);
}

/// ```
///
/// Color::rgb_u8(236, 254, 255)
/// ```
pub fn bg_cyan_50(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(236, 254, 255);
}

/// ```
///
/// Color::rgb_u8(207, 250, 254)
/// ```
pub fn bg_cyan_100(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(207, 250, 254);
}

/// ```
///
/// Color::rgb_u8(165, 243, 252)
/// ```
pub fn bg_cyan_200(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(165, 243, 252);
}

/// ```
///
/// Color::rgb_u8(103, 232, 249)
/// ```
pub fn bg_cyan_300(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(103, 232, 249);
}

/// ```
///
/// Color::rgb_u8(34, 211, 238)
/// ```
pub fn bg_cyan_400(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(34, 211, 238);
}

/// ```
///
/// Color::rgb_u8(6, 182, 212)
/// ```
pub fn bg_cyan_500(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(6, 182, 212);
}

/// ```
///
/// Color::rgb_u8(8, 145, 178)
/// ```
pub fn bg_cyan_600(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(8, 145, 178);
}

/// ```
///
/// Color::rgb_u8(14, 116, 144)
/// ```
pub fn bg_cyan_700(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(14, 116, 144);
}

/// ```
///
/// Color::rgb_u8(21, 94, 117)
/// ```
pub fn bg_cyan_800(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(21, 94, 117);
}

/// ```
///
/// Color::rgb_u8(22, 78, 99)
/// ```
pub fn bg_cyan_900(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(22, 78, 99);
}

/// ```
///
/// Color::rgb_u8(8, 51, 68)
/// ```
pub fn bg_cyan_950(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(8, 51, 68);
}

/// ```
///
/// Color::rgb_u8(240, 249, 255)
/// ```
pub fn bg_sky_50(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(240, 249, 255);
}

/// ```
///
/// Color::rgb_u8(224, 242, 254)
/// ```
pub fn bg_sky_100(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(224, 242, 254);
}

/// ```
///
/// Color::rgb_u8(186, 230, 253)
/// ```
pub fn bg_sky_200(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(186, 230, 253);
}

/// ```
///
/// Color::rgb_u8(125, 211, 252)
/// ```
pub fn bg_sky_300(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(125, 211, 252);
}

/// ```
///
/// Color::rgb_u8(56, 189, 248)
/// ```
pub fn bg_sky_400(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(56, 189, 248);
}

/// ```
///
/// Color::rgb_u8(14, 165, 233)
/// ```
pub fn bg_sky_500(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(14, 165, 233);
}

/// ```
///
/// Color::rgb_u8(2, 132, 199)
/// ```
pub fn bg_sky_600(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(2, 132, 199);
}

/// ```
///
/// Color::rgb_u8(3, 105, 161)
/// ```
pub fn bg_sky_700(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(3, 105, 161);
}

/// ```
///
/// Color::rgb_u8(7, 89, 133)
/// ```
pub fn bg_sky_800(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(7, 89, 133);
}

/// ```
///
/// Color::rgb_u8(12, 74, 110)
/// ```
pub fn bg_sky_900(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(12, 74, 110);
}

/// ```
///
/// Color::rgb_u8(8, 47, 73)
/// ```
pub fn bg_sky_950(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(8, 47, 73);
}

/// ```
///
/// Color::rgb_u8(239, 246, 255)
/// ```
pub fn bg_blue_50(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(239, 246, 255);
}

/// ```
///
/// Color::rgb_u8(219, 234, 254)
/// ```
pub fn bg_blue_100(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(219, 234, 254);
}

/// ```
///
/// Color::rgb_u8(191, 219, 254)
/// ```
pub fn bg_blue_200(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(191, 219, 254);
}

/// ```
///
/// Color::rgb_u8(147, 197, 253)
/// ```
pub fn bg_blue_300(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(147, 197, 253);
}

/// ```
///
/// Color::rgb_u8(96, 165, 250)
/// ```
pub fn bg_blue_400(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(96, 165, 250);
}

/// ```
///
/// Color::rgb_u8(59, 130, 246)
/// ```
pub fn bg_blue_500(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(59, 130, 246);
}

/// ```
///
/// Color::rgb_u8(37, 99, 235)
/// ```
pub fn bg_blue_600(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(37, 99, 235);
}

/// ```
///
/// Color::rgb_u8(29, 78, 216)
/// ```
pub fn bg_blue_700(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(29, 78, 216);
}

/// ```
///
/// Color::rgb_u8(30, 64, 175)
/// ```
pub fn bg_blue_800(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(30, 64, 175);
}

/// ```
///
/// Color::rgb_u8(30, 58, 138)
/// ```
pub fn bg_blue_900(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(30, 58, 138);
}

/// ```
///
/// Color::rgb_u8(23, 37, 84)
/// ```
pub fn bg_blue_950(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(23, 37, 84);
}

/// ```
///
/// Color::rgb_u8(238, 242, 255)
/// ```
pub fn bg_indigo_50(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(238, 242, 255);
}

/// ```
///
/// Color::rgb_u8(224, 231, 255)
/// ```
pub fn bg_indigo_100(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(224, 231, 255);
}

/// ```
///
/// Color::rgb_u8(199, 210, 254)
/// ```
pub fn bg_indigo_200(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(199, 210, 254);
}

/// ```
///
/// Color::rgb_u8(165, 180, 252)
/// ```
pub fn bg_indigo_300(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(165, 180, 252);
}

/// ```
///
/// Color::rgb_u8(129, 140, 248)
/// ```
pub fn bg_indigo_400(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(129, 140, 248);
}

/// ```
///
/// Color::rgb_u8(99, 102, 241)
/// ```
pub fn bg_indigo_500(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(99, 102, 241);
}

/// ```
///
/// Color::rgb_u8(79, 70, 229)
/// ```
pub fn bg_indigo_600(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(79, 70, 229);
}

/// ```
///
/// Color::rgb_u8(67, 56, 202)
/// ```
pub fn bg_indigo_700(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(67, 56, 202);
}

/// ```
///
/// Color::rgb_u8(55, 48, 163)
/// ```
pub fn bg_indigo_800(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(55, 48, 163);
}

/// ```
///
/// Color::rgb_u8(49, 46, 129)
/// ```
pub fn bg_indigo_900(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(49, 46, 129);
}

/// ```
///
/// Color::rgb_u8(30, 27, 75)
/// ```
pub fn bg_indigo_950(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(30, 27, 75);
}

/// ```
///
/// Color::rgb_u8(245, 243, 255)
/// ```
pub fn bg_violet_50(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(245, 243, 255);
}

/// ```
///
/// Color::rgb_u8(237, 233, 254)
/// ```
pub fn bg_violet_100(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(237, 233, 254);
}

/// ```
///
/// Color::rgb_u8(221, 214, 254)
/// ```
pub fn bg_violet_200(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(221, 214, 254);
}

/// ```
///
/// Color::rgb_u8(196, 181, 253)
/// ```
pub fn bg_violet_300(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(196, 181, 253);
}

/// ```
///
/// Color::rgb_u8(167, 139, 250)
/// ```
pub fn bg_violet_400(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(167, 139, 250);
}

/// ```
///
/// Color::rgb_u8(139, 92, 246)
/// ```
pub fn bg_violet_500(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(139, 92, 246);
}

/// ```
///
/// Color::rgb_u8(124, 58, 237)
/// ```
pub fn bg_violet_600(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(124, 58, 237);
}

/// ```
///
/// Color::rgb_u8(109, 40, 217)
/// ```
pub fn bg_violet_700(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(109, 40, 217);
}

/// ```
///
/// Color::rgb_u8(91, 33, 182)
/// ```
pub fn bg_violet_800(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(91, 33, 182);
}

/// ```
///
/// Color::rgb_u8(76, 29, 149)
/// ```
pub fn bg_violet_900(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(76, 29, 149);
}

/// ```
///
/// Color::rgb_u8(46, 16, 101)
/// ```
pub fn bg_violet_950(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(46, 16, 101);
}

/// ```
///
/// Color::rgb_u8(250, 245, 255)
/// ```
pub fn bg_purple_50(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(250, 245, 255);
}

/// ```
///
/// Color::rgb_u8(243, 232, 255)
/// ```
pub fn bg_purple_100(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(243, 232, 255);
}

/// ```
///
/// Color::rgb_u8(233, 213, 255)
/// ```
pub fn bg_purple_200(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(233, 213, 255);
}

/// ```
///
/// Color::rgb_u8(216, 180, 254)
/// ```
pub fn bg_purple_300(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(216, 180, 254);
}

/// ```
///
/// Color::rgb_u8(192, 132, 252)
/// ```
pub fn bg_purple_400(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(192, 132, 252);
}

/// ```
///
/// Color::rgb_u8(168, 85, 247)
/// ```
pub fn bg_purple_500(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(168, 85, 247);
}

/// ```
///
/// Color::rgb_u8(147, 51, 234)
/// ```
pub fn bg_purple_600(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(147, 51, 234);
}

/// ```
///
/// Color::rgb_u8(126, 34, 206)
/// ```
pub fn bg_purple_700(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(126, 34, 206);
}

/// ```
///
/// Color::rgb_u8(107, 33, 168)
/// ```
pub fn bg_purple_800(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(107, 33, 168);
}

/// ```
///
/// Color::rgb_u8(88, 28, 135)
/// ```
pub fn bg_purple_900(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(88, 28, 135);
}

/// ```
///
/// Color::rgb_u8(59, 7, 100)
/// ```
pub fn bg_purple_950(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(59, 7, 100);
}

/// ```
///
/// Color::rgb_u8(253, 244, 255)
/// ```
pub fn bg_fuchsia_50(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(253, 244, 255);
}

/// ```
///
/// Color::rgb_u8(250, 232, 255)
/// ```
pub fn bg_fuchsia_100(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(250, 232, 255);
}

/// ```
///
/// Color::rgb_u8(245, 208, 254)
/// ```
pub fn bg_fuchsia_200(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(245, 208, 254);
}

/// ```
///
/// Color::rgb_u8(240, 171, 252)
/// ```
pub fn bg_fuchsia_300(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(240, 171, 252);
}

/// ```
///
/// Color::rgb_u8(232, 121, 249)
/// ```
pub fn bg_fuchsia_400(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(232, 121, 249);
}

/// ```
///
/// Color::rgb_u8(217, 70, 239)
/// ```
pub fn bg_fuchsia_500(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(217, 70, 239);
}

/// ```
///
/// Color::rgb_u8(192, 38, 211)
/// ```
pub fn bg_fuchsia_600(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(192, 38, 211);
}

/// ```
///
/// Color::rgb_u8(162, 28, 175)
/// ```
pub fn bg_fuchsia_700(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(162, 28, 175);
}

/// ```
///
/// Color::rgb_u8(134, 25, 143)
/// ```
pub fn bg_fuchsia_800(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(134, 25, 143);
}

/// ```
///
/// Color::rgb_u8(112, 26, 117)
/// ```
pub fn bg_fuchsia_900(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(112, 26, 117);
}

/// ```
///
/// Color::rgb_u8(74, 4, 78)
/// ```
pub fn bg_fuchsia_950(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(74, 4, 78);
}

/// ```
///
/// Color::rgb_u8(253, 242, 248)
/// ```
pub fn bg_pink_50(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(253, 242, 248);
}

/// ```
///
/// Color::rgb_u8(252, 231, 243)
/// ```
pub fn bg_pink_100(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(252, 231, 243);
}

/// ```
///
/// Color::rgb_u8(251, 207, 232)
/// ```
pub fn bg_pink_200(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(251, 207, 232);
}

/// ```
///
/// Color::rgb_u8(249, 168, 212)
/// ```
pub fn bg_pink_300(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(249, 168, 212);
}

/// ```
///
/// Color::rgb_u8(244, 114, 182)
/// ```
pub fn bg_pink_400(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(244, 114, 182);
}

/// ```
///
/// Color::rgb_u8(236, 72, 153)
/// ```
pub fn bg_pink_500(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(236, 72, 153);
}

/// ```
///
/// Color::rgb_u8(219, 39, 119)
/// ```
pub fn bg_pink_600(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(219, 39, 119);
}

/// ```
///
/// Color::rgb_u8(190, 24, 93)
/// ```
pub fn bg_pink_700(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(190, 24, 93);
}

/// ```
///
/// Color::rgb_u8(157, 23, 77)
/// ```
pub fn bg_pink_800(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(157, 23, 77);
}

/// ```
///
/// Color::rgb_u8(131, 24, 67)
/// ```
pub fn bg_pink_900(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(131, 24, 67);
}

/// ```
///
/// Color::rgb_u8(80, 7, 36)
/// ```
pub fn bg_pink_950(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(80, 7, 36);
}

/// ```
///
/// Color::rgb_u8(255, 241, 242)
/// ```
pub fn bg_rose_50(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(255, 241, 242);
}

/// ```
///
/// Color::rgb_u8(255, 228, 230)
/// ```
pub fn bg_rose_100(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(255, 228, 230);
}

/// ```
///
/// Color::rgb_u8(254, 205, 211)
/// ```
pub fn bg_rose_200(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(254, 205, 211);
}

/// ```
///
/// Color::rgb_u8(253, 164, 175)
/// ```
pub fn bg_rose_300(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(253, 164, 175);
}

/// ```
///
/// Color::rgb_u8(251, 113, 133)
/// ```
pub fn bg_rose_400(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(251, 113, 133);
}

/// ```
///
/// Color::rgb_u8(244, 63, 94)
/// ```
pub fn bg_rose_500(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(244, 63, 94);
}

/// ```
///
/// Color::rgb_u8(225, 29, 72)
/// ```
pub fn bg_rose_600(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(225, 29, 72);
}

/// ```
///
/// Color::rgb_u8(190, 18, 60)
/// ```
pub fn bg_rose_700(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(190, 18, 60);
}

/// ```
///
/// Color::rgb_u8(159, 18, 57)
/// ```
pub fn bg_rose_800(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(159, 18, 57);
}

/// ```
///
/// Color::rgb_u8(136, 19, 55)
/// ```
pub fn bg_rose_900(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(136, 19, 55);
}

/// ```
///
/// Color::rgb_u8(76, 5, 25)
/// ```
pub fn bg_rose_950(bundle: &mut impl HasBackgroundColor) {
    let background_color = bundle.background_color();
    background_color.0 = Color::rgb_u8(76, 5, 25);
}
