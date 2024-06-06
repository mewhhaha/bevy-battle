use bevy::prelude::*;
use bevy_common_assets::toml::TomlAssetPlugin;
use el::*;
use serde::Deserialize;
use stylesheet::*;

use crate::{
    helpers::{wait_for_assets, AppState},
    ui_events::OnClick,
    MenuAction,
};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum BattleState {
    EnemyTurn,
    QueryEnemy,
    QueryAlly,
    PlayerTurn,
}

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Damage(i32);

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Ally;

#[derive(Component)]
struct Hitpoints(i32);

#[derive(Resource, Debug)]
pub struct LoadBattle {
    pub ally_layout: Vec<Handle<CharacterData>>,
    pub enemy_layout: Vec<Handle<CharacterData>>,
}

#[derive(Resource, Debug)]
pub struct Battle {
    pub ally_layout: Vec<Entity>,
    pub enemy_layout: Vec<Entity>,
    pub order: Vec<Entity>,
    pub turn: usize,
}

enum Action {
    WantAttack(Entity),
    Defend(Entity),
    WantUse(Entity),
}

#[derive(Resource)]
pub struct Turn {
    entity: Entity,
    action: Option<Action>,
}

#[derive(Component)]
pub struct EnemyArea;

#[derive(Component)]
pub struct AllyArea;

#[derive(Component)]
pub struct Attached(Entity);

#[derive(Event)]
pub struct OnAttack {
    pub attacker: Entity,
    pub target: Entity,
    damage: i32,
}

#[derive(Deserialize, Asset, TypePath)]
pub struct CharacterData {
    name: String,
    hitpoints: i32,
    damage: i32,
    portrait: String,
    image: String,
}

fn frame(image: &Handle<Image>) -> impl FnOnce(&mut ChildBuilder) + '_ {
    el!(
        div::<w_32, h_32, bg_black, p_4>,
        [el!(img::<w_full, h_full>(image.clone()))]
    )
}

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

fn footer(menu: impl FnOnce(&mut ChildBuilder)) -> impl FnOnce(&mut ChildBuilder) {
    el!(
        div::<w_full, flex, h_48, bg_black, p_4>,
        [
            el!(div::<flex, flex_col, h_full, w_64, bg_amber_100>, [menu]),
            el!(
                AllyArea,
                div::<flex, grow, p_4, gap_4, justify_center, bg_rose_100>
            )
        ]
    )
}

pub fn init_battle_screen(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let bg_image = asset_server.load::<Image>("textures/background.png");

    let root = div(cn![flex, flex_col, justify_end, w_full, h_full, relative]);
    commands.spawn(root).with_children(el![
        background(&bg_image),
        el!(
            div::<flex, flex_1, items_center, justify_center>,
            [el!(EnemyArea, div::<flex, gap_4>)]
        ),
        footer(el![
            menu_button(MenuAction::Attack),
            menu_button(MenuAction::Defend)
        ])
    ]);
}

fn init_battle(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    loaded_battle: Res<LoadBattle>,
    enemy_area_query: Query<Entity, With<EnemyArea>>,
    ally_area_query: Query<Entity, With<AllyArea>>,
    char_data: Res<Assets<CharacterData>>,
) {
    let mut battle = Battle {
        ally_layout: Vec::new(),
        enemy_layout: Vec::new(),
        order: Vec::new(),
        turn: 0,
    };

    let layout = enemy_area_query.single();
    for enemy in loaded_battle.enemy_layout.iter() {
        let CharacterData {
            name,
            hitpoints,
            damage,
            image,
            ..
        } = char_data
            .get(enemy)
            .expect("Enemy was just loaded, but not found");

        let image = asset_server.load(image);

        let entity = commands
            .spawn((
                Name(name.clone()),
                Hitpoints(*hitpoints),
                Damage(*damage),
                Enemy,
            ))
            .id();
        battle.enemy_layout.push(entity.clone());

        commands.entity(layout).with_children(el!(
            Attached(entity),
            div::<flex, flex_col>,
            [
                el!(text::<text_black, text_2xl>(name)),
                el!(text::<text_black, text_2xl>(format!("HP: {}", hitpoints))),
                el!(img::<w_32, h_32>(image))
            ]
        ));
    }

    let layout = ally_area_query.single();
    for enemy in loaded_battle.ally_layout.iter() {
        let CharacterData {
            name,
            hitpoints,
            damage,
            portrait,
            ..
        } = char_data
            .get(enemy)
            .expect("Ally was just loaded, but not found");

        let entity = commands
            .spawn((
                Name(name.clone()),
                Hitpoints(*hitpoints),
                Damage(*damage),
                Enemy,
            ))
            .id();

        let image = asset_server.load(portrait);
        battle.ally_layout.push(entity.clone());

        commands
            .entity(layout)
            .with_children(el!(Attached(entity), div::<flex>, [frame(&image)]));
    }
    println!("Battle: {:?}", battle);
    commands.insert_resource(battle);

    // Clean up the load battle scene
    // So there's an error in case we load it twice
    commands.remove_resource::<LoadBattle>();
}

fn change_to_battle(In(finished): In<bool>, mut next_state: ResMut<NextState<AppState>>) {
    if finished {
        next_state.set(AppState::Battle);
    }
}

fn on_menu_action_click(query: Query<&MenuAction>, mut on_click: EventReader<OnClick>) {
    for OnClick(entity) in on_click.read() {
        match query.get(*entity).unwrap() {
            MenuAction::Attack => {
                println!("Attack");
            }
            MenuAction::Items => {
                println!("Items");
            }
            MenuAction::Defend => {
                println!("Defend");
            }
        }
    }
}

pub struct LoadBattlePlugin;

impl Plugin for LoadBattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TomlAssetPlugin::<CharacterData>::new(&["char.toml"]))
            .add_systems(
                Update,
                (wait_for_assets.pipe(change_to_battle)).run_if(in_state(AppState::LoadBattle)),
            )
            .add_systems(
                OnEnter(AppState::Battle),
                (init_battle_screen, init_battle).chain(),
            );
    }
}

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnAttack>()
            .add_plugins(LoadBattlePlugin)
            .add_systems(
                Update,
                on_menu_action_click.run_if(in_state(AppState::Battle)),
            );
    }
}
