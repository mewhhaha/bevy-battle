use crate::components::*;
use crate::systems::*;

use bevy::render::view::RenderLayers;
use bevy::sprite::Material2dPlugin;
use bevy::text;
use bevy::{
    app::{App, Startup, Update},
    asset::{AssetMode, AssetPlugin},
    prelude::*,
};
use materials::OutlineMaterial;
mod components;
mod helpers;
mod materials;
mod systems;

fn startup_add_vending_machine(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load::<Image>("spritesheet.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(32.0, 32.0), 8, 7, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    fn spawn_machine(
        commands: &mut Commands,
        offset: Vec2,
        texture: Handle<Image>,
        texture_atlas_layout: Handle<TextureAtlasLayout>,
    ) {
        let sprite = SpriteSheetBundle {
            texture,
            transform: Transform {
                translation: offset.extend(0.0),
                ..default()
            },
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
            ..default()
        };

        let idle_frames = helpers::create_frames(20, 20, 0.2);

        let animated_sprite = (
            Animation {
                name: "idle",
                time: 0.0,
                current: 0,
                frames: idle_frames,
            },
            sprite,
        );

        commands.spawn((VendingMachine, animated_sprite, Interactive(false)));
    }

    spawn_machine(
        &mut commands,
        Vec2::new(0.0, 0.0),
        texture.clone(),
        texture_atlas_layout.clone(),
    );
    spawn_machine(
        &mut commands,
        Vec2::new(16.0, 16.0),
        texture,
        texture_atlas_layout,
    );
}

fn player_set_closest_interactive(
    query: Query<(&Transform, &RangeInteraction), With<Player>>,
    mut interactive_items: Query<(Entity, &Transform, &mut Interactive)>,
) {
    let (player_transform, &RangeInteraction(range)) = query.single();

    let mut closest_item: Option<Entity> = None;
    let mut closest_distance = range;

    for (entity, transform, mut interactive) in interactive_items.iter_mut() {
        // Reset interactivity
        interactive.0 = false;
        let distance = player_transform.translation.distance(transform.translation);
        if distance < closest_distance {
            closest_item = Some(entity);
            closest_distance = distance;
        }
    }

    let interactive_item = closest_item.and_then(|entity| interactive_items.get_mut(entity).ok());

    match interactive_item {
        Some((_, _, mut interactive)) => {
            interactive.0 = true;
        }
        _ => {}
    }
}

fn set_interactive_render_layer(
    mut commands: Commands,
    player_query: Query<(Entity, &RenderLayers), With<Player>>,
    query: Query<(Entity, Option<&RenderLayers>, &Interactive), Changed<Interactive>>,
) {
    let (player_entity, player_layers) = player_query.single();

    let mut any_interactive = false;
    for (entity, render_layers, Interactive(interactive)) in query.iter() {
        let layers = render_layers.map(|layers| *layers).unwrap_or_default();

        println!("Setting interactive: {}", interactive,);
        let mut interactive_commands = commands.entity(entity);
        if *interactive {
            any_interactive = true;
            interactive_commands.insert(layers.with(2));
        } else {
            interactive_commands.insert(layers.without(2));
        }
    }

    let mut player_commands = commands.entity(player_entity);
    if any_interactive {
        player_commands.insert(player_layers.with(2));
    } else {
        player_commands.insert(player_layers.without(2));
    }
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    mode: AssetMode::Processed,
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(Material2dPlugin::<OutlineMaterial>::default())
        .add_systems(
            Startup,
            (
                startup_add_player,
                startup_add_vending_machine,
                startup_add_camera,
                startup_add_keymap,
            ),
        )
        .add_systems(
            PreUpdate,
            (
                sort_y,
                player_set_closest_interactive,
                set_interactive_render_layer,
                animation_change_frame,
                sprite_face_velocity,
            ),
        )
        .add_systems(Update, (input_read, player_move).chain())
        .run();
}
