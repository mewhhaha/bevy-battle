use bevy::prelude::*;
use el::*;
use stylesheet::*;

use crate::MenuAction;

pub fn spawn_battle_screen(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let p_image = asset_server.load::<Image>("textures/portrait.png");
    let bg_image = asset_server.load::<Image>("textures/background.png");

    fn menu_text(t: MenuAction) -> impl FnOnce(&mut ChildBuilder) {
        el!(text::<text_black, text_4xl>(match t.into() {
            MenuAction::Attack => "Attack",
            MenuAction::Items => "Items",
            MenuAction::Defend => "Defend",
        }))
    }

    fn menu_button(t: MenuAction) -> impl FnOnce(&mut ChildBuilder) {
        el!(
            t.clone(),
            button::<bg_white, flex, justify_center>,
            [menu_text(t)]
        )
    }

    fn background(image: &Handle<Image>) -> impl FnOnce(&mut ChildBuilder) + '_ {
        el!(img::<absolute, inset_0>(image.clone()))
    }

    fn footer(
        menu: impl FnOnce(&mut ChildBuilder),
        portraits: impl FnOnce(&mut ChildBuilder),
    ) -> impl FnOnce(&mut ChildBuilder) {
        el!(
            div::<w_full, flex, h_48, bg_black, p_4>,
            [
                el!(div::<flex, flex_col, h_full, w_64, bg_amber_100>, [menu]),
                el!(
                    div::<flex, grow, p_4, gap_4, justify_center, bg_rose_100>,
                    [portraits]
                )
            ]
        )
    }

    fn frame(image: &Handle<Image>) -> impl FnOnce(&mut ChildBuilder) + '_ {
        el!(
            div::<w_32, h_32, bg_black, p_4>,
            [el!(img::<w_full, h_full>(image.clone()))]
        )
    }

    let root = div(cn![flex, items_end, w_full, h_full, relative]);
    commands.spawn(root).with_children(el![
        background(&bg_image),
        footer(
            el![
                menu_button(MenuAction::Attack),
                menu_button(MenuAction::Defend)
            ],
            el![frame(&p_image), frame(&p_image), frame(&p_image)]
        )
    ]);
}
