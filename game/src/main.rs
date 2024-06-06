use crate::components::*;
use crate::helpers::{create_shape, LAYER_WORLD};

use bevy::app::PluginGroupBuilder;
use bevy::asset::{AssetLoader, LoadContext, LoadedAsset};
use bevy::render::view::RenderLayers;
use bevy::sprite::{Material2dPlugin, Mesh2dHandle};
use bevy::utils::BoxedFuture;
use bevy::{
    app::{App, Startup, Update},
    asset::{AssetMode, AssetPlugin},
    prelude::*,
};

use helpers::{AppState, AssetsLoading, OnPlayerInteract, PlayerInputPlugin, RunningState};
use materials::OutlineMaterial;
use stylesheet::*;

mod components;
mod helpers;
mod materials;
mod scenes;
mod ui_events;

use scenes::battle::*;
use scenes::overworld::*;
use ui_events::{OnClick, UiEventsPlugin};

#[derive(Component, Clone, Debug)]
enum MenuAction {
    Attack,
    Items,
    Defend,
}

fn startup_add_people(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let rect: Mesh2dHandle = meshes.add(Rectangle::new(32., 32.)).into();
    let circle: Mesh2dHandle = meshes.add(Circle::new(16.)).into();
    let red = materials.add(Color::rgb(1.0, 0.0, 0.0));
    let green = materials.add(Color::rgb(0.0, 1.0, 0.0));
    let blue = materials.add(Color::rgb(0.0, 0.0, 1.0));

    commands.spawn((
        Player,
        SortY,
        create_shape(rect.clone(), green.clone(), Vec3::new(1., 1., 0.)),
        Speed(200.0),
        Velocity::default(),
        RangeInteraction(32.0),
        RenderLayers::layer(LAYER_WORLD),
    ));

    commands.spawn((
        VendingMachine,
        SortY,
        create_shape(circle.clone(), red.clone(), Vec3::new(0., 0., 0.)),
        Interactive(false),
        RenderLayers::layer(LAYER_WORLD),
    ));
    commands.spawn((
        VendingMachine,
        SortY,
        create_shape(circle.clone(), red.clone(), Vec3::new(-48., 100., 0.)),
        Interactive(false),
        RenderLayers::layer(LAYER_WORLD),
    ));
    commands.spawn((
        SortY,
        create_shape(rect.clone(), blue.clone(), Vec3::new(32., 32., 0.)),
        RenderLayers::layer(LAYER_WORLD),
    ));
}

fn base_plugins() -> PluginGroupBuilder {
    DefaultPlugins
        .set(AssetPlugin {
            mode: AssetMode::Processed,
            ..default()
        })
        .set(ImagePlugin::default_nearest())
}

const THIEF_ENEMY: &str = "characters/thief.char.toml";
const PLAYER_ALLY: &str = "characters/player.char.toml";

fn change_state_when_interact_enemy(
    mut commands: Commands,
    query: Query<(&Interactive, &VendingMachine)>,
    mut next_state: ResMut<NextState<AppState>>,
    mut on_player_interact: EventReader<OnPlayerInteract>,

    asset_server: ResMut<AssetServer>,
    mut assets_loading: ResMut<AssetsLoading>,
) {
    let any_interactable = query.iter().len() > 0;
    let any_interaction = on_player_interact.read().len() > 0;

    if any_interactable && any_interaction {
        let thief_handle = asset_server.load::<CharacterData>(THIEF_ENEMY);
        let player_handle = asset_server.load::<CharacterData>(PLAYER_ALLY);

        let battle = LoadBattle {
            enemy_layout: vec![
                thief_handle.clone(),
                thief_handle.clone(),
                thief_handle.clone(),
            ],
            ally_layout: vec![player_handle.clone()],
        };
        commands.insert_resource(battle);
        assets_loading.add_all(vec![thief_handle.untyped(), player_handle.untyped()]);
        next_state.set(AppState::LoadBattle);
    }
}

fn main() {
    App::new()
        .add_plugins(base_plugins())
        .add_plugins(Material2dPlugin::<OutlineMaterial>::default())
        .add_plugins(InteractionOutlinePlugin)
        .add_plugins(UiEventsPlugin)
        .add_plugins(PlayerInputPlugin)
        .add_plugins(BattlePlugin)
        .insert_resource(AssetsLoading::default())
        .insert_state(AppState::Overworld)
        .insert_state(RunningState::Running)
        .add_systems(Startup, (startup_add_people, startup_add_cameras))
        .add_systems(PreUpdate, (sort_y, animation_change_frame))
        .add_systems(
            Update,
            (player_move, change_state_when_interact_enemy).run_if(in_state(AppState::Overworld)),
        )
        .run();
}
