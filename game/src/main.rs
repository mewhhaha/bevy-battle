use crate::components::*;
use crate::helpers::{create_shape, LAYER_WORLD};

use bevy::app::PluginGroupBuilder;
use bevy::render::view::RenderLayers;
use bevy::sprite::{Material2dPlugin, Mesh2dHandle};
use bevy::{
    app::{App, Startup, Update},
    asset::{AssetMode, AssetPlugin},
    prelude::*,
};

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

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    MainMenu,
    Overworld,
    Fighting,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum RunningState {
    Running,
    Paused,
}

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

fn change_state_when_interact_enemy(
    query: Query<(&Interactive, &VendingMachine)>,
    mut state: ResMut<NextState<AppState>>,
    mut on_player_interact: EventReader<OnPlayerInteract>,
) {
    for _ in on_player_interact.read() {
        if (query.iter().len() > 0) && (state.0 == Some(AppState::Overworld)) {
            state.0 = Some(AppState::Fighting);
        }
    }
}

fn main() {
    App::new()
        .add_plugins(base_plugins())
        .add_plugins(Material2dPlugin::<OutlineMaterial>::default())
        .add_plugins(InteractionOutlinePlugin)
        .add_plugins(UiEventsPlugin)
        .add_plugins(PlayerInputPlugin)
        .insert_state(AppState::Overworld)
        .insert_state(RunningState::Running)
        .add_systems(OnEnter(AppState::Fighting), spawn_battle_screen)
        .add_systems(Startup, (startup_add_people, startup_add_cameras))
        .add_systems(PreUpdate, (sort_y, animation_change_frame))
        .add_systems(
            Update,
            (
                on_menu_action_click,
                player_move,
                change_state_when_interact_enemy,
            ),
        )
        .run();
}
