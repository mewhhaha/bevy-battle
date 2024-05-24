use crate::components::*;
use crate::helpers::{create_shape, LAYER_WORLD};
use crate::overworld::*;

use bevy::app::PluginGroupBuilder;
use bevy::render::view::RenderLayers;
use bevy::sprite::{Material2dPlugin, Mesh2dHandle};
use bevy::{
    app::{App, Startup, Update},
    asset::{AssetMode, AssetPlugin},
    prelude::*,
};
use helpers::LAYER_INTERACTIVE;
use materials::OutlineMaterial;

use styles::*;

mod components;
mod helpers;
mod materials;
mod overworld;

#[macro_use]
mod styles;

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

fn startup_add_people(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: ResMut<AssetServer>,
) {
    let rect: Mesh2dHandle = meshes.add(Rectangle::new(32., 32.)).into();
    let circle: Mesh2dHandle = meshes.add(Circle::new(16.)).into();
    let red = materials.add(Color::rgb(1.0, 0.0, 0.0));
    let green = materials.add(Color::rgb(0.0, 1.0, 0.0));
    let blue = materials.add(Color::rgb(0.0, 0.0, 1.0));

    let image = asset_server.load::<Image>("textures/spritesheet.png");

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

    commands
        .spawn((Id(0), div(cn![flex, bg_white, w_full, h_full])))
        .with_children(|parent| {
            parent.spawn(img(cn![w_full, h_full], image));
        });
}

fn player_set_closest_interactive(
    query: Query<(&Transform, &RangeInteraction), With<Player>>,
    mut interactive_items: Query<(Entity, &Transform, &mut Interactive)>,
) {
    let (player_transform, &RangeInteraction(range)) = query.single();

    for (_, _, mut interactive) in interactive_items.iter_mut() {
        interactive.0 = false;
    }

    let closest_entity = interactive_items
        .iter()
        .map(|(entity, transform, _)| {
            (
                entity,
                player_transform.translation.distance(transform.translation),
            )
        })
        .filter(|(_, distance)| *distance < range)
        .min_by(|(_, fst), (_, snd)| fst.total_cmp(snd))
        .map(|(entity, _)| entity);

    if let Some((_, _, mut interactive)) =
        closest_entity.and_then(|e| interactive_items.get_mut(e).ok())
    {
        interactive.0 = true;
    }
}

fn set_interactive_render_layer(
    mut commands: Commands,
    player_query: Query<(Entity, &RenderLayers), With<Player>>,
    query: Query<(Entity, &RenderLayers, &Interactive)>,
) {
    for (entity, render_layers, Interactive(interactive)) in query.iter() {
        let updated_layers = if *interactive {
            render_layers.with(LAYER_INTERACTIVE)
        } else {
            render_layers.without(LAYER_INTERACTIVE)
        };
        commands.entity(entity).insert(updated_layers);
    }

    let (player_entity, player_layers) = player_query.single();
    let interacting = query
        .iter()
        .any(|(_, _, Interactive(interactive))| *interactive);

    let mut player_commands = commands.entity(player_entity);

    if interacting {
        player_commands.insert(player_layers.with(LAYER_INTERACTIVE));
    } else {
        player_commands.insert(player_layers.without(LAYER_INTERACTIVE));
    }
}

fn base_plugins() -> PluginGroupBuilder {
    DefaultPlugins
        .set(AssetPlugin {
            mode: AssetMode::Processed,
            ..default()
        })
        .set(ImagePlugin::default_nearest())
}

fn main() {
    App::new()
        .add_plugins(base_plugins())
        .add_plugins(Material2dPlugin::<OutlineMaterial>::default())
        .insert_state(AppState::Overworld)
        .insert_state(RunningState::Running)
        .add_systems(
            Startup,
            (startup_add_people, startup_add_cameras, startup_add_keymap),
        )
        .add_systems(
            PreUpdate,
            (
                sort_y,
                player_set_closest_interactive,
                set_interactive_render_layer,
                animation_change_frame,
            ),
        )
        .add_systems(
            PreUpdate,
            (player_set_closest_interactive, set_interactive_render_layer)
                .run_if(in_state(AppState::Overworld)),
        )
        .add_systems(Update, (input_read, player_move).chain())
        .run();
}
