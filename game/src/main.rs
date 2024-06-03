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

use el::*;
use helpers::LAYER_INTERACTIVE;
use materials::OutlineMaterial;
use stylesheet::*;

mod components;
mod helpers;
mod materials;
mod overworld;

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

#[derive(Resource, Default)]
struct ActiveElement(Option<Entity>);

#[derive(Resource, Default)]
struct OverElement(Option<Entity>);

#[derive(Event)]
struct OnClick(Entity);

#[derive(Event)]
struct OnFocus(Entity);

#[derive(Event)]
struct OnBlur(Entity);

#[derive(Event)]
struct OnMouseEnter(Entity);

#[derive(Event)]
struct OnMouseLeave(Entity);

#[derive(Component)]
struct Focus;

#[derive(Component)]
struct Disabled;

#[derive(Component)]
struct Hover;

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
    asset_server: ResMut<AssetServer>,
) {
    let rect: Mesh2dHandle = meshes.add(Rectangle::new(32., 32.)).into();
    let circle: Mesh2dHandle = meshes.add(Circle::new(16.)).into();
    let red = materials.add(Color::rgb(1.0, 0.0, 0.0));
    let green = materials.add(Color::rgb(0.0, 1.0, 0.0));
    let blue = materials.add(Color::rgb(0.0, 0.0, 1.0));

    let p_image = asset_server.load::<Image>("textures/portrait.png");
    let bg_image = asset_server.load::<Image>("textures/background.png");

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

    let root = (Id(0), div(cn![flex, items_end, w_full, h_full, relative]));
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

fn on_click(
    query: Query<(Entity, &Interaction), Without<Disabled>>,
    mouse: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut active_element: ResMut<ActiveElement>,
    mut on_click: EventWriter<OnClick>,
) {
    let just_pressed =
        keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::Enter);

    let clicked = query
        .iter()
        .find_map(|(entity, interaction)| {
            match (interaction, mouse.just_pressed(MouseButton::Left)) {
                (Interaction::Pressed, true) => Some(entity),
                _ => None,
            }
        })
        .or(if just_pressed { active_element.0 } else { None });

    if let Some(next) = clicked {
        on_click.send(OnClick(next));

        if active_element.0 != Some(next) {
            active_element.0 = Some(next);
        }
    }
}

fn on_hover(
    query: Query<(Entity, &Interaction), Without<Disabled>>,
    mut over_element: ResMut<OverElement>,
) {
    let hovered = query
        .iter()
        .find_map(|(entity, interaction)| match interaction {
            Interaction::Hovered => Some(entity),
            _ => None,
        });

    if let Some(next) = hovered {
        if over_element.0 != Some(next) {
            over_element.0 = Some(next);
        }
    }
}

fn on_mousehover_and_mouseout(
    mut commands: Commands,
    mut local: Local<OverElement>,
    focus: Res<OverElement>,
    mut mouse_enter_events: EventWriter<OnMouseEnter>,
    mut mouse_leave_events: EventWriter<OnMouseLeave>,
) {
    if local.0 != focus.0 {
        if let Some(prev) = local.0 {
            commands.entity(prev).remove::<Focus>();
            mouse_leave_events.send(OnMouseLeave(prev));
        }

        if let Some(next) = focus.0 {
            commands.entity(next).insert(Focus);
            mouse_enter_events.send(OnMouseEnter(next));
        }

        local.0 = focus.0;
    }
}

fn on_focus_and_blur(
    mut commands: Commands,
    mut local: Local<ActiveElement>,
    focus: Res<ActiveElement>,
    mut focus_events: EventWriter<OnFocus>,
    mut blur_events: EventWriter<OnBlur>,
) {
    if local.0 != focus.0 {
        if let Some(prev) = local.0 {
            commands.entity(prev).remove::<Focus>();
            blur_events.send(OnBlur(prev));
        }

        if let Some(next) = focus.0 {
            commands.entity(next).insert(Focus);
            focus_events.send(OnFocus(next));
        }

        local.0 = focus.0;
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

fn main() {
    App::new()
        .add_plugins(base_plugins())
        .add_plugins(Material2dPlugin::<OutlineMaterial>::default())
        .insert_resource(ActiveElement::default())
        .add_event::<OnClick>()
        .add_event::<OnFocus>()
        .add_event::<OnBlur>()
        .add_event::<OnMouseEnter>()
        .add_event::<OnMouseLeave>()
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
                on_click,
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
        .add_systems(
            Update,
            (input_read, on_menu_action_click, player_move).chain(),
        )
        .run();
}
