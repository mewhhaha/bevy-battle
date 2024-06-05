use bevy::{prelude::*, scene::ron::ser};
use bevy_common_assets::toml::TomlAssetPlugin;
use el::*;
use serde::Deserialize;
use stylesheet::*;

use crate::{helpers::AppState, MenuAction};

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Damage(i32);

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Hitpoints(i32);

#[derive(Resource)]
pub struct BattleQueue(pub Vec<&'static str>);

#[derive(Component)]
pub struct BattleLayout;

#[derive(Deserialize, Asset, TypePath)]
struct EnemyData {
    name: String,
    hitpoints: i32,
    damage: i32,
    image: String,
}
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
        el!(
            div::<flex, flex_1, items_center, justify_center>,
            [el!(BattleLayout, div::<flex, w_full, justify_between>)]
        ),
        footer(
            el![
                menu_button(MenuAction::Attack),
                menu_button(MenuAction::Defend)
            ],
            el![frame(&p_image), frame(&p_image), frame(&p_image)]
        )
    ]);
}

fn spawn_enemies(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut queue: ResMut<BattleQueue>,
    layout_query: Query<Entity, With<BattleLayout>>,
    enemy_data: Res<Assets<EnemyData>>,
) {
    let layout = layout_query.single();
    for enemy in queue.0.iter() {
        let handle = asset_server.load(enemy.to_string());

        let EnemyData {
            name,
            hitpoints,
            damage,
            image,
        } = enemy_data
            .get(handle)
            .expect("Enemy was just loaded, but not found");

        let image = asset_server.load(image);

        commands.entity(layout).with_children(el!(
            (
                Name(name.clone()),
                Hitpoints(*hitpoints),
                Damage(*damage),
                Enemy
            ),
            div::<flex, flex_col>,
            [
                el!(text::<text_black, text_2xl>(name)),
                el!(text::<text_black, text_2xl>(format!("HP: {}", hitpoints))),
                el!(img::<w_32, h_32>(image))
            ]
        ));
    }

    queue.0.clear();
}

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Fighting), (spawn_battle_screen).chain())
            .insert_resource(BattleQueue(vec![]))
            .add_plugins(TomlAssetPlugin::<EnemyData>::new(&["enemy.toml"]))
            .add_systems(
                PreUpdate,
                spawn_enemies.run_if(in_state(AppState::Fighting)),
            );
    }
}
